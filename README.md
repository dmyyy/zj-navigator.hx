# zj-navigator

Navigate seamlessly between Helix splits and Zellij panes with `Alt-h`, `Alt-j`, `Alt-k`, and `Alt-l`.

## Installation

Clone the repo and

```
forge install
```

Add to `init.scm`

```scheme
(require "zj-navigator/zj-navigator.scm")
```

Build the accompanying Zellij plugin:

```
cd hx-navigator
cargo build --release
```

Then add these bindings to the `shared_except "locked"` block in `~/.config/zellij/config.kdl`. Replace `/path/to/zj-navigator.hx` with the cloned repository path.

```kdl
shared_except "locked" {
    bind "Alt h" {
        MessagePlugin "file:/path/to/zj-navigator.hx/hx-navigator/target/wasm32-wasip1/release/hx-navigator.wasm" {
            name "focus_left";
        }
    }
    bind "Alt j" {
        MessagePlugin "file:/path/to/zj-navigator.hx/hx-navigator/target/wasm32-wasip1/release/hx-navigator.wasm" {
            name "focus_down";
        }
    }
    bind "Alt k" {
        MessagePlugin "file:/path/to/zj-navigator.hx/hx-navigator/target/wasm32-wasip1/release/hx-navigator.wasm" {
            name "focus_up";
        }
    }
    bind "Alt l" {
        MessagePlugin "file:/path/to/zj-navigator.hx/hx-navigator/target/wasm32-wasip1/release/hx-navigator.wasm" {
            name "focus_right";
        }
    }
}
```

Add plugin to config

```kdl
load_plugins {
    "file:/path/to/zj-navigator.hx/hx-navigator/target/wasm32-wasip1/release/hx-navigator.wasm"
}
```

Approve the plugin permissions when Zellij asks. It forwards the keys to Helix when the focused pane is running `hx`; otherwise it moves Zellij focus.
