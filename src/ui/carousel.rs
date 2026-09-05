use iced::widget::{row, scrollable};
use iced::Element;

use crate::config::AppEntry;
use crate::ui::tile;

pub const SPACING: f32 = 24.0;

pub fn view<'a, Message: 'a>(apps: &'a [AppEntry], focused: usize) -> Element<'a, Message> {
    let tiles = apps
        .iter()
        .enumerate()
        .map(|(index, app)| tile::view(app, index == focused));

    let content = row(tiles).spacing(SPACING);

    scrollable(content)
        .direction(scrollable::Direction::Horizontal(scrollable::Scrollbar::new()))
        .into()
}
