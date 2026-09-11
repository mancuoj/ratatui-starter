//! A small [Ratatui] starter built around a minimal Elm architecture.
//!
//! - [`app`] owns the application state and applies [`app::Msg`] updates.
//! - [`input`] translates terminal key events into [`app::Msg`] values.
//! - [`theme`] defines the built-in palettes and current theme selection.
//! - [`ui`] renders the current [`app::App`] state into the terminal frame.
//!
//! Data flows one way: key event → [`input::translate`] → [`app::Msg`] →
//! [`app::App::update`] → new state → [`ui::render`].
//!
//! `main.rs` only owns the terminal lifecycle, and delegates it to
//! [`ratatui::run`]: that call enters raw mode and the alternate screen,
//! installs a panic hook so a crash restores the terminal instead of leaving it
//! in raw mode, runs the event/render loop, and restores the terminal on exit.

pub mod app;
pub mod input;
pub mod theme;
pub mod ui;
