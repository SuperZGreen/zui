use rustc_hash::FxHashSet;
use winit::dpi::{PhysicalPosition, PhysicalSize};

use super::{
    font::SymbolKey, primitives::Dimensions, render_layer::RenderLayer,
    simple_renderer::SimpleVertex, text_renderer::TextVertex, Rectangle, Typeface,
};
use std::collections::VecDeque;

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

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Span {
    //
    //  Fixed Spans: can be calculated given some environmental constants (viewport dimensions),
    //               and the Span value itself
    //
    /// Fixed Span, Size as a portion of the view height, ie. the height of the application window
    /// surface is 1
    ViewWidth(f32),

    /// Fixed Span: Size as a portion of the view width, ie. the width of the application window
    /// surface is 1
    ViewHeight(f32),

    /// Fixed Span: Relative size with respect to the minimum dimension of the wgpu viewport
    ViewMin(f32),

    /// Fixed Span: The length in pixels of the Span
    Pixels(f32),

    //
    //  Contents based sizes, dynamically resizes based on the size of the contents. This is text in
    //  the case of a Container.
    //
    FitContents,

    // TODO: ViewMax?

    //
    //  Parent-based Sizes, dynamically resizes depending on the size of the parent widget
    //
    /// Weighted size with respect to the parent's size. Is summed up and divided amongst other
    /// child's sizes to determine the actual screen-space size of the widget
    ParentWeight(f32),

    /// The size as a proportion of the parent's size
    ParentRatio(f32),
}

impl Span {
    pub fn view_min_to_span_px(view_min: f32, viewport_dimensions_px: PhysicalSize<u32>) -> f32 {
        if viewport_dimensions_px.width < viewport_dimensions_px.height {
            Self::view_width_to_span_px(view_min, viewport_dimensions_px.width)
        } else {
            Self::view_height_to_span_px(view_min, viewport_dimensions_px.height)
        }
    }

    pub fn view_height_to_span_px(view_height: f32, viewport_height_px: u32) -> f32 {
        view_height as f32 * viewport_height_px as f32
    }

    pub fn view_width_to_span_px(
        view_width: f32,
        // The width of the viewport in pixels
        viewport_width_px: u32,
    ) -> f32 {
        view_width as f32 * viewport_width_px as f32
    }

    /// Converts the span into a pixel span value given context and parent widget region and axis
    pub fn to_viewport_px(
        &self,
        parent_rectangle: &Rectangle<f32>,
        parent_axis: Axis,
        sum_of_parent_weights: Option<f32>,
        // the amount of screen space not taken up by non-weighted widgets
        parent_span_px_available: Option<u32>,
        context: &Context,
        fit_contents_span_px: f32,
    ) -> f32 {
        match self {
            Span::ViewWidth(vw) => {
                Self::view_width_to_span_px(*vw, context.viewport_dimensions_px.width)
            }
            Span::ViewHeight(vh) => {
                Self::view_height_to_span_px(*vh, context.viewport_dimensions_px.height)
            }
            Span::ViewMin(vm) => Self::view_min_to_span_px(*vm, context.viewport_dimensions_px),
            Span::Pixels(px) => *px,
            Span::ParentWeight(pw) => {
                if sum_of_parent_weights.is_none() || parent_span_px_available.is_none() {
                    0f32
                } else {
                    *pw / sum_of_parent_weights.unwrap() * parent_span_px_available.unwrap() as f32
                }
            }
            Span::ParentRatio(ratio) => *ratio * parent_rectangle.span_by_axis(parent_axis) as f32,
            Span::FitContents => fit_contents_span_px,
        }
    }

    /// Converts fixed type Spans to f32 viewport pixels
    pub fn min_span_px(
        &self,
        axis: Axis,
        region_dimensions_px: Dimensions<f32>,
        viewport_dimensions_px: PhysicalSize<u32>,
    ) -> Option<f32> {
        match self {
            Span::ViewWidth(vw) => Some(Self::view_width_to_span_px(
                *vw,
                viewport_dimensions_px.width,
            )),
            Span::ViewHeight(vh) => Some(Self::view_height_to_span_px(
                *vh,
                viewport_dimensions_px.height,
            )),
            Span::ViewMin(vm) => Some(Self::view_min_to_span_px(*vm, viewport_dimensions_px)),
            Span::Pixels(p) => Some(*p),
            Span::ParentRatio(pr) => {
                let parent_span_px = region_dimensions_px.span_by_axis(axis);
                Some(pr * parent_span_px)
            }

            // This is also potentially a 'fixed' type, but can not be calculated here, so returns
            // None instead
            Span::FitContents => None,
            Span::ParentWeight(_) => None,
        }
    }
}

#[derive(Debug)]
pub enum MouseEvent {
    ButtonPressed,
    ButtonReleased,
    CursorEnteredWindow,
    CursorExitedWindow,
    CursorMoved(PhysicalPosition<f64>),
}

#[derive(Debug)]
pub enum WindowEvent {
    // Resized {
    //     /// Width in pixels
    //     width: u32,
    //     /// Height in pixels
    //     height: u32,
    // },
}

#[derive(Debug)]
pub enum Event {
    /// An event that involves mouse interaction
    MouseEvent(MouseEvent),

    // /// The widget is commanded to fit the provided rectangle
    // FitRectangle(Rectangle<f32>),
}

pub enum EventResponse<Message> {
    /// The [`Event`] is consumed by the widget and will not be propagated to its children
    Consumed,
    /// The [`Event`] is consumed by the widget and a message is produced
    Message(Message),
    /// The [`Event`] is not consumed by the widget and will be propagated to its children
    Propagate,
}

#[allow(unused_variables)]
pub trait Widget<Message> {
    /// Handles interaction events, returning the EventResponse that determines whether events
    /// should be propagated to children
    fn handle_event(&mut self, event: &Event, context: &Context) -> EventResponse<Message> {
        EventResponse::Propagate
    }

    /// Gives the children of the [`Widget`] as a slice
    fn children(&self) -> &[Box<(dyn Widget<Message>)>] {
        &[]
    }

    /// Gives the children of the [`Widget`] as a mutable slice
    fn children_mut(&mut self) -> &mut [Box<(dyn Widget<Message>)>] {
        &mut []
    }

    /// This tells the children to update their internal layouts (ie for text etc), such that they
    /// can calculate their minimum dimensions in the next step. The Widget must comply with the
    /// LayoutBoundaries, given to it by its parents. Gives the minimum possible dimensions of the
    /// child widget given the provided layout_boundaries
    fn try_update_dimensions(
        &mut self,
        layout_boundaries: &LayoutBoundaries,
        context: &Context,
    ) -> Dimensions<f32>;
    
    /// Gives the layout struct of self
    fn layout<'a>(&'a self) -> &'a Layout;

    /// Tells the widget to place itself in the rectangle
    fn try_fit_rectangle(&mut self, clip_rectangle: &Rectangle<f32>, context: &Context);

    // calls update_contents_dimensions and fit_rectangle for children
    fn place_children(&mut self, context: &Context) {
        // Do nothing by default
    }

    /// Gives the Span::ParentWeight value of the widget, if any
    fn span_weight(&self) -> Option<f32> {
        None
    }

    /// Widgets insert all characters (SymbolKeys) that they will need to render their Text, and
    /// propogates the call to their children. This is to ensure that the font system has the
    /// required characters rasterised and ready to render when text Presymbols are generated
    fn collect_text(&self, symbol_keys: &mut FxHashSet<SymbolKey>) {
        // Note: not collecting own text by default!

        for child in self.children().iter() {
            child.collect_text(symbol_keys);
        }
    }

    /// Returns the vertices necessary to render a widget, will append a new RenderLayer to
    /// render_layers to allow for a new clipping region when overflowing. This is not normally
    /// required by widgets that do not have nested children/do not have grand-children that may
    /// overflow
    fn to_vertices(
        &self,
        viewport_dimensions_px: PhysicalSize<u32>,
        render_layers: &mut VecDeque<RenderLayer>,
    ) -> (Vec<SimpleVertex>, Vec<TextVertex>) {
        (Vec::new(), Vec::new())
    }
}

pub enum BoundaryType {
    /// Indicates that the parent Widget will not flexibly extend if this boundary is overflown,
    /// causing clipping of the contents of the child Widget
    Static,
    /// Indicates that the parent Widget will flexibly extend in this direction if overflown
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
    /// The minimum dimensions of the contents of the Widget
    pub dimensions_px: Option<Dimensions<f32>>,
    
    /// The rectangle that has been given to the widget by its parents to be placed in
    pub clip_rectangle_px: Option<Rectangle<f32>>,
}

impl Layout {
    pub fn new() -> Self {
        Self {
            dimensions_px: None,
            clip_rectangle_px: None,
        }
    }
}