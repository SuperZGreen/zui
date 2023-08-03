use zui::{
    premade_widgets::Container, PaddingWeights, PositionConstraint, Scene, SpanConstraint,
    WidgetEntryDescriptor, WidgetId, WidgetStore,
};

use crate::UiMessage;

pub struct MainScene {
    frame_counter: u64,
    custom_counter: u64,
    cursor_widget_id: Option<WidgetId>,
}

impl MainScene {
    pub fn new() -> Self {
        Self {
            frame_counter: 0u64,
            custom_counter: 0u64,
            cursor_widget_id: None,
        }
    }
}

impl Scene for MainScene {
    type Message = UiMessage;

    fn handle_message(
        &mut self,
        widget_store: &mut WidgetStore<Self::Message>,
        message: Self::Message,
    ) -> (Option<Self::Message>, bool) {
        match message {
            UiMessage::IncrementFrameCounter(increment) => {
                self.frame_counter += increment;
                (None, true)
            }
            UiMessage::SetCustomCounter(count) => {
                self.custom_counter = count;
                (None, true)
            }
            UiMessage::IncrementCustomCounter(increment) => {
                self.custom_counter += increment;
                (None, true)
            }
            UiMessage::MoveCursor(Some(physical_position)) => {
                _ = widget_store.widget_set_position_constraint(
                    &self.cursor_widget_id.unwrap(),
                    PositionConstraint::Floating(
                        physical_position.x as i32,
                        physical_position.y as i32,
                    ),
                );
                (None, true)
            }
            UiMessage::MoveCursor(None) => {
                // TODO
                (None, true)
            }
            _ => (Some(message), false),
        }
    }

    fn init(&mut self, widget_store: &mut WidgetStore<Self::Message>) -> WidgetId {
        // TODO: this is a workaround as the scenehandle is not placing the widgets correctly..
        let root = widget_store.add(
            Container::new(),
            WidgetEntryDescriptor {
                width_constraint: SpanConstraint::ParentRatio(1f32),
                height_constraint: SpanConstraint::ParentRatio(1f32),
                ..Default::default()
            },
        );

        let main_container = widget_store.add(
            Container::new().with_background(Some(zui::named_colours::StarryNight)),
            WidgetEntryDescriptor {
                width_constraint: SpanConstraint::ParentRatio(0.5f32),
                height_constraint: SpanConstraint::ParentRatio(0.9f32),
                position_constraint: PositionConstraint::ParentDetermined(PaddingWeights::vh(
                    1f32, 1f32,
                )),
                ..Default::default()
            },
        );

        let header = widget_store.add(
            Container::new().with_background(Some(zui::named_colours::SolarFlare)),
            WidgetEntryDescriptor {
                width_constraint: SpanConstraint::ParentRatio(0.8f32),
                height_constraint: SpanConstraint::Pixels(64f32),
                position_constraint: PositionConstraint::ParentDetermined(PaddingWeights::vh(
                    0f32, 1f32,
                )),
                ..Default::default()
            },
        );

        let body = widget_store.add(
            Container::new().with_background(Some(zui::named_colours::StarshipTonic)),
            WidgetEntryDescriptor {
                width_constraint: SpanConstraint::ParentRatio(0.6f32),
                height_constraint: SpanConstraint::ParentRatio(0.8f32),
                position_constraint: PositionConstraint::ParentDetermined(PaddingWeights::vh(
                    1f32, 1f32,
                )),
                ..Default::default()
            },
        );

        let footer = widget_store.add(
            Container::new().with_background(Some(zui::named_colours::Abyssal)),
            WidgetEntryDescriptor {
                width_constraint: SpanConstraint::ParentRatio(1f32),
                height_constraint: SpanConstraint::Pixels(50f32),
                ..Default::default()
            },
        );

        // widget that follows the cursor
        let cursor = widget_store.add(
            Container::new().with_background(Some(zui::named_colours::ActiveVolcano)),
            WidgetEntryDescriptor {
                width_constraint: SpanConstraint::Pixels(32f32),
                height_constraint: SpanConstraint::Pixels(32f32),
                position_constraint: PositionConstraint::Floating(30, 30),
                ..Default::default()
            },
        );

        self.cursor_widget_id = Some(cursor);

        _ = widget_store.widget_add_child(&root, main_container);

        _ = widget_store.widget_add_child(&main_container, header);
        _ = widget_store.widget_add_child(&main_container, body);
        _ = widget_store.widget_add_child(&main_container, footer);

        _ = widget_store.widget_add_child(&main_container, cursor);

        root
    }
}
