use iced::widget::text;
use iced::{Element, Task};

#[derive(Debug, Default)]
pub struct Hub;

#[derive(Debug, Clone)]
pub enum Message {
    Noop,
}

impl Hub {
    pub fn new() -> (Self, Task<Message>) {
        (Self, Task::none())
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Noop => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        text("Hearth").size(32).into()
    }
}
