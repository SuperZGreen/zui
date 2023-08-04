use std::collections::VecDeque;

use crate::{
    zui::{
        primitives::{Dimensions, Rectangle},
        render_layer::RenderLayer,
        simple_renderer::SimpleVertex,
        text_renderer::TextVertex,
        widget::{DimensionsError, LayoutBoundaries},
        Axis, Colour, Context, Event, SpanConstraint, Widget, widget_store::{EntryDefaultDescriptor, EntryChildren},
    },
    MouseEvent, PaddingWeights,
};

pub struct Container {
    /// The axis that the children of the Container are placed along within the Container
    pub axis: Axis,

    // The name/tag of the Container, usually for debugging purposes
    pub name: Option<String>,

    /// The background of the Container, None is comletely transparent, a Colour will cause the
    /// container to render vertices of that Colour in the region of its Layout's clip Rectangle
    pub background: Option<Colour>,

    /// Flag that describes whether the container is overflowing or not
    pub overflowing: bool,

    /// A test toggle that inverts the background colour when true
    test_toggle: bool,
}

impl Container {
    pub fn new() -> Self {
        Self {
            name: None,
            axis: Axis::Vertical,
            background: None,
            overflowing: false,
            test_toggle: false,
        }
    }

    /// Used for diagnostic purposes
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(String::from(name));
        self
    }

    /// Sets the Axis along which the container's Widgets are laid out
    pub fn with_axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }

    pub fn with_background(mut self, background: Option<Colour>) -> Self {
        self.background = background;
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
}

impl<Message> Widget<Message> for Container
where
    Message: Clone + Copy,
{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn handle_event(&mut self, event: &Event, region: &Rectangle<i32>, context: &Context) {
        match event {
            // Event::MouseEvent(MouseEvent::ButtonReleased) => {
            //     self.test_toggle = !self.test_toggle;
            // }

            Event::MouseEvent(MouseEvent::CursorMoved) => {
                if let Some(cursor_position) = context.cursor_position {
                    if region.is_in(&cursor_position) {
                        self.test_toggle = true;
                    } else {
                        self.test_toggle = false;
                    }
                }
            }

            Event::MouseEvent(MouseEvent::CursorExitedWindow) => {
                self.test_toggle = false;
            }

            _ => {}
        }
    }

    fn to_vertices(
        &self,
        region: Rectangle<i32>,
        context: &Context,
        simple_vertices: &mut Vec<SimpleVertex>,
        _text_vertices: &mut Vec<TextVertex>,
        _render_layers: &mut VecDeque<RenderLayer>,
    ) {
        // adding own rectangle/simple vertices
        if let Some(base_colour) = self.background {
            let modified_colour = if self.test_toggle {
                Colour {
                    r: 1f32 - base_colour.r,
                    g: 1f32 - base_colour.g,
                    b: 1f32 - base_colour.b,
                    a: 1f32,
                }
            } else {
                base_colour
            };

            simple_vertices.extend_from_slice(&SimpleVertex::from_rectangle(
                region,
                modified_colour,
                context.viewport_dimensions_px,
            ));
        }
    }

    fn calculate_minimum_dimensions(
        &self,
        layout_boundaries: &LayoutBoundaries,
        width_contraint: SpanConstraint,
        height_contraint: SpanConstraint,
        _context: &Context,
    ) -> Result<Dimensions<i32>, DimensionsError> {
        let width_px = match width_contraint {
            SpanConstraint::Pixels(px) => px as i32,
            SpanConstraint::ParentRatio(pr) => {
                (layout_boundaries.horizontal.span_px as f32 * pr) as i32
            }
            SpanConstraint::FitContents => {
                return Err(DimensionsError::FitsChildren);
            }
            _ => 32i32,
        };

        let height_px = match height_contraint {
            SpanConstraint::Pixels(px) => px as i32,
            SpanConstraint::ParentRatio(pr) => {
                (layout_boundaries.vertical.span_px as f32 * pr) as i32
            }
            SpanConstraint::FitContents => {
                return Err(DimensionsError::FitsChildren);
            }
            _ => 32i32,
        };

        Ok(Dimensions {
            width: width_px,
            height: height_px,
        })
    }

    fn entry_default_descriptor(&self) -> EntryDefaultDescriptor {
        EntryDefaultDescriptor {
            children: Some(EntryChildren::new(Axis::Vertical)),
            width_constraint: SpanConstraint::ParentWeight(1f32),
            height_constraint: SpanConstraint::ParentWeight(1f32),
            position_constraint: crate::PositionConstraint::ParentDetermined(PaddingWeights::NONE),
        }
    }
}

impl<'a, Message> Into<Box<dyn Widget<Message> + 'a>> for Container
where
    Message: Clone + Copy + 'a,
{
    fn into(self) -> Box<dyn Widget<Message> + 'a> {
        Box::new(self)
    }
}
