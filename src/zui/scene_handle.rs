use std::collections::VecDeque;

use crate::{ContextMutTypeface, WidgetStore};

use super::{
    render_layer::RenderLayer, widget::Event, widget_store::WidgetId, Context, Renderable, Scene,
};

pub struct SceneHandle<Message>
where
    Message: Clone,
{
    /// the root widget produced by the scene
    root_widget_id: WidgetId,

    /// the scene implemented by the user
    scene: Box<dyn Scene<Message = Message>>,

    /// messages produced by the scene widgets
    messages: VecDeque<Message>,

    /// messages to be processed by the user developer
    external_messages: VecDeque<Message>,

    /// The persistence/container for Widgets in the underlying scene
    widget_store: WidgetStore<Message>,
}

impl<Message> SceneHandle<Message>
where
    Message: Clone,
{
    pub fn new(mut scene: Box<dyn Scene<Message = Message>>) -> Self {
        let mut widget_store = WidgetStore::new();
        let root_widget_id = scene.init(&mut widget_store);

        Self {
            root_widget_id,
            scene,
            messages: VecDeque::new(),
            external_messages: VecDeque::new(),
            widget_store,
        }
    }

    /// Clears the Widget layout information in the scene, and recalculates Layout info
    pub fn update(
        &mut self,
        context: &mut ContextMutTypeface,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        // handling all messages
        for message in self.messages.drain(..) {
            let (external_message, _) = self.scene.handle_message(&mut self.widget_store, message);

            if let Some(external_message) = external_message {
                self.external_messages.push_back(external_message);
            }
        }

        // early exit, not bothering to calculate layouts etc when minimised
        if !context.viewport_dimensions_px.has_non_zero_area() {
            return;
        }

        // collecting all text
        let symbol_keys = self.widget_store.collect_text();

        // rasterise necessary text and calculate their dimensions
        context
            .typeface
            .rasterise_symbol_keys(symbol_keys, device, queue);

        // take the mutability out of the typeface
        let context = &Context {
            typeface: context.typeface,
            aspect_ratio: context.aspect_ratio,
            cursor_position: context.cursor_position,
            viewport_dimensions_px: context.viewport_dimensions_px,
        };

        // place all widgets in the root widget's tree
        _ = self
            .widget_store
            .place_widgets(&self.root_widget_id, context);
    }

    /// Allows passing a message to the Scene externally, to be dealt with by the UI
    pub fn handle_message(&mut self, message: Message) -> Option<Message> {
        let (external_message, _rebuild_requested) =
            self.scene.handle_message(&mut self.widget_store, message);

        external_message
    }

    /// Passes a certain event to all the widgets in the WidgetStore
    pub fn handle_event(&mut self, event: Event, context: &Context) {
        for widget_entry in self.widget_store.iter_mut() {
            // getting the widget's clip rectangle/region
            let widget_region = match widget_entry.placement_info.clip_rectangle_px {
                Some(region) => region,
                None => {
                    warn!("Could not get region for Widget");
                    continue;
                }
            };

            if let Some(message) = widget_entry
                .widget
                .handle_event(&event, &widget_region, context)
            {
                self.messages.push_back(message);
            }
        }
    }

    /// Pops an external message from the queue
    pub fn pop_external_message(&mut self) -> Option<Message> {
        self.external_messages.pop_front()
    }

    /// Queues a message for the scene
    pub fn queue_message(&mut self, message: Message) {
        self.messages.push_back(message);
    }

    /// Scrolls the widgets under the cursor
    pub fn scroll_under_cursor(&mut self, context: &Context, translation: glam::IVec2) {
        self.widget_store.scroll_under_cursor(context, translation);
    }
}

impl<Message> Renderable for SceneHandle<Message>
where
    Message: Clone,
{
    fn to_render_layers(&self, context: &Context) -> Vec<RenderLayer> {
        let mut render_layers = Vec::new();
        render_layers.push(RenderLayer::new(None));

        _ = self.widget_store.widget_to_vertices(
            &self.root_widget_id,
            context,
            &mut render_layers,
            0usize,
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
