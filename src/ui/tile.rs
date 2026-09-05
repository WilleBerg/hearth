use iced::widget::{column, container, svg, text, Space};
use iced::{Alignment, Element};

use crate::config::AppEntry;
use crate::ui::theme;

pub const WIDTH: f32 = 280.0;
pub const HEIGHT: f32 = 180.0;
pub const ICON_SIZE: f32 = 64.0;
pub const CONTENT_SPACING: f32 = 12.0;
pub const NAME_TEXT_SIZE: f32 = 18.0;

const ICONS_DIR: &str = "assets/icons";

pub fn view<'a, Message: 'a>(app: &AppEntry, focused: bool) -> Element<'a, Message> {
    let icon: Element<'_, Message> = match &app.icon {
        Some(filename) => svg(svg::Handle::from_path(format!(
            "{ICONS_DIR}/{filename}"
        )))
        .width(ICON_SIZE)
        .height(ICON_SIZE)
        .into(),
        None => Space::new().width(ICON_SIZE).height(ICON_SIZE).into(),
    };

    let content = column![icon, text(app.name.clone()).size(NAME_TEXT_SIZE)]
        .spacing(CONTENT_SPACING)
        .align_x(Alignment::Center);

    container(content)
        .width(WIDTH)
        .height(HEIGHT)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(theme::tile_container_style(focused))
        .into()
}
