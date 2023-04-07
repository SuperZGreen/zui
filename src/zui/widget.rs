use super::Rectangle;

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
}

pub enum Span {
    //
    //  Absolute Sizes
    //
    /// Size as a portion of the view height, ie. the height of the application window surface is 1
    ViewWidth(f32),

    /// Size as a portion of the view width, ie. the width of the application window surface is 1
    ViewHeight(f32),

    /// Relative size with respect to the minimum dimension of the wgpu viewport
    ViewMin(f32),

    // TODO: ViewMax?

    //
    //  Flexible Sizes, dynamically resizes depending on the size of the parent widget
    //
    /// Weighted size with respect to the parent's size. Is summed up and divided amongst other
    /// child's sizes to determine the actual screen-space size of the widget
    ParentWeight(f32),

    /// The size as a proportion of the parent's size
    ParentRatio(f32),
}

impl Span {
    pub fn view_min_to_screen_space_span(
        view_min: f32,
        aspect_ratio: f32,
        parent_widget_axis: Axis,
    ) -> f32 {
        if aspect_ratio < 1f32 {
            Self::view_width_to_screen_space_span(view_min, aspect_ratio, parent_widget_axis)
        } else {
            Self::view_height_to_screen_space_span(view_min, aspect_ratio, parent_widget_axis)
        }
    }

    pub fn view_height_to_screen_space_span(
        view_height: f32,
        aspect_ratio: f32,
        parent_widget_axis: Axis,
    ) -> f32 {
        match parent_widget_axis {
            Axis::Vertical => view_height * 2f32,
            Axis::Horizontal => view_height / aspect_ratio * 2f32,
        }
    }

    pub fn view_width_to_screen_space_span(
        view_width: f32,
        aspect_ratio: f32,
        parent_widget_axis: Axis,
    ) -> f32 {
        match parent_widget_axis {
            Axis::Vertical => view_width * aspect_ratio * 2f32,
            Axis::Horizontal => view_width * 2f32,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Colour {
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1f32 }
    }

    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl From<Colour> for glam::Vec4 {
    fn from(colour: Colour) -> Self {
        Self::new(colour.r, colour.g, colour.b, colour.a)
    }
}

pub struct Widget {
    children: Vec<Widget>,

    // structure
    pub axis: Axis,
    pub span: Span,

    // style
    pub background: Option<Colour>,

    // calculated screen space area
    pub rectangle: Option<Rectangle>,
}

impl Widget {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            axis: Axis::Vertical,
            span: Span::ParentWeight(1f32),
            background: None,
            rectangle: None,
        }
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

    pub fn push(mut self, child: Widget) -> Self {
        self.children.push(child);
        self
    }

    pub fn set_rectangle(&mut self, rectangle: Option<Rectangle>) {
        self.rectangle = rectangle;
    }

    pub fn update_child_rectangles_recursively(&mut self, aspect_ratio: f32) {
        let self_rectangle = self.rectangle.unwrap();

        let axis_index = self.axis.to_index();

        let self_normalised_space_available = Self::get_parent_normalised_space_available(
            &self.children,
            self.axis,
            self_rectangle.dimensions[axis_index],
            aspect_ratio,
        );

        if self_normalised_space_available < 0.0f32 {
            warn!("overflowing: screen space span: {}", self_normalised_space_available);
        }

        let sum_of_child_span_weights = Self::get_sum_of_child_span_weights(&self.children);
        let mut top_left_accumulator = self_rectangle.top_left;

        for child in self.children.iter_mut() {
            let screen_space_span = match child.span {
                Span::ViewWidth(vw) => {
                    Span::view_width_to_screen_space_span(vw, aspect_ratio, self.axis)
                }
                Span::ViewHeight(vh) => {
                    Span::view_height_to_screen_space_span(vh, aspect_ratio, self.axis)
                }
                Span::ViewMin(vm) => {
                    Span::view_min_to_screen_space_span(vm, aspect_ratio, self.axis)
                }
                Span::ParentWeight(pw) => {
                    pw / sum_of_child_span_weights
                        * self_normalised_space_available
                        * self_rectangle.dimensions[axis_index]
                }
                Span::ParentRatio(pr) => pr * self_rectangle.dimensions[axis_index],
            };

            let mut dimensions = self_rectangle.dimensions;
            dimensions[axis_index] = screen_space_span;

            let child_rectangle = Rectangle {
                top_left: top_left_accumulator,
                dimensions,
            };

            child.set_rectangle(Some(child_rectangle));

            // updating accumulator
            match self.axis {
                Axis::Vertical => top_left_accumulator[1] -= screen_space_span,
                Axis::Horizontal => top_left_accumulator[0] += screen_space_span,
            }

            child.update_child_rectangles_recursively(aspect_ratio);
        }
    }

    // TODO: will need to change
    fn get_parent_normalised_space_available(
        children: &[Widget],
        parent_widget_axis: Axis,
        parent_screen_space_span: f32,
        aspect_ratio: f32,
    ) -> f32 {
        let mut space = 1f32;
        for child in children.iter() {
            let space_used = match child.span {
                Span::ViewHeight(vh) => {
                    Span::view_height_to_screen_space_span(vh, aspect_ratio, parent_widget_axis)
                        / parent_screen_space_span
                }
                Span::ViewWidth(vw) => {
                    Span::view_width_to_screen_space_span(vw, aspect_ratio, parent_widget_axis)
                        / parent_screen_space_span
                }
                Span::ViewMin(vm) => {
                    Span::view_min_to_screen_space_span(vm, aspect_ratio, parent_widget_axis)
                        / parent_screen_space_span
                }
                Span::ParentRatio(ratio) => ratio,
                Span::ParentWeight(_) => 0f32,
            };

            space -= space_used;
        }
        space
    }

    fn get_sum_of_child_span_weights(children: &[Widget]) -> f32 {
        let mut sum = 0f32;
        for child in children.iter() {
            match child.span {
                Span::ViewHeight(_) => {}
                Span::ViewWidth(_) => {}
                Span::ViewMin(_) => {}
                Span::ParentRatio(_) => {}
                Span::ParentWeight(weight) => sum += weight,
            }
        }
        sum
    }

    pub fn traverse<F>(&self, f: &mut F)
    where
        F: FnMut(&Widget),
    {
        f(self);

        for child in self.children.iter() {
            child.traverse(f);
        }
    }
}
