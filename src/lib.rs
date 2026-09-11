//! A small [Ratatui] starter built around a minimal Elm architecture.
//!
//! - [`app`] owns the application state and applies [`app::Msg`] updates.
//! - [`input`] translates terminal key events into [`app::Msg`] values.
//! - [`theme`] defines the built-in palettes and current theme selection.
//! - [`ui`] renders the current [`app::App`] state into the terminal frame.
//!
//! Data flows one way: key event → [`input::translate`] → [`app::Msg`] →
//! [`app::App::update`] → new state → [`ui::render`]. The loop also emits
//! [`app::Msg::Tick`] on a fixed interval, so time-based state (spinners,
//! animations) travels the same path as any other message.
//!
//! `main.rs` only owns the terminal lifecycle, and delegates it to
//! [`ratatui::run`].

pub mod app;
pub mod input;
pub mod theme;
pub mod ui;
