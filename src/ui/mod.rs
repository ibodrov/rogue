pub mod sfml_ui;

pub enum UIKey {
    Unknown,
    Up,
    Down,
    Left,
    Right,
    Space,
}

pub enum UIEvent {
    Unknown,
    Closed,
    KeyPressed {
        code: UIKey,
    }
}

pub trait UI {
    fn is_open(&self) -> bool;

    fn poll_event(&mut self) -> Option<UIEvent>;

    fn display(&mut self);
}
