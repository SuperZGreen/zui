use rustc_hash::FxHashSet;

use super::{
    primitives::Dimensions, render_layer::RenderLayer, renderers::SimpleVertex,
    renderers::TextVertex, typeface::SymbolKey, widget_store::EntryDefaultDescriptor, Rectangle,
};
use std::{any::Any, collections::VecDeque};

use crate::zui::Context;

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

pub struct ChildPlacementDescriptor {
    /// The axis that the children will be placed along
    pub axis: Axis,
}

#[allow(unused_variables)]
pub trait Widget<Message> {
    /// Handles an input event for the widget. Region is the region of the placed Widget
    fn handle_event(
        &mut self,
        event: &Event,
        region: &Rectangle<i32>,
        context: &Context,
    ) -> Option<Message> {
        // Do nothing
        None
    }

    /// Asks the widget to calculate its dimensions provided the LayoutBoundaries and width and
    /// height SpanConstraints. The Widget will return a DimensionsError if it can not do this on
    /// its own, and the dimensions calculation will then be performed by the parent
    fn calculate_minimum_dimensions(
        &mut self,
        layout_boundaries: &LayoutBoundaries,
        context: &Context,
    ) -> Dimensions<i32> {
        // Returns 0 dimensions by default
        Dimensions::new(0i32, 0i32)
    }

    /// Adds the Text's SymbolKeys to the symbol_keys FxHashSet for this widget. Does not propagate
    /// to chilren as it is called on all valid entries by the WidgetStore. This is to ensure that
    /// the font system has the required characters rasterised and ready to render when text
    /// Presymbols are generated
    fn collect_text(&self, symbol_keys: &mut FxHashSet<SymbolKey>) {
        // Do nothing
    }

    /// Allows the widget to update any extra internals required when the widget is placed
    fn place(&mut self, region: &Rectangle<i32>, context: &Context) {
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
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub enum BoundaryType {
    /// Indicates that the parent would not like this boundary overflown, if possible
    Static,
    /// Indicates that the parent doesn't mind if this boundary is overflown
    Dynamic,
}

pub struct Boundary {
    pub boundary_type: BoundaryType,
    pub span_px: i32,
}

impl Boundary {
    pub fn new(boundary_type: BoundaryType, span_px: i32) -> Self {
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

/// Converts LayoutBoundaries to Dimensions<i32> for SpanConstraint::ParentWidth/Height calculation
impl Into<Dimensions<i32>> for &LayoutBoundaries {
    fn into(self) -> Dimensions<i32> {
        let width = self.horizontal.span_px;
        let height = self.vertical.span_px;

        Dimensions::new(width, height)
    }
}

impl From<&Rectangle<i32>> for LayoutBoundaries {
    fn from(value: &Rectangle<i32>) -> Self {
        LayoutBoundaries::new(
            Boundary::new(BoundaryType::Static, value.width()),
            Boundary::new(BoundaryType::Static, value.height()),
        )
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
