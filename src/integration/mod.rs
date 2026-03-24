#[cfg(feature = "ratatui-integration")]
mod ratatui;

#[cfg(feature = "egui-integration")]
mod egui;

#[cfg(feature = "crossterm-integration")]
mod crossterm;

#[cfg(feature = "iced-integration")]
mod iced;
