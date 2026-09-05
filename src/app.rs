use iced::keyboard;
use iced::widget::{column, text};
use iced::{window, Element, Subscription, Task};

use crate::config::{AppEntry, Config};

const PLACEHOLDER_SIZE: f32 = 24.0;
const PLACEHOLDER_SPACING: f32 = 8.0;

pub struct Hub {
    apps: Vec<AppEntry>,
    focused: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    Quit,
}

impl Hub {
    pub fn new() -> (Self, Task<Message>) {
        let config = Config::load_default(None);
        (
            Self {
                apps: config.apps,
                focused: 0,
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Quit => window::latest().and_then(window::close),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let entries = self.apps.iter().enumerate().map(|(index, app)| {
            let label = if index == self.focused {
                format!("> {}", app.name)
            } else {
                app.name.clone()
            };
            text(label).size(PLACEHOLDER_SIZE).into()
        });

        column(entries)
            .spacing(PLACEHOLDER_SPACING)
            .padding(PLACEHOLDER_SIZE)
            .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        use keyboard::key;

        keyboard::listen().filter_map(|event| match event {
            keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(key::Named::Escape),
                ..
            } => Some(Message::Quit),
            _ => None,
        })
    }
}
