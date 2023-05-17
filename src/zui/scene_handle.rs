use std::collections::VecDeque;

use super::{
    primitives::Rectangle, renderer::SimpleVertex, text_renderer::TextVertex, Colour, CursorState,
    Font, Renderable, Scene, Widget,
};

/// Allows for caching of the widgets produced by Scene::view
pub struct SceneHandle<Message>
where
    Message: Clone + Copy,
{
    // the root widget produced by the scene
    root_widget: Option<Widget<Message>>,

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
        self.solve_cursor_events(cursor_state);

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

        // updating widget rectangles
        self.update_widget_rectangles(aspect_ratio);

        // updating widget text
        self.update_text_symbols(font, aspect_ratio);

        self.widget_recreation_required = false;
    }

    /// Recalculates all the widget rectangles/regions in a scene
    fn update_widget_rectangles(&mut self, aspect_ratio: f32) {
        if let Some(root_widget) = &mut self.root_widget {
            // setting the root widget's rectangle to the whole screen
            root_widget.set_rectangle(Some(Rectangle::new(-1f32, 1f32, -1f32, 1f32)));

            // updating the child rectangles
            root_widget.update_child_rectangles_recursively(aspect_ratio);
        }
    }

    /// Updates the text symbols (and rectangles) for a scene
    fn update_text_symbols(&mut self, font: &Font, aspect_ratio: f32) {
        if let Some(root_widget) = &mut self.root_widget {
            root_widget.update_text_symbols_recursively(font, aspect_ratio);
        }
    }

    /// Propagates through all widgets and adds to self.messages queue if widget contains an on_x
    /// message
    fn solve_cursor_events(&mut self, cursor_state: &Option<CursorState>) {
        if let Some(root_widget) = &mut self.root_widget {
            root_widget.update_cursor_events_recursively(cursor_state, &mut self.messages);
        }
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
        let mut simple_vertices = Vec::new();
        let mut text_vertices = Vec::new();

        let root_widget = match &self.root_widget {
            Some(rw) => rw,
            None => {
                warn!("attempting to render SceneHandle with empty root widget");
                return (Vec::new(), Vec::new());
            }
        };

        // simple rectangle vertices
        root_widget.traverse(&mut |widget| {
            let rectangle = match widget.rectangle {
                Some(r) => r,
                None => return,
            };

            // adding the background box vertices if it contains a colour/background setting
            if let Some(colour) = widget.background {
                let rectangle_vertices = rectangle.vertices();

                let a = SimpleVertex::new(rectangle_vertices[0], colour.into());
                let b = SimpleVertex::new(rectangle_vertices[1], colour.into());
                let c = SimpleVertex::new(rectangle_vertices[2], colour.into());
                let d = SimpleVertex::new(rectangle_vertices[3], colour.into());

                simple_vertices.push(a);
                simple_vertices.push(c);
                simple_vertices.push(b);

                simple_vertices.push(b);
                simple_vertices.push(c);
                simple_vertices.push(d);
            }

            // adding text vertices if text exists
            if let Some(text) = &widget.text {
                text_vertices.append(&mut text.to_vertices(rectangle, viewport_dimensions_px));
            }
        });

        (simple_vertices, text_vertices)
    }
}
