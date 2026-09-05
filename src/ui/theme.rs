use iced::widget::container;
use iced::{color, Background, Border, Color, Theme};

pub const SCREEN_PADDING: f32 = 32.0;

pub const TILE_BORDER_WIDTH_FOCUSED: f32 = 4.0;
pub const TILE_BORDER_WIDTH_UNFOCUSED: f32 = 1.0;
pub const TILE_CORNER_RADIUS: f32 = 8.0;

pub fn background() -> Color {
    color!(0x121212)
}

pub fn text() -> Color {
    color!(0xe5e5e5)
}

pub fn tile_fill(focused: bool) -> Color {
    if focused {
        color!(0x2a2a2a)
    } else {
        color!(0x1e1e1e)
    }
}

pub fn tile_border(focused: bool) -> Color {
    if focused {
        color!(0xf5a623)
    } else {
        color!(0x3a3a3a)
    }
}

pub fn root_container_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(background())),
        text_color: Some(text()),
        ..container::Style::default()
    }
}

pub fn tile_container_style(focused: bool) -> impl Fn(&Theme) -> container::Style {
    move |_theme| container::Style {
        background: Some(Background::Color(tile_fill(focused))),
        border: Border {
            color: tile_border(focused),
            width: if focused {
                TILE_BORDER_WIDTH_FOCUSED
            } else {
                TILE_BORDER_WIDTH_UNFOCUSED
            },
            radius: TILE_CORNER_RADIUS.into(),
        },
        text_color: Some(text()),
        ..container::Style::default()
    }
}
