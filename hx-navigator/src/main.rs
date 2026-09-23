use std::collections::BTreeMap;
use zellij_tile::prelude::*;

#[derive(Clone, Copy)]
enum FocusDirection {
    Left,
    Right,
    Up,
    Down,
}

impl FocusDirection {
    fn from_message(message: &str) -> Option<Self> {
        match message {
            "focus_left" => Some(Self::Left),
            "focus_right" => Some(Self::Right),
            "focus_up" => Some(Self::Up),
            "focus_down" => Some(Self::Down),
            _ => None,
        }
    }

    fn as_zellij_direction(self) -> Direction {
        match self {
            Self::Left => Direction::Left,
            Self::Right => Direction::Right,
            Self::Up => Direction::Up,
            Self::Down => Direction::Down,
        }
    }

    fn as_helix_key(self) -> &'static str {
        match self {
            Self::Left => "Alt h",
            Self::Right => "Alt l",
            Self::Up => "Alt k",
            Self::Down => "Alt j",
        }
    }
}

#[derive(Default)]
struct State;

impl State {
    fn request_focus_change(&mut self, direction: FocusDirection) {
        // If the focused pane is Helix, trigger its Space-w directional binding.
        if let Ok((_, pane_id)) = get_focused_pane_info() {
            if Self::is_helix_pane(pane_id) {
                let pane_id = pane_id.to_string();
                run_command(
                    &[
                        "zellij",
                        "action",
                        "send-keys",
                        "--pane-id",
                        &pane_id,
                        direction.as_helix_key(),
                    ],
                    BTreeMap::new(),
                );
                return;
            }
        }

        move_focus(direction.as_zellij_direction());
    }

    fn is_helix_pane(pane_id: PaneId) -> bool {
        if let Ok(cmd) = get_pane_running_command(pane_id) {
            return cmd.first().is_some_and(|base_cmd| base_cmd.contains("hx"));
        }
        false
    }
}

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
            PermissionType::RunCommands,
        ]);
    }

    fn update(&mut self, _event: Event) -> bool {
        false
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        if let Some(direction) = FocusDirection::from_message(&pipe_message.name) {
            self.request_focus_change(direction);
        }
        false
    }

    fn render(&mut self, _rows: usize, _cols: usize) {}
}

register_plugin!(State);
