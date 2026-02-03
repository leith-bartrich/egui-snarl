//! Default input handling that replicates the original snarl behavior.
//!
//! This module provides a configurable input handler that translates
//! egui input events to [`SnarlAction`]s. The default configuration
//! matches the original hardcoded behavior.

use egui::{Context, Id, Modifiers, PointerButton, Pos2, Vec2};

use super::{SelectMode, SnarlAction, SnarlInputState};

/// Configuration for input handling.
///
/// Allows customization of modifier keys and mouse buttons used for
/// various interactions. The default configuration matches the original
/// egui-snarl hardcoded behavior.
#[derive(Debug, Clone)]
pub struct SnarlInputConfig {
    /// Modifier that enables additive selection (add to existing selection).
    /// Default: Shift
    pub additive_select: Modifiers,

    /// Modifier that enables subtractive selection (remove from selection).
    /// Default: Command (Ctrl on Windows/Linux, Cmd on macOS)
    pub subtractive_select: Modifiers,

    /// Modifier that starts rectangle selection when dragging on background.
    /// Default: Shift
    pub rect_select: Modifiers,

    /// Mouse button for primary actions (select, drag).
    /// Default: Primary (left click)
    pub primary_button: PointerButton,

    /// Mouse button for context menus.
    /// Default: Secondary (right click)
    pub context_button: PointerButton,

    /// Mouse button for canceling in-progress operations.
    /// Default: Secondary (right click)
    pub cancel_button: PointerButton,

    /// Minimum scale (zoom out limit).
    /// Default: 0.2
    pub min_scale: f32,

    /// Maximum scale (zoom in limit).
    /// Default: 2.0
    pub max_scale: f32,
}

impl Default for SnarlInputConfig {
    fn default() -> Self {
        Self {
            additive_select: Modifiers::SHIFT,
            subtractive_select: Modifiers::COMMAND,
            rect_select: Modifiers::SHIFT,
            primary_button: PointerButton::Primary,
            context_button: PointerButton::Secondary,
            cancel_button: PointerButton::Secondary,
            min_scale: 0.2,
            max_scale: 2.0,
        }
    }
}

/// Default input handler for snarl graphs.
///
/// Converts raw egui input into [`SnarlAction`]s based on the current
/// [`SnarlInputConfig`]. This provides the same behavior as the original
/// hardcoded input handling, but in a configurable and extensible form.
///
/// # Example
///
/// ```ignore
/// let mut default_input = DefaultSnarlInput::new();
/// let mut input = SnarlInputState::new();
///
/// // Poll for background-level actions (pan, zoom, rect select)
/// default_input.poll_background(ctx, snarl_id, &viewport_rect, &mut input);
///
/// // Snarl will handle these actions
/// snarl.show(&mut viewer, &style, &mut input, id, ui);
/// ```
#[derive(Debug, Clone)]
pub struct DefaultSnarlInput {
    /// The input configuration.
    pub config: SnarlInputConfig,

    /// Tracks whether we're in the middle of a rectangle selection.
    rect_select_origin: Option<Pos2>,
}

impl Default for DefaultSnarlInput {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultSnarlInput {
    /// Create a new default input handler with default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: SnarlInputConfig::default(),
            rect_select_origin: None,
        }
    }

    /// Create a new default input handler with custom configuration.
    #[must_use]
    pub fn with_config(config: SnarlInputConfig) -> Self {
        Self {
            config,
            rect_select_origin: None,
        }
    }

    /// Get the current select mode based on modifiers.
    fn select_mode(&self, modifiers: Modifiers) -> SelectMode {
        let shift = modifiers.contains(self.config.additive_select);
        let cmd = modifiers.contains(self.config.subtractive_select);

        match (shift, cmd) {
            (true, true) => SelectMode::Toggle,
            (true, false) => SelectMode::Add,
            (false, true) => SelectMode::Subtract,
            (false, false) => SelectMode::Replace,
        }
    }

    /// Poll for viewport-level actions (pan, zoom).
    ///
    /// Call this once per frame before `snarl.show()`. This handles:
    /// - Scroll wheel zoom
    /// - Middle mouse pan (or touchpad two-finger pan)
    ///
    /// Note: Rectangle selection and node-specific interactions are
    /// handled internally by snarl after the refactor is complete.
    pub fn poll_viewport(
        &mut self,
        ctx: &Context,
        _snarl_id: Id,
        viewport_rect: &egui::Rect,
        input: &mut SnarlInputState,
    ) {
        ctx.input(|i| {
            // Zoom with scroll wheel
            if let Some(hover_pos) = i.pointer.hover_pos() {
                if viewport_rect.contains(hover_pos) {
                    let scroll_delta = i.smooth_scroll_delta.y;
                    if scroll_delta.abs() > 0.0 {
                        // Convert scroll to zoom factor
                        let factor = 1.0 + scroll_delta * 0.001;
                        let factor = factor.clamp(0.9, 1.1); // Limit per-frame zoom

                        input.push(SnarlAction::Zoom {
                            center: hover_pos,
                            factor,
                        });
                    }
                }
            }

            // Pan with middle mouse or touchpad gesture
            // Note: egui's Scene handles this, but we can add custom behavior here
            // For now, we'll let snarl's internal pan handling continue to work
        });
    }

    /// Poll for keyboard shortcut actions.
    ///
    /// This can be extended to handle things like:
    /// - Delete key for removing selected nodes
    /// - Escape for deselecting or canceling
    ///
    /// For now, this is a placeholder - keyboard shortcuts are handled
    /// by the desktop's ShortcutMap which converts to GraphAction, which
    /// then converts to SnarlAction.
    pub fn poll_keyboard(
        &self,
        _ctx: &Context,
        _snarl_id: Id,
        _input: &mut SnarlInputState,
    ) {
        // Placeholder - keyboard handling is done by desktop's ShortcutMap
        // which converts GraphAction to SnarlAction externally
    }

    /// Start tracking a rectangle selection.
    pub fn start_rect_select(&mut self, origin: Pos2) {
        self.rect_select_origin = Some(origin);
    }

    /// Update rectangle selection tracking.
    ///
    /// Returns the current selection rectangle if active.
    #[must_use]
    pub fn update_rect_select(&self, current: Pos2) -> Option<egui::Rect> {
        self.rect_select_origin
            .map(|origin| egui::Rect::from_two_pos(origin, current))
    }

    /// End rectangle selection and emit the action.
    pub fn end_rect_select(
        &mut self,
        current: Pos2,
        modifiers: Modifiers,
        input: &mut SnarlInputState,
    ) {
        if let Some(origin) = self.rect_select_origin.take() {
            let rect = egui::Rect::from_two_pos(origin, current);
            let mode = self.select_mode(modifiers);

            input.push(SnarlAction::RectSelect { rect, mode });
        }
    }

    /// Cancel rectangle selection without emitting an action.
    pub fn cancel_rect_select(&mut self) {
        self.rect_select_origin = None;
    }

    /// Check if rectangle selection is in progress.
    #[must_use]
    pub fn is_rect_selecting(&self) -> bool {
        self.rect_select_origin.is_some()
    }

    /// Get the selection mode configuration.
    #[must_use]
    pub fn config(&self) -> &SnarlInputConfig {
        &self.config
    }

    /// Get mutable access to the configuration.
    pub fn config_mut(&mut self) -> &mut SnarlInputConfig {
        &mut self.config
    }
}

/// Convert a pan delta to a [`SnarlAction::Pan`].
#[must_use]
pub fn pan_action(delta: Vec2) -> SnarlAction {
    SnarlAction::Pan(delta)
}

/// Convert zoom parameters to a [`SnarlAction::Zoom`].
#[must_use]
pub fn zoom_action(center: Pos2, factor: f32) -> SnarlAction {
    SnarlAction::Zoom { center, factor }
}
