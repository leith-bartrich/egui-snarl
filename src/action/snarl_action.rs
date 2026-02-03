//! Snarl action definitions.

use egui::{Pos2, Rect, Vec2};

use crate::ui::{AnyPin, WireId};
use crate::NodeId;

/// Mode for rectangle selection operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectMode {
    /// Replace current selection with nodes in rect.
    Replace,
    /// Add nodes in rect to current selection.
    Add,
    /// Remove nodes in rect from current selection.
    Subtract,
    /// Toggle selection state of nodes in rect.
    Toggle,
}

/// Actions that can be injected into the snarl graph.
///
/// These represent semantic intentions, not raw input events.
/// Actions can come from keyboard shortcuts, mouse gestures, touch
/// interactions, or programmatic control.
#[derive(Debug, Clone)]
pub enum SnarlAction {
    // === Selection ===
    /// Select a single node.
    SelectNode {
        /// Node to select.
        node: NodeId,
        /// If true, add to existing selection. If false, replace selection.
        additive: bool,
    },

    /// Remove a node from the current selection.
    DeselectNode(NodeId),

    /// Select all nodes in the graph.
    SelectAll,

    /// Clear the current selection.
    DeselectAll,

    /// Perform rectangle selection.
    RectSelect {
        /// The selection rectangle in graph coordinates.
        rect: Rect,
        /// How to apply the selection.
        mode: SelectMode,
    },

    // === Node Movement ===
    /// Move a node by the given delta.
    ///
    /// If the node is part of the current selection, all selected nodes
    /// are moved by the same delta.
    MoveNode {
        /// The node being dragged.
        node: NodeId,
        /// Movement delta in graph coordinates.
        delta: Vec2,
    },

    /// Bring a node to the front of the draw order.
    BringToTop(NodeId),

    /// Toggle a node's collapsed/expanded state.
    ToggleOpen(NodeId),

    // === Wires ===
    /// Start dragging a new wire from a pin.
    StartWire(AnyPin),

    /// Cancel the in-progress wire drag.
    CancelWire,

    /// Disconnect an existing wire.
    DisconnectWire(WireId),

    // === Viewport ===
    /// Pan the viewport by the given delta.
    Pan(Vec2),

    /// Zoom the viewport around a center point.
    Zoom {
        /// Center point for the zoom operation (in screen coordinates).
        center: Pos2,
        /// Zoom factor (>1 zooms in, <1 zooms out).
        factor: f32,
    },

    /// Fit all nodes into the visible viewport.
    FitToView,

    /// Reset zoom to 100%.
    ResetZoom,

    /// Center the viewport on a specific rectangle.
    CenterOn(Rect),

    // === Menus ===
    /// Request to open the context menu for a node.
    NodeMenu {
        /// The node to show menu for.
        node: NodeId,
        /// Position to show menu at (in graph coordinates).
        pos: Pos2,
    },

    /// Request to open the graph background context menu.
    GraphMenu(Pos2),

    /// Request to open the dropped wire menu.
    WireDropMenu {
        /// The pins the wire was dragged from.
        pins: AnyPin,
        /// Position where the wire was dropped.
        pos: Pos2,
    },
}

impl SnarlAction {
    /// Returns whether this action modifies the graph state.
    ///
    /// Actions that modify state may trigger undo checkpoints in the
    /// consuming application.
    #[must_use]
    pub fn modifies_state(&self) -> bool {
        match self {
            // Selection changes are typically ephemeral
            SnarlAction::SelectNode { .. }
            | SnarlAction::DeselectNode(_)
            | SnarlAction::SelectAll
            | SnarlAction::DeselectAll
            | SnarlAction::RectSelect { .. } => false,

            // Node movement modifies positions
            SnarlAction::MoveNode { .. } => true,

            // Draw order and open state are metadata, not graph structure
            SnarlAction::BringToTop(_) | SnarlAction::ToggleOpen(_) => false,

            // Wire operations modify graph structure
            SnarlAction::StartWire(_) => false, // Starting doesn't modify yet
            SnarlAction::CancelWire => false,
            SnarlAction::DisconnectWire(_) => true,

            // Viewport changes don't modify graph
            SnarlAction::Pan(_)
            | SnarlAction::Zoom { .. }
            | SnarlAction::FitToView
            | SnarlAction::ResetZoom
            | SnarlAction::CenterOn(_) => false,

            // Menus don't modify anything directly
            SnarlAction::NodeMenu { .. }
            | SnarlAction::GraphMenu(_)
            | SnarlAction::WireDropMenu { .. } => false,
        }
    }

    /// Returns a human-readable description of this action.
    #[must_use]
    pub fn description(&self) -> &'static str {
        match self {
            SnarlAction::SelectNode { additive: true, .. } => "Add to selection",
            SnarlAction::SelectNode { additive: false, .. } => "Select node",
            SnarlAction::DeselectNode(_) => "Deselect node",
            SnarlAction::SelectAll => "Select all",
            SnarlAction::DeselectAll => "Deselect all",
            SnarlAction::RectSelect { .. } => "Rectangle select",
            SnarlAction::MoveNode { .. } => "Move node",
            SnarlAction::BringToTop(_) => "Bring to top",
            SnarlAction::ToggleOpen(_) => "Toggle collapsed",
            SnarlAction::StartWire(_) => "Start wire",
            SnarlAction::CancelWire => "Cancel wire",
            SnarlAction::DisconnectWire(_) => "Disconnect wire",
            SnarlAction::Pan(_) => "Pan",
            SnarlAction::Zoom { .. } => "Zoom",
            SnarlAction::FitToView => "Fit to view",
            SnarlAction::ResetZoom => "Reset zoom",
            SnarlAction::CenterOn(_) => "Center on",
            SnarlAction::NodeMenu { .. } => "Node menu",
            SnarlAction::GraphMenu(_) => "Graph menu",
            SnarlAction::WireDropMenu { .. } => "Wire drop menu",
        }
    }
}
