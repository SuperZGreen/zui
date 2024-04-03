use zui::{
    premade_widgets::{
        Button, Container, ContainerBackground, TextContainer, TextContainerDescriptor,
    },
    text::TextDescriptor,
    typeface::FontStyle,
    Axis, Colour, EntryChildren, EntryOverrideDescriptor, PaddingWeights, ParentHeight,
    ParentWidth, PositionConstraint, Scene, SpanConstraint, SpriteId, Text, TextConfiguration,
    TextSegment,
};

use crate::UiMessage;

pub struct ContainerScene {
    // TODO
}

impl ContainerScene {
    pub fn new() -> Self {
        Self {}
    }
}

impl Scene for ContainerScene {
    type Message = UiMessage;

    fn handle_message(
        &mut self,
        _widget_store: &mut zui::WidgetStore<Self::Message>,
        _message: Self::Message,
    ) -> (Option<Self::Message>, bool) {
        todo!()
    }

    fn init(&mut self, widget_store: &mut zui::WidgetStore<Self::Message>) -> zui::WidgetId {
        // TODO: this is a workaround as the scenehandle is not placing the widgets correctly..
        let root = widget_store.add(
            Container::new(),
            EntryOverrideDescriptor {
                children: Some(EntryChildren::new(Axis::Vertical)),
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1f32))),
                height_constraint: Some(SpanConstraint::ParentHeight(ParentHeight::new(1f32))),
                ..Default::default()
            },
        );

        let title = widget_store.add(
            TextContainerDescriptor {
                text: Some(
                    TextDescriptor {
                        segments: vec![TextSegment {
                            string: String::from("Container Demo"),
                            style: FontStyle::Bold,
                            ..Default::default()
                        }],
                        configuration: TextConfiguration {
                            size_px: 64,
                            ..Default::default()
                        },
                    }
                    .into(),
                ),
                background_colour: None,
            },
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1f32))),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::NONE,
                )),
                ..Default::default()
            },
        );

        let info_text = widget_store.add(
            TextContainer::new().with_text(
                Text::new()
                    .push_segment(TextSegment::new(
                        "Containers are just empty widgets that allow for other widgets to be \
                        drawn as their children. Containers can have their own background colours.",
                        Colour::WHITE,
                    ))
                    .with_configuration(TextConfiguration {
                        line_wrapping: zui::LineWrapping::Word,
                        ..Default::default()
                    }),
            ),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1f32))),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::NONE,
                )),
                ..Default::default()
            },
        );

        let overflowing_container = widget_store.add(
            Container::new()
                .with_background(Some(ContainerBackground::Colour(Colour::DARK_ORANGE))),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(0.5f32))),
                height_constraint: Some(SpanConstraint::Aspect(1f32)),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::vh(0f32, 1f32),
                )),
                ..Default::default()
            },
        );

        let overflowing_container_title_text = widget_store.add(
            TextContainer::new().with_text(
                Text::new()
                    .push_segment(TextSegment {
                        string: String::from("Overflowing/Scrollable Containers"),
                        colour: Colour::WHITE,
                        style: FontStyle::Bold,
                    })
                    .with_configuration(TextConfiguration {
                        line_wrapping: zui::LineWrapping::Word,
                        ..Default::default()
                    }),
            ),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1f32))),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::NONE,
                )),
                ..Default::default()
            },
        );

        let overflowing_container_info_text = widget_store.add(
            TextContainer::new().with_text(
                Text::new()
                    .push_segment(TextSegment::new(
                        "This is an example of a scrollable container that is overflowing. Note \
                        that overflowing elements are clipped using RenderLayers. This allows \
                        containers elements to be ensured to be kept within the container, however \
                        this does require an extra draw call in the RenderPass to be performed.",
                        Colour::WHITE,
                    ))
                    .with_configuration(TextConfiguration {
                        line_wrapping: zui::LineWrapping::Word,
                        ..Default::default()
                    }),
            ),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1f32))),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::NONE,
                )),
                ..Default::default()
            },
        );

        let overflowing_container_filler_text = widget_store.add(
            TextContainer::new().with_text(
                Text::new()
                    .push_segment(TextSegment::new(
                        "The following is filler text just to bulk out the overflowing container. \
                        This is to give you an idea of how the container will look when it's \
                        filled with items, and then allow the demonstration of both clipping of \
                        overflowing items, and allow the demonstration of scrolling of overflowing \
                        items. At the time of me writing this, it is very likely that there is \
                        more work to be done on scrolling and clipping, because of various \
                        internal overhauls of unit systems, as well as my desire to rework my code \
                        into something neat and maintainable, while preserving performance. \
                        Hopefully this is overflowing by now!",
                        Colour::WHITE,
                    ))
                    .with_configuration(TextConfiguration {
                        line_wrapping: zui::LineWrapping::Word,
                        ..Default::default()
                    }),
            ).with_background_colour(Some(zui::named_colours::LeafyLush)),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1f32))),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::NONE,
                )),
                ..Default::default()
            },
        );

        let overflowing_container_filler_text_final = widget_store.add(
            TextContainer::new().with_text(
                Text::new()
                    .push_segment(TextSegment::new(
                        "This is the final line of the filler text! It has a transparent \
                        background just so you can be sure where the filler text ends. Goodbye!",
                        Colour::WHITE,
                    ))
                    .with_configuration(TextConfiguration {
                        line_wrapping: zui::LineWrapping::Word,
                        ..Default::default()
                    }),
            ),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1f32))),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::NONE,
                )),
                ..Default::default()
            },
        );

        let overflowing_container_image = widget_store.add(
            Container::new().with_background(Some(ContainerBackground::Image(SpriteId(1)))),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(0.5f32))),
                height_constraint: Some(SpanConstraint::Aspect(1f32)),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::NONE,
                )),
                ..Default::default()
            },
        );

        let overflowing_container_button = widget_store.add(
            Button::new(
                UiMessage::ToggleFullscreen,
                zui::Colour::DARK_GREY,
                zui::Colour::LIGHT_GREY,
            )
            .with_text(TextDescriptor {
                segments: vec![TextSegment::from("Hello")],
                ..Default::default()
            }),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1f32))),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::vh(0f32, 1f32),
                )),
                ..Default::default()
            },
        );

        let lorem_ipsum =
            "Praesent ac iaculis mauris. Pellentesque varius consequat odio in tempus. Pellentesque\
            euismod, massa eleifend vulputate volutpat, purus neque e uismod erat, ac pretium nunc \
            ipsum quis libero. Phasellus posuere augue laoreet ipsum vulputate, quis commodo sem di\
            gnissim. Phasellus et sapien sit amet libero pellentesque tempus vitae eu eros. Aenean \
            fermentum leo sit amet neque venenatis, at congue ex interdum. Pellentesque purus risus\
            , porta vitae lorem quis, varius pharetra nunc. Pellentesque et bibendum justo. ";

        let mut big_string = String::with_capacity(lorem_ipsum.len() * 50);
        for _ in 0..50 {
            big_string.push_str(lorem_ipsum);
        }

        let lorem_ipsum_text_container = widget_store.add(
            TextContainer::new().with_text(
                Text::new()
                    .push_segment(TextSegment::new(&big_string, Colour::WHITE))
                    .with_configuration(TextConfiguration {
                        line_wrapping: zui::LineWrapping::Symbol,
                        ..Default::default()
                    }),
            ),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1f32))),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::NONE,
                )),
                ..Default::default()
            },
        );

        _ = widget_store.widget_add_child(&root, title);
        _ = widget_store.widget_add_child(&root, info_text);

        _ = widget_store.widget_add_child(&overflowing_container, overflowing_container_title_text);
        _ = widget_store.widget_add_child(&overflowing_container, overflowing_container_info_text);
        _ = widget_store
            .widget_add_child(&overflowing_container, overflowing_container_filler_text);
        _ = widget_store.widget_add_child(
            &overflowing_container,
            overflowing_container_filler_text_final,
        );
        _ = widget_store.widget_add_child(&overflowing_container, overflowing_container_image);
        _ = widget_store.widget_add_child(&overflowing_container, overflowing_container_button);
        _ = widget_store.widget_add_child(&root, overflowing_container);
        _ = widget_store.widget_add_child(&root, lorem_ipsum_text_container);

        root
    }
}
