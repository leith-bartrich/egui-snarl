//! Input state for collecting and processing snarl actions.

use super::SnarlAction;

/// Collects actions to be processed during the current frame.
///
/// Actions are injected by input handlers (keyboard, mouse, touch) and
/// consumed by the snarl during rendering. This provides a clean separation
/// between input handling and graph manipulation.
///
/// # Example
///
/// ```ignore
/// let mut input = SnarlInputState::default();
///
/// // Inject actions from various sources
/// if keyboard.ctrl_a_pressed() {
///     input.push(SnarlAction::SelectAll);
/// }
/// if let Some(delta) = mouse.drag_delta() {
///     input.push(SnarlAction::Pan(delta));
/// }
///
/// // Pass to snarl.show()
/// snarl.show(&mut viewer, &style, &mut input, ui);
/// ```
#[derive(Debug, Default, Clone)]
pub struct SnarlInputState {
    actions: Vec<SnarlAction>,
}

impl SnarlInputState {
    /// Create a new empty input state.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Push an action to be processed this frame.
    pub fn push(&mut self, action: SnarlAction) {
        self.actions.push(action);
    }

    /// Push multiple actions to be processed this frame.
    pub fn extend(&mut self, actions: impl IntoIterator<Item = SnarlAction>) {
        self.actions.extend(actions);
    }

    /// Returns true if there are no pending actions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    /// Returns the number of pending actions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.actions.len()
    }

    /// Drain all pending actions for processing.
    pub fn drain(&mut self) -> impl Iterator<Item = SnarlAction> + '_ {
        self.actions.drain(..)
    }

    /// Clear all pending actions without processing them.
    pub fn clear(&mut self) {
        self.actions.clear();
    }

    /// Peek at pending actions without consuming them.
    pub fn peek(&self) -> &[SnarlAction] {
        &self.actions
    }
}
