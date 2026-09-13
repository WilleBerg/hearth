use iced::widget::{operation, row, scrollable};
use iced::{Element, Task};

use crate::app::LaunchState;
use crate::config::AppEntry;
use crate::ui::tile::{self, TileState, TileView};

pub const SPACING: f32 = 24.0;
const ID: &str = "carousel";

const LOADING_SPINNER_FRAMES: [char; 10] = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

pub struct Carousel {
    apps: Vec<AppEntry>,
    focused: usize,
}

impl Carousel {
    pub fn new(apps: Vec<AppEntry>) -> Self {
        Self { apps, focused: 0 }
    }

    pub fn focused_app(&self) -> Option<&AppEntry> {
        self.apps.get(self.focused)
    }

    pub fn focused_index(&self) -> usize {
        self.focused
    }

    pub fn navigate<Message: 'static>(&mut self, direction: Direction) -> Task<Message> {
        self.focused = next_index(self.focused, direction, self.apps.len());
        scroll_to_focused(self.focused, self.apps.len())
    }

    pub fn view<'a, Message: 'a>(
        &'a self,
        launch_state: &LaunchState,
        frame: usize,
    ) -> Element<'a, Message> {
        let launch_state_index = launch_state.get_index();
        let tiles = self.apps.iter().enumerate().map(|(index, app)| {
            let tile_state = if launch_state_index == Some(index) {
                match launch_state {
                    LaunchState::Launching(_) => {
                        let glyph = LOADING_SPINNER_FRAMES[frame % LOADING_SPINNER_FRAMES.len()];
                        TileState::Launching(glyph)
                    }
                    LaunchState::Running(_) => TileState::Running,
                    _ => TileState::Idle,
                }
            } else {
                TileState::Idle
            };
            let tile_view = TileView {
                focused: index == self.focused,
                tile_state,
            };
            tile::view(app, tile_view)
        });

        let content = row(tiles).spacing(SPACING);

        scrollable(content)
            .id(ID)
            .direction(scrollable::Direction::Horizontal(
                scrollable::Scrollbar::hidden(),
            ))
            .into()
    }
}

/// Scrolls only when focus reaches the first or last tile, snapping fully to
/// that edge. Every other step leaves the scroll position untouched, so
/// browsing tiles that are already visible doesn't shift the view at all.
fn scroll_to_focused<Message: 'static>(focused: usize, total: usize) -> Task<Message> {
    if focused == 0 {
        operation::snap_to(ID, operation::RelativeOffset::START)
    } else if total > 0 && focused == total - 1 {
        operation::snap_to(ID, operation::RelativeOffset::END)
    } else {
        Task::none()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

/// Moves focus one step in `direction`, wrapping around at either end of
/// `total` tiles.
fn next_index(current: usize, direction: Direction, total: usize) -> usize {
    match direction {
        Direction::Left => (current + total - 1) % total,
        Direction::Right => (current + 1) % total,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_advances_by_one() {
        assert_eq!(next_index(2, Direction::Right, 7), 3);
    }

    #[test]
    fn left_retreats_by_one() {
        assert_eq!(next_index(2, Direction::Left, 7), 1);
    }

    #[test]
    fn right_wraps_from_last_to_first() {
        assert_eq!(next_index(6, Direction::Right, 7), 0);
    }

    #[test]
    fn left_wraps_from_first_to_last() {
        assert_eq!(next_index(0, Direction::Left, 7), 6);
    }

    #[test]
    fn single_item_stays_put() {
        assert_eq!(next_index(0, Direction::Right, 1), 0);
        assert_eq!(next_index(0, Direction::Left, 1), 0);
    }
}
