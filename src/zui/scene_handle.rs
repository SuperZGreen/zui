use std::collections::VecDeque;

use winit::dpi::PhysicalSize;

use super::{
    primitives::Rectangle,
    render_layer::RenderLayer,
    widget::{Boundary, BoundaryType, Event, EventResponse, LayoutBoundaries, Widget},
    Context, Renderable, Scene,
};

/// Allows for caching of the widgets produced by Scene::view
pub struct SceneHandle<Message>
where
    Message: Clone + Copy,
{
    // the root widget produced by the scene
    root_widget: Option<Box<dyn Widget<Message>>>,

    // the scene implemented by the user
    scene: Box<dyn Scene<Message = Message>>,

    // flag checked in update, to rebuild the scene's widgets and rectangles
    widget_recreation_required: bool,

    // messages produced by the scene widgets
    messages: VecDeque<Message>,

    // messages to be processed by the user developer
    external_messages: VecDeque<Message>,
}

impl<Message> SceneHandle<Message>
where
    Message: Clone + Copy,
{
    pub fn new(scene: Box<dyn Scene<Message = Message>>) -> Self {
        Self {
            root_widget: None,
            scene,
            widget_recreation_required: true,
            messages: VecDeque::new(),
            external_messages: VecDeque::new(),
        }
    }

    /// Handles external events and rebuilds widgets and rectangles if required
    pub fn update(&mut self, context: &Context) {
        // self.solve_cursor_events(cursor_state);

        self.handle_messages();

        // lazy widget recreation
        if self.widget_recreation_required {
            self.rebuild_scene(context);
        }
    }

    /// Queues the recreation of widgets via calling view on the underlying scene
    pub fn queue_widget_recreation(&mut self) {
        self.widget_recreation_required = true;
    }

    /// Rebuilds the scene using Scene::view, refits the Widgets to the Normalised Device
    /// Coordinates square
    pub fn rebuild_scene(&mut self, context: &Context) {
        // recreating widgets
        self.root_widget = Some(self.scene.view(context.aspect_ratio));

        // getting the root widget
        let root_widget = self.root_widget.as_mut().unwrap();

        // creating the layout boundaries to pass to the root widget
        let layout_boundaries = &LayoutBoundaries::new(
            Boundary::new(
                BoundaryType::Static,
                context.viewport_dimensions_px.width as f32,
            ),
            Boundary::new(
                BoundaryType::Static,
                context.viewport_dimensions_px.height as f32,
            ),
        );

        // updating the root widget's dimensions
        let dims = root_widget.try_update_dimensions(layout_boundaries, context);
        let clip_rectangle = Rectangle::new(
            0f32,
            dims.width,
            context.viewport_dimensions_px.height as f32 - dims.height,
            context.viewport_dimensions_px.height as f32,
        );
        root_widget.try_fit_rectangle(&clip_rectangle, context);

        root_widget.place_children(context);

        self.widget_recreation_required = false;
    }

    /// Queues resizing the root widget
    pub fn resize_scene(&mut self, context: &Context) {
        // TODO: resize_scene might be defunct?
        self.rebuild_scene(context);

        // if let Some(root_widget) = &mut self.root_widget {
        //     // root_widget.handle_event(
        //     //     &mut Event::FitRectangle(Rectangle::new(
        //     //         0f32,
        //     //         context.viewport_dimensions_px.width as f32,
        //     //         0f32,
        //     //         context.viewport_dimensions_px.height as f32,
        //     //     )),
        //     //     &context,
        //     // );
        //     re
        // } else {
        //     warn!("no root widget to resize!");
        // }
    }

    /// Iterates through the self.messages queue and passes messages to the underlying scene one by
    /// one
    fn handle_messages(&mut self) {
        while let Some(message) = self.messages.pop_front() {
            let (external_message_opt, rebuild_required) = self.scene.handle_message(message);

            if let Some(external_message) = external_message_opt {
                self.external_messages.push_back(external_message);
            }

            if rebuild_required {
                self.widget_recreation_required = true;
            }
        }
    }

    /// Passes a certain event to the scene to be propageed through the widget tree
    pub fn handle_event(&mut self, mut event: Event, context: &Context) {
        if let Some(root_widget) = &mut self.root_widget {
            Self::handle_event_recursively(root_widget, &mut event, context, &mut self.messages);
        }
    }

    /// Handles widget events recursively
    pub fn handle_event_recursively(
        // The Widget to handle the event
        widget: &mut Box<dyn Widget<Message>>,

        // The event for the Widget to handle
        event: &mut Event,

        // The event Context (mouse position, aspect ratio, fonts, etc)
        context: &Context,

        // The message as a result of the Widget handling the event
        result_messages: &mut VecDeque<Message>,
    ) {
        let event_response = widget.handle_event(event, context);

        match event_response {
            EventResponse::Consumed => {} // Do nothing
            EventResponse::Message(message) => {
                result_messages.push_back(message);
            }
            EventResponse::Propagate => {
                for child in widget.children_mut().iter_mut() {
                    Self::handle_event_recursively(child, event, context, result_messages);
                }
            }
        }
    }

    /// Pops an external message from the queue
    pub fn pop_external_message(&mut self) -> Option<Message> {
        self.external_messages.pop_front()
    }
}

impl<Message> Renderable for SceneHandle<Message>
where
    Message: Clone + Copy,
{
    fn to_render_layers(&self, viewport_dimensions_px: PhysicalSize<u32>) -> VecDeque<RenderLayer> {
        let root_widget = match &self.root_widget {
            Some(rw) => rw,
            None => {
                warn!("attempting to render SceneHandle with empty root widget");
                return VecDeque::new();
            }
        };

        let mut render_layers = VecDeque::new();
        let (simple_vertices, text_vertices) =
            root_widget.to_vertices(viewport_dimensions_px, &mut render_layers);

        render_layers.push_front(RenderLayer::new(simple_vertices, text_vertices, None));

        render_layers
    }
}
