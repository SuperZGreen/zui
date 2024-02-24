use crate::{zui::widget::Layout, Axis, PositionConstraint, SpanConstraint, Widget, WidgetId};

/// A widget's entry in the WidgetStore
pub struct Entry<Message> {
    /// The user-implemented Widget trait object
    pub widget: Box<dyn Widget<Message>>,

    /// The size of the widget and its position
    pub layout: Layout,

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

    /// Children of the widget, including Floating and ParentDetermined
    pub ids: Vec<WidgetId>,
}

impl EntryChildren {
    pub fn new(axis: Axis) -> Self {
        Self {
            axis,
            ids: Vec::new(),
        }
    }
}

impl Default for EntryChildren {
    fn default() -> Self {
        Self {
            axis: Axis::Vertical,
            ids: Vec::new(),
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

