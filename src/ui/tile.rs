use std::fmt;

use iced::widget::{column, container, svg, text, Space};
use iced::{Alignment, Element};

use crate::config::AppEntry;
use crate::ui::theme;

pub const WIDTH: f32 = 280.0;
pub const HEIGHT: f32 = 180.0;
pub const ICON_SIZE: f32 = 64.0;
pub const CONTENT_SPACING: f32 = 12.0;
pub const NAME_TEXT_SIZE: f32 = 18.0;
pub const STATE_TEXT_SIZE: f32 = 14.0;

const ICONS_DIR: &str = "assets/icons";

pub struct TileView {
    pub focused: bool,
    pub tile_state: TileState,
}

pub enum TileState {
    Idle,
    Launching(char),
    Running,
}

impl fmt::Display for TileState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TileState::Launching(frame) => write!(f, "{frame}"),
            TileState::Running => write!(f, "Running"),
            TileState::Idle => write!(f, ""),
        }
    }
}

pub fn view<'a, Message: 'a>(app: &AppEntry, tile_view: TileView) -> Element<'a, Message> {
    let icon: Element<'_, Message> = match &app.icon {
        Some(filename) => svg(svg::Handle::from_path(format!("{ICONS_DIR}/{filename}")))
            .width(ICON_SIZE)
            .height(ICON_SIZE)
            .into(),
        None => Space::new().width(ICON_SIZE).height(ICON_SIZE).into(),
    };

    let content = column![
        icon,
        text(app.name.clone()).size(NAME_TEXT_SIZE),
        text(tile_view.tile_state.to_string()).size(STATE_TEXT_SIZE),
    ]
    .spacing(CONTENT_SPACING)
    .align_x(Alignment::Center);

    container(content)
        .width(WIDTH)
        .height(HEIGHT)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(theme::tile_container_style(tile_view.focused))
        .into()
}
