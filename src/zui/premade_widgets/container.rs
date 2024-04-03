use crate::{
    zui::{
        primitives::{Dimensions, Rectangle},
        renderers::{
            image_renderer::{ImageVertex, SpriteId},
            SimpleVertex, TextVertex,
        },
        widget::LayoutBoundaries,
        widget_store::{EntryChildren, EntryDefaultDescriptor},
        Axis, Colour, Context, Event, SpanConstraint, Widget,
    },
    PaddingWeights,
};

pub enum ContainerBackground {
    None,
    Colour(Colour),
    Image(SpriteId),
}

/// A widget that contains other widgets
pub struct Container {
    /// The name/tag of the Container, usually for debugging purposes
    pub name: Option<String>,

    /// The background of the Container, None is comletely transparent, a Colour will cause the
    /// container to render vertices of that Colour in the region of its Layout's clip Rectangle.
    pub background: ContainerBackground,
    // /// A test toggle that inverts the background colour when true
    // test_toggle: bool,
}

impl Container {
    pub fn new() -> Self {
        Self {
            name: None,
            background: ContainerBackground::None,
            // test_toggle: false,
        }
    }

    /// Used for diagnostic purposes
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(String::from(name));
        self
    }

    pub fn with_background(mut self, background: ContainerBackground) -> Self {
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
    Message: Clone,
{
    fn handle_event(
        &mut self,
        event: &Event,
        _region: &Rectangle<i32>,
        _context: &Context,
    ) -> Option<Message> {
        match event {
            // Event::MouseEvent(MouseEvent::CursorMoved) => {
            //     if let Some(cursor_position) = context.cursor_position {
            //         if region.is_in(&cursor_position) {
            //             self.test_toggle = true;
            //         } else {
            //             self.test_toggle = false;
            //         }
            //     }
            // }

            // Event::MouseEvent(MouseEvent::CursorExitedWindow) => {
            //     self.test_toggle = false;
            // }
            _ => {}
        }

        None
    }

    fn to_vertices(
        &self,
        region: Rectangle<i32>,
        context: &Context,
        simple_vertices: &mut Vec<SimpleVertex>,
        _text_vertices: &mut Vec<TextVertex>,
        image_vertices: &mut Vec<ImageVertex>,
    ) {
        match self.background {
            ContainerBackground::Colour(colour) => {
                simple_vertices.extend_from_slice(&SimpleVertex::from_rectangle(
                    region,
                    colour,
                    context.viewport_dimensions_px,
                ));
            }
            ContainerBackground::Image(sprite_id) => {
                image_vertices.extend_from_slice(&ImageVertex::from_rectangle_and_packed_sprite(
                    region,
                    context.viewport_dimensions_px,
                    context.image_texture_atlas.get(sprite_id),
                ));
            }
            ContainerBackground::None => {
                // do nothing
            }
        }
    }

    fn calculate_minimum_dimensions(
        &mut self,
        _layout_boundaries: &LayoutBoundaries,
        _context: &Context,
    ) -> Dimensions<i32> {
        Dimensions::new(0i32, 0i32)
    }

    fn entry_default_descriptor(&self) -> EntryDefaultDescriptor {
        EntryDefaultDescriptor {
            children: Some(EntryChildren::new(Axis::Vertical)),
            width_constraint: SpanConstraint::ParentWeight(1f32),
            height_constraint: SpanConstraint::ParentWeight(1f32),
            position_constraint: crate::PositionConstraint::ParentDetermined(PaddingWeights::NONE),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl<'a, Message> Into<Box<dyn Widget<Message> + 'a>> for Container
where
    Message: Clone + 'a,
{
    fn into(self) -> Box<dyn Widget<Message> + 'a> {
        Box::new(self)
    }
}
