use zui::{
    premade_widgets::{Container, TextContainer},
    EntryOverrideDescriptor, PaddingWeights, ParentHeight, ParentWidth, PositionConstraint, Scene,
    SpanConstraint, Text, TextSegment, WidgetId, WidgetStore, TextConfiguration, EntryChildren, Axis,
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
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1f32))),
                height_constraint: Some(SpanConstraint::ParentHeight(ParentHeight::new(1f32))),
                ..Default::default()
            },
        );

        let main_container = widget_store.add(
            Container::new().with_background(Some(zui::named_colours::StarryNight)),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(0.5f32))),
                height_constraint: Some(SpanConstraint::ParentHeight(ParentHeight::new(0.9f32))),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::vh(1f32, 1f32),
                )),
                ..Default::default()
            },
        );

        let header = widget_store.add(
            Container::new().with_background(Some(zui::named_colours::SolarFlare)),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWeight(2f32)),
                height_constraint: Some(SpanConstraint::Pixels(64f32)),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::new(0f32, 0f32, 1f32, 0f32),
                )),
                ..Default::default()
            },
        );

        let body = widget_store.add(
            Container::new().with_background(Some(zui::named_colours::StarshipTonic)),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(0.6f32))),
                height_constraint: Some(SpanConstraint::ParentWeight(10f32)),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::vh(1f32, 1f32),
                )),
                ..Default::default()
            },
        );

        let first_paragraph = widget_store.add(
            TextContainer::new()
            .with_text(Text::new().push_segment(TextSegment::new("Hello there! This is my very long first paragraph full of much useful information that you'll need on your journey through this demo!", zui::named_colours::White)))
            .with_background_colour(Some(zui::named_colours::RedTape)),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1.0f32))),
                // height_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(0.6f32))),
                ..Default::default()
            },
        );

        let second_paragraph = widget_store.add(
            TextContainer::new()
            .with_text(
                Text::new()
                    .push_segment(TextSegment::new("This is the Second Paragraph. I spent a lot of time putting together this demo for this library. I hope you enjoy. Take care! Adieu! Bye-bye!", zui::named_colours::White))
                    .with_configuration(TextConfiguration {
                        line_wrapping: zui::LineWrapping::Word,
                        ..Default::default()
                    })
            )
            .with_background_colour(Some(zui::named_colours::Avocado)),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1.0f32))),
                // height_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(0.6f32))),
                ..Default::default()
            },
        );

        let pretend_image = widget_store.add(
            Container::new().with_background(Some(zui::named_colours::Goldfish)),
            EntryOverrideDescriptor {
                children: Some(EntryChildren::new(Axis::Horizontal)),
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(0.6f32))),
                height_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(0.4f32))),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::vh(1f32, 1f32),
                )),
                ..Default::default()
            },
        );

        let horizontal_test_1 = widget_store.add(
            Container::new().with_background(Some(zui::named_colours::RedDevil)),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::Pixels(32f32)),
                height_constraint: Some(SpanConstraint::Pixels(32f32)),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::new(1f32, 0f32, 0f32, 0f32),
                )),
                ..Default::default()
            },
        );

        let horizontal_test_2 = widget_store.add(
            Container::new().with_background(Some(zui::named_colours::GreenEnvy)),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::Pixels(32f32)),
                height_constraint: Some(SpanConstraint::Pixels(32f32)),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::new(1f32, 1f32, 1f32, 1f32),
                )),
                ..Default::default()
            },
        );

        let horizontal_test_3 = widget_store.add(
            Container::new().with_background(Some(zui::named_colours::BlueChip)),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::Pixels(32f32)),
                height_constraint: Some(SpanConstraint::Pixels(32f32)),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::new(0f32, 1f32, 0f32, 0f32),
                )),
                ..Default::default()
            },
        );

        let third_paragraph = widget_store.add(
            TextContainer::new()
            .with_text(
                Text::new()
                    .push_segment(TextSegment::new("This is the third paragraph", zui::named_colours::White))
                    .with_configuration(TextConfiguration {
                        horizontal_alignment: zui::TextAlignmentHorizontal::Right,
                        line_wrapping: zui::LineWrapping::Word,
                        ..Default::default()
                    })
            )
            .with_background_colour(Some(zui::named_colours::Azure)),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1.0f32))),
                // height_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(0.6f32))),
                ..Default::default()
            },
        );

        let footer = widget_store.add(
            Container::new().with_background(Some(zui::named_colours::Abyssal)),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1f32))),
                height_constraint: Some(SpanConstraint::Pixels(50f32)),
                ..Default::default()
            },
        );

        // widget that follows the cursor
        let cursor = widget_store.add(
            Container::new().with_background(Some(zui::named_colours::ActiveVolcano)),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::Pixels(32f32)),
                height_constraint: Some(SpanConstraint::Pixels(32f32)),
                position_constraint: Some(PositionConstraint::Floating(30, 30)),
                ..Default::default()
            },
        );

        self.cursor_widget_id = Some(cursor);

        _ = widget_store.widget_add_child(&root, main_container);

        _ = widget_store.widget_add_child(&main_container, header);
        _ = widget_store.widget_add_child(&main_container, body);
        _ = widget_store.widget_add_child(&main_container, footer);

        _ = widget_store.widget_add_child(&main_container, cursor);

        _ = widget_store.widget_add_child(&body, first_paragraph);
        _ = widget_store.widget_add_child(&body, second_paragraph);
        _ = widget_store.widget_add_child(&body, pretend_image);
        _ = widget_store.widget_add_child(&body, third_paragraph);

        _ = widget_store.widget_add_child(&pretend_image, horizontal_test_1);
        _ = widget_store.widget_add_child(&pretend_image, horizontal_test_2);
        _ = widget_store.widget_add_child(&pretend_image, horizontal_test_3);

        root
    }
}
