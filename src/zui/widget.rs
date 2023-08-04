use rustc_hash::FxHashSet;

use super::{
    primitives::Dimensions,
    render_layer::RenderLayer,
    simple_renderer::SimpleVertex,
    text_renderer::TextVertex,
    typeface::SymbolKey,
    widget_store::EntryDefaultDescriptor,
    Rectangle,
};
use std::{any::Any, collections::VecDeque};

use crate::{zui::Context, SpanConstraint, WidgetStore};

#[derive(Copy, Clone)]
pub enum Axis {
    Vertical,
    Horizontal,
}

impl Axis {
    pub fn to_index(&self) -> usize {
        match self {
            Axis::Horizontal => 0usize,
            Axis::Vertical => 1usize,
        }
    }

    /// Gives the perpendicular axis to self
    pub fn other(&self) -> Self {
        match self {
            Axis::Vertical => Axis::Horizontal,
            Axis::Horizontal => Axis::Vertical,
        }
    }
}

#[derive(Debug)]
pub enum MouseEvent {
    ButtonPressed,
    ButtonReleased,
    _CursorEnteredWindow,
    CursorExitedWindow,

    /// Event for when the mouse cursor is moved, the position of the cursor is provided in the
    /// provided context
    CursorMoved,
}

/// An event that is passed to a Widget from the outside context, for the widget to deal with and/or
/// to pass on to its children
#[derive(Debug)]
pub enum Event {
    /// An event that involves mouse interaction
    MouseEvent(MouseEvent),
    // /// The widget is commanded to fit the provided rectangle
    // FitRectangle(Rectangle<f32>),
}

/// Error returned by Widget::calculate_dimensions
#[derive(Debug)]
pub enum DimensionsError {
    /// Is given if the Widget contains a SpanConstraint::ParentWeight, where the dimensions of the
    /// widget is unable to be calculated by the Widget in isolation or with its children, and
    /// instead is determined by the Parent.
    IsSetByParent,

    /// This is returned if the Widget needs to fit its children, as the Widget object in isolation
    /// has no concept of its children, this needs to be performed by the WidgetEntry, which is
    /// made aware of the fact it needs to perform this calculation by this error value
    FitsChildren,

    /// A catch all error
    Other,
}

pub struct ChildPlacementDescriptor {
    /// The axis that the children will be placed along
    pub axis: Axis,
}

#[allow(unused_variables)]
pub trait Widget<Message> {
    /// Handles an input event for the widget. Region is the region of the placed Widget
    fn handle_event(&mut self, event: &Event, region: &Rectangle<i32>, context: &Context) {
        // Do nothing
    }

    /// Asks the widget to calculate its dimensions provided the LayoutBoundaries and width and
    /// height SpanConstraints. The Widget will return a DimensionsError if it can not do this on
    /// its own, and the dimensions calculation will then be performed by the parent
    fn calculate_minimum_dimensions(
        &self,
        layout_boundaries: &LayoutBoundaries,
        width_contraint: SpanConstraint,
        height_contraint: SpanConstraint,
        context: &Context,
    ) -> Result<Dimensions<i32>, DimensionsError>;

    /// Widgets insert all characters (SymbolKeys) that they will need to render their Text, and
    /// propogates the call to their children. This is to ensure that the font system has the
    /// required characters rasterised and ready to render when text Presymbols are generated
    fn collect_text(
        &self,
        widget_store: &WidgetStore<Message>,
        symbol_keys: &mut FxHashSet<SymbolKey>,
    ) {
        // Do nothing
    }

    /// Returns the vertices necessary to render a widget, will append a new RenderLayer to
    /// render_layers to allow for a new clipping region when overflowing. This is not normally
    /// required by widgets that do not have nested children/do not have grand-children that may
    /// overflow
    fn to_vertices(
        &self,
        // the area that the widget must reside in
        region: Rectangle<i32>,
        context: &Context,
        simple_vertices: &mut Vec<SimpleVertex>,
        text_vertices: &mut Vec<TextVertex>,
        render_layers: &mut VecDeque<RenderLayer>,
    ) {
        // Do nothing by default
    }

    /// Describes how the WidgetStore should lay out the widget's children. Most Widgets will not
    /// implement this
    /// TODO
    fn child_placement_descriptor(&self) -> Option<ChildPlacementDescriptor> {
        None
    }

    fn entry_default_descriptor(&self) -> EntryDefaultDescriptor;

    fn as_any(&self) -> &dyn Any;
}

pub enum BoundaryType {
    /// Indicates that the parent would not like this boundary overflown, if possible
    Static,
    /// Indicates that the parent doesn't mind if this boundary is overflown
    Dynamic,
}

pub struct Boundary {
    pub boundary_type: BoundaryType,
    pub span_px: f32,
}

impl Boundary {
    pub fn new(boundary_type: BoundaryType, span_px: f32) -> Self {
        Self {
            boundary_type,
            span_px,
        }
    }
}

/// Describes the boundaries of a layout that is given to a child widget to calculate its minimum
/// dimensions
pub struct LayoutBoundaries {
    pub horizontal: Boundary,
    pub vertical: Boundary,
}

impl LayoutBoundaries {
    pub fn new(horizontal: Boundary, vertical: Boundary) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }

    pub fn by_axis<'a>(&'a self, axis: &Axis) -> &'a Boundary {
        match axis {
            Axis::Vertical => &self.vertical,
            Axis::Horizontal => &self.horizontal,
        }
    }
}

/// Provides a bound on an axis, to allow a widget to expand in the non-bounded axis
pub struct Bounds<T> {
    /// The limit in that axis
    pub span: T,
}

/// The layout of a widget
pub struct Layout {
    /// The minimum dimensions of the contents of the Widget, used for determining placement by a
    /// parent Container.
    pub minimum_dimensions_px: Option<Dimensions<i32>>,

    /// The rectangle that has been given to the widget by its parents to be placed in. This
    /// describes the widget's placement on the viewport/screen.
    pub clip_rectangle_px: Option<Rectangle<i32>>,
}

impl Layout {
    pub fn new() -> Self {
        Self {
            minimum_dimensions_px: None,
            clip_rectangle_px: None,
        }
    }
}
