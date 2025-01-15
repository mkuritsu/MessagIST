use ratatui::style::{Color, Style};

use super::widgets::{button::ButtonColors, logger::LoggingColors};

pub const CATPUCCIN_MOCHA: AppTheme = AppTheme {
    accent: Color::Rgb(180, 190, 254),
    highlight: Color::Rgb(137, 180, 250),
    text: Color::Rgb(205, 214, 244),
    subtext: Color::Rgb(166, 173, 200),
    warn: Color::Rgb(249, 226, 175),
    error: Color::Rgb(243, 139, 168),
    success: Color::Rgb(166, 227, 161),
    background: Color::Rgb(30, 30, 46),
    shadow: Color::Rgb(24, 24, 37),
    dark_text: Color::Rgb(17, 17, 27),
};

#[derive(Clone, Copy, Debug)]
pub struct AppTheme {
    pub accent: Color,
    pub highlight: Color,
    pub text: Color,
    pub subtext: Color,
    pub warn: Color,
    pub error: Color,
    pub success: Color,
    pub background: Color,
    pub shadow: Color,
    pub dark_text: Color,
}

impl AppTheme {
    pub const fn text_style(&self) -> Style {
        Style::new().fg(self.text).bg(self.background)
    }

    pub const fn accent_style(&self) -> Style {
        Style::new().fg(self.accent).bg(self.background)
    }

    pub const fn warn_style(&self) -> Style {
        Style::new().fg(self.warn).bg(self.background)
    }

    pub const fn error_style(&self) -> Style {
        Style::new().fg(self.error).bg(self.background)
    }

    pub const fn success_style(&self) -> Style {
        Style::new().fg(self.success).bg(self.background)
    }

    pub const fn background_style(&self) -> Style {
        Style::new().fg(self.text).bg(self.background)
    }

    pub const fn subtext_stye(&self) -> Style {
        Style::new().fg(self.subtext).bg(self.background)
    }

    pub const fn button_colors(&self) -> ButtonColors {
        ButtonColors {
            text: self.dark_text,
            shadow: self.shadow,
            background: self.accent,
            highlight: self.highlight,
        }
    }

    pub const fn negative_button_colors(&self) -> ButtonColors {
        ButtonColors {
            text: self.dark_text,
            shadow: self.shadow,
            background: self.error,
            highlight: self.error,
        }
    }
}

impl Into<LoggingColors> for AppTheme {
    fn into(self) -> LoggingColors {
        LoggingColors {
            trace: Style::new().fg(self.dark_text).bg(self.background),
            debug: Style::new().fg(self.subtext).bg(self.background),
            info: Style::new().fg(self.success).bg(self.background),
            warn: Style::new().fg(self.warn).bg(self.background),
            error: Style::new().fg(self.error).bg(self.background),
        }
    }
}
