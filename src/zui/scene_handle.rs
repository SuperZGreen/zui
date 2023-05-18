use std::collections::VecDeque;

use super::{
    primitives::Rectangle,
    renderer::SimpleVertex,
    text_renderer::TextVertex,
    widget::{Event, EventResponse, ResizingInformation, Widget},
    BaseWidget, Colour, CursorState, Font, Renderable, Scene,
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
    pub fn update(&mut self, cursor_state: &Option<CursorState>, font: &Font, aspect_ratio: f32) {
        // self.solve_cursor_events(cursor_state);

        self.handle_messages();

        // lazy widget recreation
        if self.widget_recreation_required {
            self.rebuild_scene(font, aspect_ratio);
        }
    }

    /// Queues the recreation of widgets via calling view on the underlying scene
    pub fn queue_widget_recreation(&mut self) {
        self.widget_recreation_required = true;
    }

    /// Recreates widgets, updates rectangles and text
    pub fn rebuild_scene(&mut self, font: &Font, aspect_ratio: f32) {
        // recreating widgets
        self.root_widget = Some(self.scene.view(aspect_ratio));

        let resizing_information = ResizingInformation { font, aspect_ratio };

        // updating widget rectangles
        // self.resize_widgets(resizing_information);
        self.root_widget
            .as_mut()
            .unwrap()
            .handle_event(&Event::FitRectangle((
                Rectangle::new(-1f32, 1f32, -1f32, 1f32),
                &resizing_information,
            )));

        // // updating widget text
        // self.update_text_symbols(font, aspect_ratio);

        self.widget_recreation_required = false;
    }

    // /// Updates the text symbols (and rectangles) for a scene
    // fn update_text_symbols(&mut self, font: &Font, aspect_ratio: f32) {
    //     if let Some(root_widget) = &mut self.root_widget {
    //         root_widget.update_text_symbols_recursively(font, aspect_ratio);
    //     }
    // }

    // /// Propagates through all widgets and adds to self.messages queue if widget contains an on_x
    // /// message
    // fn solve_cursor_events(&mut self, cursor_state: &Option<CursorState>) {
    //     if let Some(root_widget) = &mut self.root_widget {
    //         root_widget.update_cursor_events_recursively(cursor_state, &mut self.messages);
    //     }
    // }

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
    pub fn handle_event(&mut self, event: Event) {
        if let Some(root_widget) = &mut self.root_widget {
            match Self::handle_event_recursively(root_widget, &event) {
                Some(message) => self.messages.push_back(message),
                None => {}, // Do nothing
            }
        }
    }

    /// Handles widget events recursively
    pub fn handle_event_recursively(
        widget: &mut Box<dyn Widget<Message>>,
        event: &Event,
    ) -> Option<Message> {
        let event_response = widget.handle_event(event);

        match event_response {
            EventResponse::Consumed => {}, // Do nothing
            EventResponse::Message(message) => {
                return Some(message);
            }
            EventResponse::Propagate => {
                if let Some(children_iter_mut) = widget.children_iter_mut() {
                    for child in children_iter_mut {
                        if let Some(message) =  Self::handle_event_recursively(child, event) {
                            return Some(message);
                        }
                    }
                }
            }
        }

        None
    }

    /// Pops an external message from the queue
    pub fn pop_external_message(&mut self) -> Option<Message> {
        self.external_messages.pop_front()
    }

    // /// Gives a mut reference to the underlying scene
    // pub fn scene_mut(&mut self) -> &mut Scene {
    //     &mut self.scene
    // }
}

impl<Message> Renderable for SceneHandle<Message>
where
    Message: Clone + Copy,
{
    fn to_vertices(
        &self,
        viewport_dimensions_px: glam::Vec2,
    ) -> (Vec<SimpleVertex>, Vec<TextVertex>) {
        // let mut simple_vertices = Vec::new();
        // let mut text_vertices = Vec::new();

        let root_widget = match &self.root_widget {
            Some(rw) => rw,
            None => {
                warn!("attempting to render SceneHandle with empty root widget");
                return (Vec::new(), Vec::new());
            }
        };

        // simple rectangle vertices
        // root_widget.traverse(&mut |widget| {
        //     let (mut sv, mut tv) = widget.to_vertices(viewport_dimensions_px);
        //     simple_vertices.append(&mut sv);
        //     text_vertices.append(&mut tv);
        // });

        let (simple_vertices, text_vertices) = root_widget.to_vertices(viewport_dimensions_px);

        (simple_vertices, text_vertices)
    }
}
