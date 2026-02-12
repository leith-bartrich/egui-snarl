//! Input state for collecting and processing snarl actions.

use super::SnarlAction;

/// Selection policy for the current frame, set by the editor.
///
/// Controls how node clicks and rect-drags affect selection state.
/// The editor sets this each frame based on modifier keys or input mode.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SelectionMode {
    /// Selection inactive — node clicks don't select.
    #[default]
    Inactive,
    /// Replace selection with clicked/rect'd nodes.
    Replace,
    /// Toggle clicked/rect'd nodes in/out of selection.
    Toggle,
    /// Remove clicked/rect'd nodes from selection.
    Subtract,
}

/// Collects actions and input state to be processed during the current frame.
///
/// Actions are injected by input handlers (keyboard, mouse, touch) and
/// consumed by the snarl during rendering. The `selection_mode` and
/// `nav_active` fields persist across `drain()` and control how the
/// snarl handles built-in node interactions.
///
/// # Example
///
/// ```ignore
/// let mut input = SnarlInputState::default();
///
/// // Set selection mode based on modifier keys
/// input.selection_mode = SelectionMode::Replace;
///
/// // Inject actions from various sources
/// if keyboard.ctrl_a_pressed() {
///     input.push(SnarlAction::SelectAll);
/// }
/// if navigating {
///     input.push(SnarlAction::Pan(delta));
/// }
///
/// // Pass to snarl.show()
/// snarl.show(&mut viewer, &style, &mut input, ui);
/// ```
#[derive(Debug, Default, Clone)]
pub struct SnarlInputState {
    actions: Vec<SnarlAction>,
    /// Current selection mode (set by editor each frame).
    pub selection_mode: SelectionMode,
    /// Navigation mode active — suppresses node dragging.
    pub nav_active: bool,
    /// Read-only mode — suppresses all graph mutations (node movement,
    /// wire creation/disconnection, pin drags). Pan and zoom still work.
    pub read_only: bool,
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
