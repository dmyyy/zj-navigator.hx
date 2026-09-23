use std::collections::{BTreeMap, HashMap};
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
struct State {
    external_tui_depths: HashMap<PaneId, u32>,
}

impl State {
    fn request_focus_change(&mut self, direction: FocusDirection) {
        // If the focused pane is Helix, trigger its Space-w directional binding.
        if let Ok((_, pane_id)) = get_focused_pane_info() {
            if self.is_external_tui(pane_id) {
                move_focus(direction.as_zellij_direction());
                return;
            }

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

    fn is_external_tui(&self, pane_id: PaneId) -> bool {
        self.external_tui_depths.contains_key(&pane_id)
    }

    fn update_external_tui_state(&mut self, message: &str, payload: Option<&str>) {
        let Some(pane_id) = payload.and_then(|payload| payload.parse::<PaneId>().ok()) else {
            return;
        };

        match message {
            "external_tui_enter" => {
                self.external_tui_depths
                    .entry(pane_id)
                    .and_modify(|depth| *depth = depth.saturating_add(1))
                    .or_insert(1);
            }
            "external_tui_exit" => {
                let remove_pane = match self.external_tui_depths.get_mut(&pane_id) {
                    Some(depth) if *depth > 1 => {
                        *depth -= 1;
                        false
                    }
                    _ => true,
                };

                if remove_pane {
                    self.external_tui_depths.remove(&pane_id);
                }
            }
            _ => {}
        }
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
        self.update_external_tui_state(&pipe_message.name, pipe_message.payload.as_deref());

        if let Some(direction) = FocusDirection::from_message(&pipe_message.name) {
            self.request_focus_change(direction);
        }
        false
    }

    fn render(&mut self, _rows: usize, _cols: usize) {}
}

register_plugin!(State);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn external_tui_state_is_pane_local_and_nestable() {
        let mut state = State::default();
        let pane = PaneId::Terminal(1);

        state.update_external_tui_state("external_tui_enter", Some("terminal_1"));
        state.update_external_tui_state("external_tui_enter", Some("terminal_1"));
        assert!(state.is_external_tui(pane));
        assert!(!state.is_external_tui(PaneId::Terminal(2)));

        state.update_external_tui_state("external_tui_exit", Some("terminal_1"));
        assert!(state.is_external_tui(pane));

        state.update_external_tui_state("external_tui_exit", Some("terminal_1"));
        assert!(!state.is_external_tui(pane));
    }

    #[test]
    fn external_tui_state_ignores_invalid_pane_ids() {
        let mut state = State::default();

        state.update_external_tui_state("external_tui_enter", Some("not-a-pane"));
        assert!(state.external_tui_depths.is_empty());
    }
}
