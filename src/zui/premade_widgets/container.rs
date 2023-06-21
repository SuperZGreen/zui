use std::collections::VecDeque;

use winit::dpi::PhysicalSize;

use crate::zui::{
    primitives::{Dimensions, Rectangle},
    render_layer::RenderLayer,
    simple_renderer::SimpleVertex,
    text_renderer::TextVertex,
    widget::{Boundary, BoundaryType, EventResponse, Layout, LayoutBoundaries},
    Axis, Colour, Context, Event, Span, Text, Widget,
};

pub struct Container<Message>
where
    Message: Clone + Copy,
{
    children: Vec<Box<dyn Widget<Message>>>,

    // structure
    pub axis: Axis,
    pub span: Span,

    // internal callbacks
    pub callback_cursor_clicked: Option<fn(&mut Self) -> bool>,
    pub callback_cursor_on: Option<fn(&mut Self) -> bool>,
    pub callback_cursor_off: Option<fn(&mut Self) -> bool>,

    // message behaviour
    pub message_cursor_clicked: Option<Message>,
    pub message_cursor_on: Option<Message>,
    pub message_cursor_off: Option<Message>,

    /// style
    pub background: Option<Colour>,

    /// The number of pixels reserved for this Container's contents
    pub dimensions_px: Option<Dimensions<f32>>,

    /// calculated screen space area
    pub clip_rectangle: Option<Rectangle<f32>>,

    /// calculated screen space span
    pub span_px: Option<f32>,

    /// Flag that describes whether the container is overflowing or not
    pub overflowing: bool,

    /// Text contained within a widget's area
    pub text: Option<Text>,

    pub layout: Layout,
}

impl<Message> Container<Message>
where
    Message: Clone + Copy,
{
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            axis: Axis::Vertical,
            span: Span::ParentWeight(1f32),
            callback_cursor_clicked: None,
            callback_cursor_on: None,
            callback_cursor_off: None,
            message_cursor_clicked: None,
            message_cursor_on: None,
            message_cursor_off: None,
            background: None,
            dimensions_px: None,
            clip_rectangle: None,
            span_px: None,
            overflowing: false,
            text: None,
            layout: Layout::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }

    #[allow(dead_code)]
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }

    #[allow(dead_code)]
    pub fn with_background(mut self, background: Option<Colour>) -> Self {
        self.background = background;
        self
    }

    #[allow(dead_code)]
    pub fn with_callback_cursor_clicked(mut self, callback: Option<fn(&mut Self) -> bool>) -> Self {
        self.callback_cursor_clicked = callback;
        self
    }

    #[allow(dead_code)]
    pub fn with_callback_cursor_on(mut self, callback: Option<fn(&mut Self) -> bool>) -> Self {
        self.callback_cursor_on = callback;
        self
    }

    #[allow(dead_code)]
    pub fn with_callback_cursor_off(mut self, callback: Option<fn(&mut Self) -> bool>) -> Self {
        self.callback_cursor_off = callback;
        self
    }

    #[allow(dead_code)]
    pub fn with_message_clicked(mut self, message: Option<Message>) -> Self {
        self.message_cursor_clicked = message;
        self
    }

    #[allow(dead_code)]
    pub fn with_message_cursor_on(mut self, message: Option<Message>) -> Self {
        self.message_cursor_on = message;
        self
    }

    #[allow(dead_code)]
    pub fn with_message_cursor_off(mut self, message: Option<Message>) -> Self {
        self.message_cursor_off = message;
        self
    }

    #[allow(dead_code)]
    pub fn with_text(mut self, text: Text) -> Self {
        self.text = Some(text);
        self
    }

    #[allow(dead_code)]
    pub fn push(mut self, child: impl Into<Box<dyn Widget<Message>>>) -> Self {
        self.children.push(child.into());
        self
    }
}

impl<Message> Widget<Message> for Container<Message>
where
    Message: Clone + Copy,
{
    fn handle_event(&mut self, event: &Event, context: &Context) -> EventResponse<Message> {
        match event {
            Event::MouseEvent(_) => EventResponse::Propagate,
            _ => EventResponse::Propagate,
        }
    }

    fn layout<'a>(&'a self) -> &'a Layout {
        &self.layout
    }

    fn children(&self) -> &[Box<(dyn Widget<Message>)>] {
        &self.children
    }

    fn children_mut(&mut self) -> &mut [Box<(dyn Widget<Message>)>] {
        &mut self.children
    }

    fn collect_text(&self, symbol_keys: &mut rustc_hash::FxHashSet<crate::zui::font::SymbolKey>) {
        if let Some(text) = &self.text {
            text.collect_symbol_keys(symbol_keys);
        }

        for child in self.children.iter() {
            child.collect_text(symbol_keys);
        }
    }

    fn to_vertices(
        &self,
        viewport_dimensions_px: PhysicalSize<u32>,
        render_layers: &mut VecDeque<RenderLayer>,
    ) -> (Vec<SimpleVertex>, Vec<TextVertex>) {
        // adding own text vertices
        let mut text_vertices = Vec::new();
        if let Some(text) = &self.text {
            if let Some(clip_rectangle) = self.clip_rectangle {
                text_vertices.append(&mut text.to_vertices(clip_rectangle, viewport_dimensions_px));
            }
        }

        // adding own rectangle/simple vertices
        let mut simple_vertices = Vec::new();
        if let Some(rectangle) = self.clip_rectangle {
            if let Some(colour) = self.background {
                // simple_vertices.append(&mut rectangle.to_simple_vertices(colour));
                simple_vertices.extend_from_slice(&SimpleVertex::from_rectangle(
                    rectangle,
                    colour,
                    viewport_dimensions_px,
                ));
            }
        }

        // adding children's vertices
        for child in self.children.iter() {
            let (mut sv, mut tv) = child.to_vertices(viewport_dimensions_px, render_layers);
            simple_vertices.append(&mut sv);
            text_vertices.append(&mut tv);
        }

        // creating new layer if overflowing
        if self.overflowing {
            let render_layer =
                RenderLayer::new(simple_vertices, text_vertices, self.clip_rectangle);
            render_layers.push_back(render_layer);
            (Vec::new(), Vec::new())
        } else {
            (simple_vertices, text_vertices)
        }
    }

    fn span_weight(&self) -> Option<f32> {
        match self.span {
            Span::ParentWeight(weight) => Some(weight),
            _ => None,
        }
    }

    // TODO: this will always "fit contents"
    fn try_update_dimensions(
        &mut self,
        layout_boundaries: &LayoutBoundaries,
        context: &Context,
    ) -> Dimensions<f32> {
        // early exit guard
        match self.layout.dimensions_px {
            Some(dim) => return dim,
            None => {}
        }

        let dimensions = match self.span {
            Span::Pixels(p) => Dimensions::new(p, p),
            Span::FitContents => {
                // calculating the self dimensions based on the updated contents dimensions of the children
                // the span along the self.axis
                let mut self_axis_span = 0f32;

                // the span perpendicular to the self.axis
                let mut self_perpendicular_span = 0f32;

                for child in self.children.iter_mut() {
                    // calculating child dimensions
                    let child_dimensions = child.try_update_dimensions(layout_boundaries, context);

                    match self.axis {
                        Axis::Vertical => {
                            self_axis_span += child_dimensions.height;
                            if child_dimensions.width > self_perpendicular_span {
                                self_perpendicular_span = child_dimensions.width;
                            }
                        }
                        Axis::Horizontal => {
                            self_axis_span += child_dimensions.width;
                            if child_dimensions.height > self_perpendicular_span {
                                self_perpendicular_span = child_dimensions.height;
                            }
                        }
                    };
                }

                match self.axis {
                    Axis::Vertical => Dimensions::new(self_perpendicular_span, self_axis_span),
                    Axis::Horizontal => Dimensions::new(self_axis_span, self_perpendicular_span),
                }
            }
            _ => Dimensions::new(0f32, 0f32),
        };

        self.layout.dimensions_px = Some(dimensions);

        dimensions
    }

    fn try_fit_rectangle(&mut self, clip_rectangle: &Rectangle<f32>, context: &Context) {
        self.clip_rectangle = Some(*clip_rectangle);
    }

    fn place_children(&mut self, context: &Context) {
        if let Some(self_clip_rectangle) = self.clip_rectangle {
            let layout_boundaries = LayoutBoundaries::new(
                Boundary::new(BoundaryType::Static, self_clip_rectangle.width()),
                Boundary::new(BoundaryType::Static, self_clip_rectangle.height()),
            );

            let mut children_dimensions = Vec::new();
            for child in self.children.iter_mut() {
                children_dimensions.push(child.try_update_dimensions(&layout_boundaries, context));
            }

            let mut span_accumulator_px = 0f32;
            for (child, child_dimensions) in self.children.iter_mut().zip(children_dimensions) {
                let child_rect = match self.axis {
                    Axis::Vertical => {
                        let rect = Rectangle::new(
                            self_clip_rectangle.x_min,
                            self_clip_rectangle.x_min + child_dimensions.width,
                            self_clip_rectangle.y_max
                                - span_accumulator_px
                                - child_dimensions.height,
                            self_clip_rectangle.y_max - span_accumulator_px,
                        );
                        span_accumulator_px += child_dimensions.height;

                        rect
                    }
                    Axis::Horizontal => {
                        let rect = Rectangle::new(
                            self_clip_rectangle.x_min + span_accumulator_px,
                            self_clip_rectangle.x_min
                                + span_accumulator_px
                                + child_dimensions.width,
                            self_clip_rectangle.y_max - child_dimensions.height,
                            self_clip_rectangle.y_max,
                        );
                        span_accumulator_px += child_dimensions.width;

                        rect
                    }
                };

                child.try_fit_rectangle(&child_rect, context);
                child.place_children(context);
            }
        } else {
            warn!("no self.clip_rectangle!");
        }
    }
}

impl<'a, Message> Into<Box<dyn Widget<Message> + 'a>> for Container<Message>
where
    Message: Clone + Copy + 'a,
{
    fn into(self) -> Box<dyn Widget<Message> + 'a> {
        Box::new(self)
    }
}
