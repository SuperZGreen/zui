use std::collections::VecDeque;

use rustc_hash::FxHashSet;

use crate::StateStore;

use super::{
    primitives::Rectangle,
    render_layer::RenderLayer,
    widget::{Boundary, BoundaryType, Event, EventResponse, LayoutBoundaries, Widget},
    Context, ContextMutTypeface, Renderable, Scene, stopwatch::Stopwatch,
};

/// Allows for caching of the widgets produced by Scene::view
pub struct SceneHandle<Message, StateIdentifier>
where
    Message: Clone + Copy,
    StateIdentifier: std::hash::Hash + Eq + std::fmt::Debug,
{
    /// the root widget produced by the scene
    root_widget: Option<Box<dyn Widget<Message, StateIdentifier>>>,

    /// the scene implemented by the user
    scene: Box<dyn Scene<Message = Message, StateIdentifier = StateIdentifier>>,

    /// flag checked in update, to rebuild the scene's widgets and rectangles
    widget_recreation_required: bool,

    /// messages produced by the scene widgets
    messages: VecDeque<Message>,

    /// messages to be processed by the user developer
    external_messages: VecDeque<Message>,

    /// Holds the state for the scene's widgets to persist between 'rebuild_scene' calls
    state_store: StateStore<StateIdentifier>,
}

impl<Message, StateIdentifier> SceneHandle<Message, StateIdentifier>
where
    Message: Clone + Copy,
    StateIdentifier: std::hash::Hash + Eq + std::fmt::Debug,
{
    pub fn new(
        scene: Box<dyn Scene<Message = Message, StateIdentifier = StateIdentifier>>,
    ) -> Self {
        Self {
            root_widget: None,
            scene,
            widget_recreation_required: true,
            messages: VecDeque::new(),
            external_messages: VecDeque::new(),
            state_store: StateStore::new(),
        }
    }

    /// Allows passing a message to the Scene externally, to be dealt with by the UI
    pub fn handle_message(&mut self, message: Message) -> Option<Message> {
        let (message, rebuild_requested) = self.scene.handle_message(message);

        if rebuild_requested {
            self.widget_recreation_required = true;
        }

        message
    }

    /// Handles external events and rebuilds widgets and rectangles if required
    pub fn update(
        &mut self,
        context_mut_typeface: &mut ContextMutTypeface,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        self.handle_messages();

        // lazy widget recreation
        if self.widget_recreation_required {
            // IMPORTANT: prevents scene from rebuilding if viewport is zero, as this causes a
            // severe memory leak
            let viewport_is_zero = context_mut_typeface.viewport_dimensions_px.width == 0
                || context_mut_typeface.viewport_dimensions_px.height == 0;
            if !viewport_is_zero {
                self.rebuild_scene(context_mut_typeface, device, queue);
            }
        }
    }

    /// Queues the recreation of widgets via calling view on the underlying scene
    pub fn queue_rebuild_scene(&mut self) {
        self.widget_recreation_required = true;
    }

    /// Rebuilds the scene using Scene::view, refits the Widgets to the Normalised Device
    /// Coordinates square
    fn rebuild_scene(
        &mut self,
        context_mut_typeface: &mut ContextMutTypeface,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        // recreating widgets
        self.root_widget = Some(
            self.scene
                .view(&mut self.state_store, context_mut_typeface.aspect_ratio),
        );

        // getting the root widget
        let root_widget = self.root_widget.as_mut().unwrap();

        let rasterisation_stopwatch = Stopwatch::start();
        // collecting text symbols for rasterisation
        let mut symbol_keys = FxHashSet::default();
        root_widget.collect_text(&mut symbol_keys);

        // // debug printing collected symbol keys
        // for symbol_key in symbol_keys.iter() {
        //     info!("symbol_key: {symbol_key:?}");
        // }
        // info!("");

        // rasterising the collected symbols and preparing the text atlas etc
        context_mut_typeface
            .typeface
            .rasterise_symbol_keys(symbol_keys, device, queue);
        // trace!("text rasterisation took: {:.2}ms", rasterisation_stopwatch.elapsed() * 1000f32);

        // creating the layout boundaries to pass to the root widget
        let layout_boundaries = &LayoutBoundaries::new(
            Boundary::new(
                BoundaryType::Static,
                context_mut_typeface.viewport_dimensions_px.width as f32,
            ),
            Boundary::new(
                BoundaryType::Static,
                context_mut_typeface.viewport_dimensions_px.height as f32,
            ),
        );

        // creating the context
        let context = Context {
            typeface: &*context_mut_typeface.typeface,
            aspect_ratio: context_mut_typeface.aspect_ratio,
            cursor_position: context_mut_typeface.cursor_position,
            viewport_dimensions_px: context_mut_typeface.viewport_dimensions_px,
        };

        // updating the root widget's dimensions
        let dims = root_widget.try_update_dimensions(layout_boundaries, &context);
        let clip_rectangle = Rectangle::new(
            0f32,
            dims.width,
            context.viewport_dimensions_px.height as f32 - dims.height,
            context.viewport_dimensions_px.height as f32,
        );
        root_widget.try_fit_rectangle(&clip_rectangle, &context);

        root_widget.place_children(&context);

        self.widget_recreation_required = false;
    }

    // /// Queues resizing the root widget
    // pub fn resize_scene(
    //     &mut self,
    //     context_mut_typeface: &mut ContextMutTypeface,
    //     device: &wgpu::Device,
    //     queue: &wgpu::Queue,
    // ) {
    //     // TODO: resize_scene might be defunct?
    //     self.rebuild_scene(context_mut_typeface, device, queue);
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
    pub fn handle_event(&mut self, mut event: Event, context: &Context) {
        if let Some(root_widget) = &mut self.root_widget {
            Self::handle_event_recursively(
                root_widget,
                &mut event,
                context,
                &mut self.state_store,
                &mut self.messages,
            );
        }
    }

    /// Handles widget events recursively
    pub fn handle_event_recursively(
        // The Widget to handle the event
        widget: &mut Box<dyn Widget<Message, StateIdentifier>>,

        // The event for the Widget to handle
        event: &mut Event,

        // The event Context (mouse position, aspect ratio, fonts, etc)
        context: &Context,

        // the SceneHandle's state store
        state_store: &mut StateStore<StateIdentifier>,

        // The message as a result of the Widget handling the event
        result_messages: &mut VecDeque<Message>,
    ) {
        let event_response = widget.handle_event(event, context, state_store);

        match event_response {
            EventResponse::Consumed => {} // Do nothing
            EventResponse::Message(message) => {
                result_messages.push_back(message);
            }
            EventResponse::Propagate => {
                for child in widget.children_mut().iter_mut() {
                    Self::handle_event_recursively(
                        child,
                        event,
                        context,
                        state_store,
                        result_messages,
                    );
                }
            }
        }
    }

    /// Pops an external message from the queue
    pub fn pop_external_message(&mut self) -> Option<Message> {
        self.external_messages.pop_front()
    }
}

impl<Message, StateIdentifier> Renderable for SceneHandle<Message, StateIdentifier>
where
    Message: Clone + Copy,
    StateIdentifier: std::hash::Hash + Eq + std::fmt::Debug,
{
    fn to_render_layers(&self, context: &Context) -> VecDeque<RenderLayer> {
        let root_widget = match &self.root_widget {
            Some(rw) => rw,
            None => {
                warn!("attempting to render SceneHandle with empty root widget");
                return VecDeque::new();
            }
        };

        let mut render_layers = VecDeque::new();
        let mut simple_vertices = Vec::new();
        let mut text_vertices = Vec::new();

        root_widget.to_vertices(
            context,
            &self.state_store,
            &mut simple_vertices,
            &mut text_vertices,
            &mut render_layers,
        );

        render_layers.push_front(
            RenderLayer::new(simple_vertices, text_vertices, None).with_name(Some("root_layer")),
        );

        // for (render_layer_index, render_layer) in render_layers.iter().enumerate() {
        //     info!(
        //         "render_layer[{render_layer_index}] sv.len: {}, tv.len: {}, clip_rect: {:?}",
        //         render_layer.simple_vertices.len(),
        //         render_layer.text_vertices.len(),
        //         render_layer.clip_rectangle,
        //     );
        // }
        // info!("");

        render_layers
    }
}
