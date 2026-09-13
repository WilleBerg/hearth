use std::sync::Arc;

use iced::keyboard;
use iced::widget::{column, container, row, Space};
use iced::{window, Element, Length, Subscription, Task};
use log::error;

use crate::config::browsers::Browsers;
use crate::config::{AppAction, AppEntry, Config};
use crate::platform::{launch_command, launch_url, LaunchError};
use crate::ui::{carousel, theme};

pub struct Hub {
    apps: Vec<AppEntry>,
    browsers: Browsers,
    focused: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    Quit,
    Navigate(carousel::Direction),
    Select,
    LaunchFinished(Result<(), Arc<LaunchError>>),
}

impl Hub {
    pub fn new() -> (Self, Task<Message>) {
        let config = Config::load_default(None);
        let browsers = Browsers::load_default(None);
        (
            Self {
                apps: config.apps,
                browsers,
                focused: 0,
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Quit => window::latest().and_then(window::close),
            Message::Navigate(direction) => {
                self.focused = carousel::next_index(self.focused, direction, self.apps.len());
                carousel::scroll_to_focused(self.focused, self.apps.len())
            }
            Message::Select => {
                if let Some(app) = self.apps.get(self.focused) {
                    match &app.action {
                        AppAction::Url { url, browser } => {
                            let browsers = self.browsers.clone();
                            let url = url.clone();
                            let browser = browser.clone();
                            return Task::perform(
                                async move {
                                    launch_url(&url, &browser, &browsers)
                                        .await
                                        .map_err(Arc::new)
                                },
                                Message::LaunchFinished,
                            );
                        }
                        AppAction::Command { command, args } => {
                            let command = command.clone();
                            let args = args.clone();
                            return Task::perform(
                                async move { launch_command(&command, args).await.map_err(Arc::new) },
                                Message::LaunchFinished,
                            );
                        }
                    }
                }
                // TODO: Update top bar. Will need to create top bar first.
                Task::none()
            }
            Message::LaunchFinished(Err(err)) => {
                error!("Failed to launch: {err}");
                Task::none()
            }
            Message::LaunchFinished(Ok(())) => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let carousel_column = column![
            Space::new().height(Length::Fill),
            carousel::view(&self.apps, self.focused),
        ]
        .width(Length::FillPortion(theme::CAROUSEL_WIDTH_PORTION))
        .height(Length::Fill);

        // Reserved for the future "always there" widgets column.
        let reserved = Space::new()
            .width(Length::FillPortion(theme::RESERVED_WIDTH_PORTION))
            .height(Length::Fill);

        let content = row![carousel_column, reserved]
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(theme::SCREEN_PADDING);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(theme::root_container_style)
            .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        use keyboard::key;

        keyboard::listen().filter_map(|event| match event {
            keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(named),
                ..
            } => match named {
                key::Named::Escape => Some(Message::Quit),
                key::Named::ArrowLeft => Some(Message::Navigate(carousel::Direction::Left)),
                key::Named::ArrowRight => Some(Message::Navigate(carousel::Direction::Right)),
                key::Named::Enter => Some(Message::Select),
                _ => None,
            },
            _ => None,
        })
    }
}
