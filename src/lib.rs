//! - [`app`] owns the application state and applies [`app::Msg`] updates.
//! - [`input`] translates terminal key events into [`app::Msg`] values.
//! - [`theme`] defines the built-in palettes and current theme selection.
//! - [`ui`] renders the current [`app::App`] state into the terminal frame.
//!
//! `main.rs` only owns the terminal lifecycle: initialize Ratatui, run the
//! event/render loop, then restore the terminal on exit.

pub mod app;
pub mod input;
pub mod theme;
pub mod ui;
