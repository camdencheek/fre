Integration of `tmux` with `pass`, using `fre` to show frecent selections first via `fzf`.

Inspired by [`passmux`](https://github.com/hughdavenport/passmux/)

# Prerequisites

- [pass](https://www.passwordstore.org/)
- [fzf](https://github.com/junegunn/fzf/)
- [tmux](https://github.com/tmux/tmux/)

# Installation

1. Copy `pass-tmux` to `~/.local/bin` (or somewhere in your path)
2. In `tmux.conf`, add the following key bindings:

    ```
    bind-key -T prefix C-p run-shell -b 'pass-tmux'
    bind-key -T prefix C-t run-shell -b 'pass-tmux --paste'
    ```

Note: the `--paste` option inserts the retrieved password into the active pane (without sending Enter).
