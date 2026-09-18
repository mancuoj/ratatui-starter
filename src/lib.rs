//! A small [Ratatui] starter built around a minimal Elm architecture.
//!
//! Data flows one way: key event → [`input::translate`] → [`app::Msg`] →
//! [`app::App::update`] → new state → [`ui::render`].
//!
//! `main.rs` only owns the terminal lifecycle.

pub mod app;
pub mod input;
pub mod theme;
pub mod ui;
