use std::collections::VecDeque;

use super::{
    primitives::Rectangle, renderer::SimpleVertex, text_renderer::TextVertex, ScreenSpacePosition,
    Widget,
};

pub struct SceneHandle<Scene>
where
    Scene: super::Scene,
{
    root_widget: Widget<Scene::Message>,
    scene: Scene,
    widget_recreation_required: bool,
    messages: VecDeque<Scene::Message>,
}

impl<Scene> SceneHandle<Scene>
where
    Scene: super::Scene,
{
    pub fn new(scene: Scene, aspect_ratio: f32) -> Self {
        let mut this = Self {
            root_widget: scene.view(),
            scene,
            widget_recreation_required: false,
            messages: VecDeque::new(),
        };
        this.update_widget_rectangles(aspect_ratio);

        this
    }

    /// Handles external events and rebuilds widgets and rectangles if required
    pub fn update(&mut self, cursor_position: ScreenSpacePosition, aspect_ratio: f32) {
        self.solve_cursor_events(cursor_position);

        self.handle_messages();

        if self.widget_recreation_required {
            self.recreate_widgets();
            self.update_widget_rectangles(aspect_ratio);
            self.widget_recreation_required = false;
        }
    }

    /// Queues the recreation of widgets via calling view on the underlying scene
    pub fn queue_widget_recreation(&mut self) {
        self.widget_recreation_required = true;
    }

    /// Called when the widgets of a scene need to change/update
    fn recreate_widgets(&mut self) {
        self.root_widget = self.scene.view();
    }

    /// Called when the widgets of a scene's rectangles need to be recalculated
    fn update_widget_rectangles(&mut self, aspect_ratio: f32) {
        // setting the root widget's rectangle to the whole screen
        self.root_widget.set_rectangle(Some(Rectangle::new(
            glam::Vec2::new(-1f32, 1f32),
            glam::Vec2::new(2f32, 2f32),
        )));

        // updating the child rectangles
        self.root_widget
            .update_child_rectangles_recursively(aspect_ratio);

        // debug printing rectangles
        // self.root_widget.traverse(&mut |widget| {
        //     match widget.rectangle {
        //         Some(r) => println!("rect: {:?}", r),
        //         None => println!("rect: None"),
        //     };
        // });
    }

    /// Propagates through all widgets and adds to self.messages queue if widget contains an on_x
    /// message
    fn solve_cursor_events(&mut self, cursor_position: ScreenSpacePosition) {
        self.root_widget
            .update_cursor_events_recursively(cursor_position, &mut self.messages);
    }

    /// Iterates through the self.messages queue and passes messages to the underlying scene one by
    /// one
    fn handle_messages(&mut self) {
        while let Some(message) = self.messages.pop_front() {
            let rebuild_required = self.scene.handle_message(message);

            if rebuild_required {
                self.widget_recreation_required = true;
            }
        }
    }
}

impl<Scene> Renderable for SceneHandle<Scene>
where
    Scene: super::Scene,
{
    fn to_vertices(&self) -> (Vec<SimpleVertex>, Vec<TextVertex>) {
        let mut simple_vertices = Vec::new();
        let mut text_vertices = Vec::new();

        // simple rectangle vertices
        self.root_widget.traverse(&mut |widget| {
            let colour = match widget.background {
                Some(c) => c,
                None => return,
            };

            let rectangle = match widget.rectangle {
                Some(r) => r,
                None => return,
            };

            let a = SimpleVertex::new(rectangle.top_left, colour.into());
            let b = SimpleVertex::new(
                rectangle.top_left + glam::Vec2::new(rectangle.dimensions[0], 0f32),
                colour.into(),
            );
            let c = SimpleVertex::new(
                rectangle.top_left + glam::Vec2::new(0f32, -rectangle.dimensions[1]),
                colour.into(),
            );
            let d = SimpleVertex::new(
                rectangle.top_left
                    + glam::Vec2::new(rectangle.dimensions[0], -rectangle.dimensions[1]),
                colour.into(),
            );

            simple_vertices.push(a);
            simple_vertices.push(c);
            simple_vertices.push(b);

            simple_vertices.push(b);
            simple_vertices.push(c);
            simple_vertices.push(d);
        });

        // test text vertices
        let hspan = 0.5f32;
        let text_verts = [
            TextVertex::new(
                glam::Vec2::new(-hspan, hspan),
                glam::vec2(0f32, 0f32),
                glam::Vec4::new(1f32, 1f32, 1f32, 1f32),
            ),
            TextVertex::new(
                glam::Vec2::new(hspan, hspan),
                glam::vec2(1f32, 0f32),
                glam::Vec4::new(1f32, 1f32, 1f32, 1f32),
            ),
            TextVertex::new(
                glam::Vec2::new(-hspan, -hspan),
                glam::vec2(0f32, 1f32),
                glam::Vec4::new(1f32, 1f32, 1f32, 1f32),
            ),
            TextVertex::new(
                glam::Vec2::new(hspan, -hspan),
                glam::vec2(1f32, 1f32),
                glam::Vec4::new(1f32, 1f32, 1f32, 1f32),
            ),
        ];

        text_vertices.push(text_verts[0]);
        text_vertices.push(text_verts[2]);
        text_vertices.push(text_verts[1]);

        text_vertices.push(text_verts[1]);
        text_vertices.push(text_verts[2]);
        text_vertices.push(text_verts[3]);

        (simple_vertices, text_vertices)
    }
}

pub trait Renderable {
    fn to_vertices(
        &self,
    ) -> (
        Vec<super::renderer::SimpleVertex>,
        Vec<super::text_renderer::TextVertex>,
    );
}

pub trait Scene {
    type Message: Copy + Clone;

    /// Implements the update logic when a new message is received,
    /// Returns true if a rebuild (call of view function) is required
    fn handle_message(&mut self, message: Self::Message) -> bool;

    /// Returns the root widget of the scene
    fn view(&self) -> Widget<Self::Message>
    where
        <Self as Scene>::Message: std::marker::Copy;
}
