use std::collections::VecDeque;

use crate::zui::{
    primitives::{Dimensions, Rectangle},
    render_layer::RenderLayer,
    simple_renderer::SimpleVertex,
    text_renderer::TextVertex,
    widget::{Boundary, BoundaryType, EventResponse, Layout, LayoutBoundaries},
    Axis, Colour, Context, Event, Span, Widget,
};

pub struct Container<Message>
where
    Message: Clone + Copy,
{
    /// The children of the Container
    children: Vec<Box<dyn Widget<Message>>>,

    /// The axis that the children of the Container are placed along within the Container
    pub axis: Axis,

    /// The spatial span that the Container will try to take up
    pub span: Span,

    // The name/tag of the Container, usually for debugging purposes
    pub name: Option<String>,

    /// The background of the Container, None is comletely transparent, a Colour will cause the
    /// container to render vertices of that Colour in the region of its Layout's clip Rectangle
    pub background: Option<Colour>,

    /// Flag that describes whether the container is overflowing or not
    pub overflowing: bool,

    /// The Layout of the container, containing the clip rectangle and inner content dimensions
    pub layout: Layout,
}

impl<Message> Container<Message>
where
    Message: Clone + Copy,
{
    pub fn new() -> Self {
        Self {
            name: None,
            children: Vec::new(),
            axis: Axis::Vertical,
            span: Span::ParentWeight(1f32),
            background: None,
            overflowing: false,
            layout: Layout::new(),
        }
    }

    /// Used for diagnostic purposes
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(String::from(name));
        self
    }

    pub fn with_axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }

    pub fn with_background(mut self, background: Option<Colour>) -> Self {
        self.background = background;
        self
    }

    pub fn push(mut self, child: impl Into<Box<dyn Widget<Message>>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Gives the name of the widget as a string or "N/A"
    #[allow(dead_code)]
    fn name_as_str<'a>(name: &'a Option<String>) -> &'a str {
        match name {
            Some(name) => name.as_str(),
            None => "N/A",
        }
    }

    /// Increments a span_px accumulator by the child's rectangle
    fn update_span_accumulator_px(
        span_accumulator_px: &mut f32,
        axis: Axis,
        dimensions: &Dimensions<f32>,
    ) {
        match axis {
            Axis::Vertical => *span_accumulator_px += dimensions.height,
            Axis::Horizontal => *span_accumulator_px += dimensions.width,
        }
    }

    /// Gives the number of span pixels used by children's layout minimum dimensions
    fn calculate_reserved_span_px(&self) -> f32 {
        let mut span_accumulator_px = 0f32;
        for child in self.children.iter() {
            // getting the child's dimensions_px
            let child_dimensions = match child.layout().dimensions_px {
                Some(dp) => dp,
                None => continue,
            };

            // updating the accumulator
            Self::update_span_accumulator_px(
                &mut span_accumulator_px,
                self.axis,
                &child_dimensions.into(),
            );
        }

        span_accumulator_px
    }

    // Gives the sum of children's Span::ParentWeight values
    fn sum_of_child_span_parent_weights(&self) -> f32 {
        let mut sum = 0f32;
        for child in self.children.iter() {
            match child.span_weight() {
                Some(w) => sum += w,
                None => {}
            };
        }

        sum
    }

    /// Calculates the dimensions of a child based on the dimensions of a parent, essentially
    /// cutting a dimensions/rectangle into smaller parts
    fn calculate_child_dimensions(
        self_dimensions: Dimensions<f32>,
        axis: Axis,
        span_px: f32,
    ) -> Dimensions<f32> {
        match axis {
            Axis::Vertical => Dimensions::new(self_dimensions.width, span_px),
            Axis::Horizontal => Dimensions::new(span_px, self_dimensions.height),
        }
    }
}

impl<Message> Widget<Message> for Container<Message>
where
    Message: Clone + Copy,
{
    fn handle_event(
        &mut self,
        event: &Event,
        _context: &Context,
    ) -> EventResponse<Message> {
        match event {
            Event::MouseEvent(_) => EventResponse::Propagate,
            // _ => EventResponse::Propagate,
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

    fn to_vertices(
        &self,
        context: &Context,
        simple_vertices: &mut Vec<SimpleVertex>,
        text_vertices: &mut Vec<TextVertex>,
        render_layers: &mut VecDeque<RenderLayer>,
    ) {
        // adding own rectangle/simple vertices
        if let Some(rectangle) = self.layout.clip_rectangle_px {
            if let Some(colour) = self.background {
                simple_vertices.extend_from_slice(&SimpleVertex::from_rectangle(
                    rectangle,
                    colour,
                    context.viewport_dimensions_px,
                ));
            }
        }

        // adding children's vertices
        if self.overflowing {
            // creating new layer if overflowing
            let mut simple_vertices = Vec::new();
            let mut text_vertices = Vec::new();

            for child in self.children.iter() {
                child.to_vertices(
                    context,
                    &mut simple_vertices,
                    &mut text_vertices,
                    render_layers,
                );
            }

            let render_layer = RenderLayer::new(
                simple_vertices,
                text_vertices,
                self.layout.clip_rectangle_px,
            )
            .with_name(self.name.as_ref().map(|n| n.as_str()));

            render_layers.push_front(render_layer);
        } else {
            for child in self.children.iter() {
                child.to_vertices(context, simple_vertices, text_vertices, render_layers);
            }
        }
    }

    fn span_weight(&self) -> Option<f32> {
        match self.span {
            Span::ParentWeight(weight) => Some(weight),
            _ => None,
        }
    }

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
            Span::Pixels(p) => Some(Dimensions::new(p * 2f32, p)),
            Span::FitContents => {
                // calculating the self dimensions based on the updated contents dimensions of the
                // children the span along the self.axis
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
                    Axis::Vertical => {
                        Some(Dimensions::new(self_perpendicular_span, self_axis_span))
                    }
                    Axis::Horizontal => {
                        Some(Dimensions::new(self_axis_span, self_perpendicular_span))
                    }
                }
            }
            Span::ParentRatio(pr) => {
                let available_width_px = layout_boundaries.horizontal.span_px;
                let available_height_px = layout_boundaries.vertical.span_px;

                match self.axis {
                    Axis::Horizontal => Some(Dimensions::new(
                        available_width_px * pr,
                        available_height_px,
                    )),
                    Axis::Vertical => Some(Dimensions::new(
                        available_width_px,
                        available_height_px * pr,
                    )),
                }
            }
            _ => None,
        };

        self.layout.dimensions_px = dimensions;

        dimensions.unwrap_or(Dimensions::new(0f32, 0f32))
    }

    fn try_fit_rectangle(&mut self, clip_rectangle: &Rectangle<f32>, _context: &Context) {
        self.layout.clip_rectangle_px = Some(*clip_rectangle);

        // // this will be called when the Span::ParentWeights are used
        // if self.layout.dimensions_px.is_none {
        //     self.try_update_dimensions
        // }
    }

    fn place_children(&mut self, context: &Context) {
        if self.children.is_empty() {
            return;
        }

        if let Some(self_clip_rectangle) = self.layout.clip_rectangle_px {
            // name for diagnostics later
            // let self_name_str = Self::name_as_str(&self.name);

            // determining the LayoutBoundaries for the children
            let layout_boundaries = LayoutBoundaries::new(
                Boundary::new(BoundaryType::Static, self_clip_rectangle.width()),
                Boundary::new(BoundaryType::Static, self_clip_rectangle.height()),
            );

            // trying to update all children's reserved dimensions
            for child in self.children.iter_mut() {
                child.try_update_dimensions(&layout_boundaries, context);
            }

            // calculating the total span used by children
            let reserved_px = self.calculate_reserved_span_px();
            let free_px = self_clip_rectangle.span_by_axis(self.axis) - reserved_px;

            // calculating the number of pixels per Span::ParentWeight(1f32)
            let sum_of_child_weights = self.sum_of_child_span_parent_weights();

            let pixels_per_parent_weight = if free_px <= 0f32 {
                0f32
            } else {
                free_px / sum_of_child_weights
            };

            // info!("container {self_name_str}, free_px: {free_px}, reserved_px: {reserved_px}, sum_of_child_weights: {sum_of_child_weights}, pixels_per_parent_weight: {pixels_per_parent_weight}");

            // flagging overflow
            if free_px < 0f32 {
                // info!("container: {} is overflowing!", self_name_str);
                self.overflowing = true;
            }

            // force updating the dimensions of weighted children
            for child in self.children.iter_mut() {
                match child.span_weight() {
                    Some(pw) => {
                        let span_px = pw * pixels_per_parent_weight;

                        let child_dimensions = Self::calculate_child_dimensions(
                            self_clip_rectangle.into(),
                            self.axis,
                            span_px,
                        );

                        child.set_dimensions(Some(child_dimensions));

                        // info!("container {self_name_str} child_dimensions: {child_dimensions:?}");
                    }
                    _ => {}
                };
            }

            // placing the children
            let mut span_accumulator_px = 0f32;
            for child in self.children.iter_mut() {
                // getting the child dimensions if they have been generated, otherwise continuing
                let child_dimensions = match child.layout().dimensions_px {
                    Some(dims) => dims,
                    None => continue,
                };

                // determining the child rectangle
                let child_rectangle = match self.axis {
                    Axis::Vertical => Rectangle::new(
                        self_clip_rectangle.x_min,
                        self_clip_rectangle.x_min + child_dimensions.width,
                        self_clip_rectangle.y_max - span_accumulator_px - child_dimensions.height,
                        self_clip_rectangle.y_max - span_accumulator_px,
                    ),
                    Axis::Horizontal => Rectangle::new(
                        self_clip_rectangle.x_min + span_accumulator_px,
                        self_clip_rectangle.x_min + span_accumulator_px + child_dimensions.width,
                        self_clip_rectangle.y_max - child_dimensions.height,
                        self_clip_rectangle.y_max,
                    ),
                };

                child.try_fit_rectangle(&child_rectangle, context);
                child.place_children(context);

                // increments the span accumulator
                Self::update_span_accumulator_px(
                    &mut span_accumulator_px,
                    self.axis,
                    &child_rectangle.into(),
                );
            }
        } else {
            warn!("no self.clip_rectangle!");
        }
    }

    fn set_dimensions(&mut self, dimensions_px: Option<Dimensions<f32>>) {
        self.layout.dimensions_px = dimensions_px;
    }
}

impl<'a, Message> Into<Box<dyn Widget<Message> + 'a>>
    for Container<Message>
where
    Message: Clone + Copy + 'a,
{
    fn into(self) -> Box<dyn Widget<Message> + 'a> {
        Box::new(self)
    }
}
