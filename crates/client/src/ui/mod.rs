use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use event_handler::{AsyncStatefulEventHandler, EventHandler};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::Backend,
    style::Style,
    widgets::Block,
    Terminal,
};
use routing::Router;
use tokio::sync::mpsc::UnboundedReceiver;
use widgets::logger::{LoggerWidget, LoggerWidgetState};

use crate::{
    app::{App, AppEvent},
    logger::LoggerRecord,
};

mod event_handler;
mod pages;
mod routing;
pub mod theming;
mod widgets;

pub const RENDER_TICKRATE: Duration = Duration::from_millis(16);

pub async fn run_ui<B: Backend>(
    mut terminal: Terminal<B>,
    app: &mut App,
    mut log_receiver: UnboundedReceiver<LoggerRecord>,
) -> anyhow::Result<()> {
    let mut router = Router::new();
    let mut last_draw = Instant::now();
    let mut logger_state = LoggerWidgetState::default();
    while !app.should_quit {
        app.frame_duration = last_draw.elapsed();
        if last_draw.elapsed() >= RENDER_TICKRATE {
            last_draw = Instant::now();
            router.update_pages(app);
            render(&mut terminal, &mut router, app, &mut logger_state)?;
        }
        process_logs(&mut log_receiver, &mut logger_state);
        process_notifications(app).await?;
        if event::poll(RENDER_TICKRATE.saturating_sub(last_draw.elapsed()))? {
            process_input(&mut router, app, &mut logger_state).await?;
        }
    }
    Ok(())
}

fn render<B: Backend>(
    terminal: &mut Terminal<B>,
    router: &mut Router,
    app: &mut App,
    logger_state: &mut LoggerWidgetState,
) -> anyhow::Result<()> {
    terminal.draw(|frame| {
        let background = Block::new().style(Style::default().bg(app.theme.background));
        frame.render_widget(background, frame.area());
        if app.show_terminal {
            let [ui_area, term_area] =
                Layout::vertical([Constraint::Percentage(70), Constraint::Percentage(30)])
                    .areas(frame.area());
            let buf = frame.buffer_mut();
            router.render(ui_area, buf, app);
            let logs = LoggerWidget::new().colors(app.theme.into());
            frame.render_stateful_widget(logs, term_area, logger_state);
        } else {
            let area = frame.area();
            let buf = frame.buffer_mut();
            router.render(area, buf, app);
        }
    })?;
    Ok(())
}

fn process_logs(log_receiver: &mut UnboundedReceiver<LoggerRecord>, state: &mut LoggerWidgetState) {
    if let Ok(record) = log_receiver.try_recv() {
        state.handle_event(AppEvent::Log(record));
    }
}

async fn process_notifications(app: &mut App) -> anyhow::Result<()> {
    if let Some(receiver) = &mut app.notification_receiver {
        if let Ok(message) = receiver.try_recv() {
            log::info!("Received notification: {:?}", message);
            let counter = message.sent_counter;
            let last_counter = match app.messages.get(&message.sender_istid) {
                Some(messages) => messages
                    .iter()
                    .map(|m| m.sent_counter)
                    .max()
                    .unwrap_or_default(),
                None => 0,
            };
            if counter != last_counter + 1 {
                log::warn!(
                    "Received message with wrong counter, expected: {}, got: {}",
                    last_counter + 1,
                    counter
                );
            }
            app.add_message(message.sender_istid.clone(), message.clone())
                .await?;
        }
    }
    Ok(())
}

async fn process_input(
    router: &mut Router,
    app: &mut App,
    logger_state: &mut LoggerWidgetState,
) -> anyhow::Result<()> {
    let event = event::read()?;
    if let Event::Key(key) = event {
        if key.kind == KeyEventKind::Release {
            return Ok(());
        }
        if key.code == KeyCode::F(10) {
            app.show_terminal = !app.show_terminal;
            return Ok(());
        }
    }
    if let Event::Mouse(mouse_event) = event {
        let pos = (mouse_event.column, mouse_event.row);
        if app.show_terminal && logger_state.area.contains(pos.into()) {
            logger_state.handle_event(AppEvent::Input(event.clone()));
        }
    }
    router.handle_event(AppEvent::Input(event), app).await;
    Ok(())
}
