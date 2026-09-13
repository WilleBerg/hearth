use std::fmt;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

use iced::keyboard;
use iced::widget::{column, container, row, Space};
use iced::{window, Element, Length, Subscription, Task};
use log::{debug, error};

use crate::config::browsers::Browsers;
use crate::config::{AppAction, Config};
use crate::platform::{launch_command, launch_url, resolve_browser, LaunchError};
use crate::ui::carousel::{self, Carousel};
use crate::ui::theme;

const ANIMATION_UPDATE_INTERVAL: Duration = Duration::from_millis(80);
const TIME_IN_LAUNCHING: Duration = Duration::from_secs(5);

pub struct Hub {
    carousel: Carousel,
    browsers: Browsers,
    launch_state: LaunchState,
    animation_frame: usize,
}

#[derive(Debug, Clone)]
pub enum LaunchState {
    Idle,
    Launching(usize),
    Running(usize),
    Error(String),
}

impl LaunchState {
    fn is_launching(&self) -> bool {
        matches!(self, LaunchState::Launching(_))
    }
    fn is_idle(&self) -> bool {
        matches!(self, LaunchState::Idle)
    }
    pub fn get_index(&self) -> Option<usize> {
        match *self {
            LaunchState::Launching(index) => Some(index),
            LaunchState::Running(index) => Some(index),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Quit,
    Navigate(carousel::Direction),
    Select,
    LaunchFinished(Result<(), Arc<LaunchError>>),
    UpdateAnimation,
    TransitionToRunning,
}

impl Hub {
    pub fn new() -> (Self, Task<Message>) {
        let config = Config::load_default(None);
        let browsers = Browsers::load_default(None);
        (
            Self {
                carousel: Carousel::new(config.apps),
                browsers,
                launch_state: LaunchState::Idle,
                animation_frame: 0,
            },
            Task::none(),
        )
    }

    fn set_idle_launch_state(&mut self) {
        self.launch_state = LaunchState::Idle
    }
    fn set_launching_launch_state(&mut self) {
        self.launch_state = LaunchState::Launching(self.carousel.focused_index())
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Quit => window::latest().and_then(window::close),
            Message::Navigate(direction) => self.carousel.navigate(direction),
            Message::Select => {
                if self.launch_state.is_launching() {
                    return Task::none();
                }
                if let Some(app) = self.carousel.focused_app() {
                    let action = app.action.clone();
                    self.set_launching_launch_state();
                    match action {
                        AppAction::Url { url, browser } => {
                            let browser = match resolve_browser(&browser, &self.browsers) {
                                Err(launch_error) => {
                                    return Task::done(Message::LaunchFinished(Err(Arc::new(
                                        launch_error,
                                    ))))
                                }
                                Ok(browser_entry) => browser_entry.clone(),
                            };
                            return spawn_launch(async move { launch_url(&url, &browser).await });
                        }
                        AppAction::Command { command, args } => {
                            return spawn_launch(
                                async move { launch_command(&command, args).await },
                            );
                        }
                    }
                }
                // TODO: Update top bar. Will need to create top bar first.
                Task::none()
            }
            Message::LaunchFinished(exit_status) => {
                self.set_idle_launch_state();
                match exit_status {
                    Ok(()) => debug!("Launch exited successfully"),
                    Err(err) => error!("Failed to launch: {err}"),
                }
                Task::none()
            }
            Message::UpdateAnimation => {
                self.animation_frame = self.animation_frame.wrapping_add(1);
                Task::none()
            }
            Message::TransitionToRunning => {
                if let Some(index) = self.launch_state.get_index() {
                    self.launch_state = LaunchState::Running(index);
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let carousel_column = column![
            Space::new().height(Length::Fill),
            self.carousel.view(&self.launch_state, self.animation_frame),
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
        let mut sub_batch = vec![];
        use keyboard::key;
        let kb_event = keyboard::listen().filter_map(|event| match event {
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
        });
        sub_batch.push(kb_event);
        if matches!(self.launch_state, LaunchState::Launching(_)) {
            let animation_ticks =
                iced::time::every(ANIMATION_UPDATE_INTERVAL).map(|_| Message::UpdateAnimation);
            let launching_ttl =
                iced::time::every(TIME_IN_LAUNCHING).map(|_| Message::TransitionToRunning);
            sub_batch.push(animation_ticks);
            sub_batch.push(launching_ttl);
        }
        Subscription::batch(sub_batch)
    }
}

impl fmt::Display for LaunchState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LaunchState::Idle => write!(f, "idle"),
            LaunchState::Launching(index) => write!(f, "launching app {index}"),
            LaunchState::Running(index) => write!(f, "app running {index}"),
            LaunchState::Error(message) => write!(f, "launch failed: {message}"),
        }
    }
}

fn spawn_launch(
    future: impl Future<Output = Result<(), LaunchError>> + Sync + Send + 'static,
) -> Task<Message> {
    Task::perform(
        async { future.await.map_err(Arc::new) },
        Message::LaunchFinished,
    )
}
