use zui::{
    premade_widgets::Container, PositionConstraint, Scene, SpanConstraint, WidgetEntryDescriptor,
    WidgetId, WidgetStore,
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
        // let box_aa = widget_store.add(Container::new().with_background(Some(zui::named_colours::Fox)));
        // _ = widget_store.widget_set_width_contraint(&box_aa, SpanConstraint::Pixels(64f32));
        // _ = widget_store.widget_set_height_contraint(&box_aa, SpanConstraint::Pixels(64f32));

        // let box_aba = widget_store.add(Container::new().with_background(Some(zui::named_colours::NavalAdventures)));
        // _ = widget_store.widget_set_width_contraint(&box_aba, SpanConstraint::Pixels(32f32));
        // _ = widget_store.widget_set_height_contraint(&box_aba, SpanConstraint::Pixels(32f32));

        // let box_abb = widget_store.add(Container::new().with_background(Some(zui::named_colours::Gold)));
        // _ = widget_store.widget_set_width_contraint(&box_abb, SpanConstraint::Pixels(150f32));
        // _ = widget_store.widget_set_height_contraint(&box_abb, SpanConstraint::Pixels(30f32));

        // let box_ab = widget_store.add(Container::new().with_background(Some(zui::named_colours::BloodOrange)));
        // _ = widget_store.widget_set_width_contraint(&box_ab, SpanConstraint::FitContents);
        // _ = widget_store.widget_set_height_contraint(&box_ab, SpanConstraint::Pixels(128f32));
        // _ = widget_store.widget_add_child(&box_ab, box_aba);
        // _ = widget_store.widget_add_child(&box_ab, box_abb);

        // let box_ac = widget_store.add(Container::new().with_background(Some(zui::named_colours::AhoyBlue)));
        // _ = widget_store.widget_set_width_contraint(&box_ac, SpanConstraint::Pixels(100f32));
        // _ = widget_store.widget_set_height_contraint(&box_ac, SpanConstraint::Pixels(50f32));

        // let box_a = widget_store.add(Container::new().with_background(Some(zui::named_colours::Frog)));
        // _ = widget_store.widget_set_width_contraint(&box_a, SpanConstraint::ParentRatio(0.8f32));
        // _ = widget_store.widget_set_height_contraint(&box_a, SpanConstraint::FitContents);
        // _ = widget_store.widget_add_child(&box_a, box_aa);
        // _ = widget_store.widget_add_child(&box_a, box_ab);
        // _ = widget_store.widget_add_child(&box_a, box_ac);

        // box_a

        let main_container = widget_store.add(
            Container::new().with_background(Some(zui::named_colours::StarryNight)),
            WidgetEntryDescriptor {
                width_constraint: SpanConstraint::ParentRatio(0.5f32),
                height_constraint: SpanConstraint::ParentRatio(0.9f32),
                ..Default::default()
            },
        );

        let header = widget_store.add(
            Container::new().with_background(Some(zui::named_colours::SolarFlare)),
            WidgetEntryDescriptor {
                width_constraint: SpanConstraint::ParentRatio(0.8f32),
                height_constraint: SpanConstraint::Pixels(64f32),
                ..Default::default()
            },
        );

        let body = widget_store
            .add(Container::new().with_background(Some(zui::named_colours::StarshipTonic)),
            WidgetEntryDescriptor {
                width_constraint: SpanConstraint::ParentRatio(0.6f32),
                height_constraint: SpanConstraint::ParentRatio(0.8f32),
                ..Default::default()
            },
        );

        // widget that follows the cursor
        let cursor = widget_store.add(Container::new().with_background(Some(zui::named_colours::ActiveVolcano)),
            WidgetEntryDescriptor {
                width_constraint: SpanConstraint::Pixels(32f32),
                height_constraint: SpanConstraint::Pixels(32f32),
                position_constraint: PositionConstraint::Floating(30, 30),
                ..Default::default()
            },
        );

        self.cursor_widget_id = Some(cursor);

        _ = widget_store.widget_add_child(&main_container, header);
        _ = widget_store.widget_add_child(&main_container, body);
        _ = widget_store.widget_add_child(&main_container, cursor);

        main_container
    }
}
