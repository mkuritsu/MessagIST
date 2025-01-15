pub trait EventHandler<E> {
    fn handle_event(&mut self, event: E);
}

pub trait StatefulEventHandler<E> {
    type State;

    fn handle_event(&mut self, event: E, state: &mut Self::State);
}

pub trait AsyncStatefulEventHandler<E> {
    type State;

    async fn handle_event(&mut self, event: E, state: &mut Self::State);
}
