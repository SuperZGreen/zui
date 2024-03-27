use crate::{zui::widget::{OverflowState, PlacementInfo}, Axis, PositionConstraint, SpanConstraint, Widget, WidgetId};

/// A widget's entry in the WidgetStore
pub struct Entry<Message> {
    /// The user-implemented Widget trait object
    pub widget: Box<dyn Widget<Message>>,

    /// The size of the widget and its position, derived from its width, height and position
    /// constraints. Note: this is temporary information and is recalculated every frame.
    pub placement_info: PlacementInfo,

    /// The children of the widget
    pub children: Option<EntryChildren>,

    /// The width constraint of the Widget
    pub width_constraint: SpanConstraint,

    /// The height constraint of the Widget
    pub height_constraint: SpanConstraint,

    /// Determines the position of the Widget
    pub position_constraint: PositionConstraint,
}

/// Describes the children of a widget
#[derive(Clone)]
pub struct EntryChildren {
    /// The axis that the controlled children are to be added along
    pub axis: Axis,

    /// Children of the widget, contains both Floating and ParentDetermined widgets. Render order is
    /// determined by position in this list, with the earlier entries being rendered first. 
    pub ids: Vec<WidgetId>,

    /// Set by zui if the children are overflowing the parent container and contains information
    /// relevant to the overflow.
    pub overflow_state: OverflowState,
}

impl EntryChildren {
    pub fn new(axis: Axis) -> Self {
        Self {
            axis,
            ids: Vec::new(),
            overflow_state: OverflowState::None,
        }
    }
}

impl Default for EntryChildren {
    fn default() -> Self {
        Self {
            axis: Axis::Vertical,
            ids: Vec::new(),
            overflow_state: OverflowState::None,
        }
    }
}

/// The override descriptor for an Entry
#[derive(Clone)]
pub struct EntryOverrideDescriptor {
    /// The children of the widget
    pub children: Option<EntryChildren>,

    /// The width constraint of the Widget
    pub width_constraint: Option<SpanConstraint>,

    /// The height constraint of the Widget
    pub height_constraint: Option<SpanConstraint>,

    /// Determines the position of the Widget
    pub position_constraint: Option<PositionConstraint>,
}

impl Default for EntryOverrideDescriptor {
    fn default() -> Self {
        Self {
            children: None,
            width_constraint: None,
            height_constraint: None,
            position_constraint: None,
        }
    }
}

/// The default descriptor for an Entry, set by the Widget's implementation
pub struct EntryDefaultDescriptor {
    /// The children of the widget
    pub children: Option<EntryChildren>,

    /// The width constraint of the Widget
    pub width_constraint: SpanConstraint,

    /// The height constraint of the Widget
    pub height_constraint: SpanConstraint,

    /// Determines the position of the Widget
    pub position_constraint: PositionConstraint,
}

impl EntryDefaultDescriptor {

    /// Applies the overrides to the entry default desc
    pub fn apply_overrides(&mut self, overrides: EntryOverrideDescriptor) {
        // TODO: this seems wrong
        if let Some(children) = overrides.children {
            self.children = Some(children);
        }

        if let Some(width_constraint) = overrides.width_constraint {
            self.width_constraint = width_constraint;
        }

        if let Some(height_constraint) = overrides.height_constraint {
            self.height_constraint = height_constraint;
        }

        if let Some(position_constraint) = overrides.position_constraint {
            self.position_constraint = position_constraint;
        }
    }

}

