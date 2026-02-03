//! Action-based input abstraction for Snarl graphs.
//!
//! This module provides a clean separation between input handling and graph
//! manipulation. Instead of egui-snarl directly interpreting mouse/keyboard
//! events, actions are injected via [`SnarlInputState`] and processed uniformly.
//!
//! # Architecture
//!
//! ```text
//! Input Sources                    Action Processing
//! ┌─────────────┐                 ┌─────────────────┐
//! │  Keyboard   │──┐              │                 │
//! ├─────────────┤  │              │  SnarlInputState│
//! │   Mouse     │──┼─SnarlAction─▶│     .drain()    │──▶ Snarl State
//! ├─────────────┤  │              │                 │
//! │   Touch     │──┘              │                 │
//! └─────────────┘                 └─────────────────┘
//! ```
//!
//! # Benefits
//!
//! - **Customizable bindings**: Consumers control how input maps to actions
//! - **Touch support**: Touch gestures can emit the same actions as mouse
//! - **Testability**: Actions can be injected without real input
//! - **Consistency**: All state changes flow through the same path
//!
//! # Example
//!
//! ```ignore
//! use egui_snarl::action::{SnarlAction, SnarlInputState};
//!
//! let mut input = SnarlInputState::new();
//!
//! // Map keyboard shortcuts to actions
//! if ctx.input(|i| i.key_pressed(Key::A) && i.modifiers.command) {
//!     input.push(SnarlAction::SelectAll);
//! }
//!
//! // Map mouse gestures to actions
//! if let Some(delta) = response.drag_delta() {
//!     input.push(SnarlAction::Pan(delta));
//! }
//!
//! // Snarl processes all actions uniformly
//! snarl.show(&mut viewer, &style, &mut input, ui);
//! ```

mod default_input;
mod input_state;
mod snarl_action;

pub use default_input::{DefaultSnarlInput, SnarlInputConfig};
pub use input_state::SnarlInputState;
pub use snarl_action::{SelectMode, SnarlAction};
