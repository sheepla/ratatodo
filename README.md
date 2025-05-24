<div align="center">

# ✅️ ratatodo

</div>

This is a sample TUI TODO app with an asynchronous, event-driven, designed with a Flux-like architecture built with [ratatui](https://ratatui.rs) and [crossterm](https://docs.rs/crossterm/latest/crossterm/)

<div align="center">

![screenshot](./assets/screenshot.png)

</div>

## Features

- [x] View todo entries 
- [x] Moving cursor
- [x] Delete current todo entry
- [x] Loading / Saving todo data automatically in JSON
- [x] Add and Edit TODO entries with textarea
- [ ] Customizable key bindings in config file
- [x] Non-blocking action execution handling

## Usage

- `k`, `j`, `Up`, `Down`: Move focus
- `i`, `a`: Focus to textarea
- `Enter`: Accept current entry
- `Esc`: Focus to todo list view
- `Space`: Toggle completed/incomplete state
- `r`: Run some heavy task (Reproduces pseudo-heavy tasks. While the task is running, the status bar is updated with `Loading...` is displayed while the task is running)


