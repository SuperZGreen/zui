use zui::{
    premade_widgets::{Container, TextContainer},
    Axis, Colour, EntryChildren, EntryOverrideDescriptor, PaddingWeights, ParentHeight,
    ParentWidth, PositionConstraint, Scene, SpanConstraint, Text, TextConfiguration, TextSegment, typeface::FontStyle,
};

use crate::UiMessage;

pub struct TextScene {
    // TODO
}

impl TextScene {
    pub fn new() -> Self {
        Self {}
    }
}

impl Scene for TextScene {
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
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1f32))),
                height_constraint: Some(SpanConstraint::ParentHeight(ParentHeight::new(1f32))),
                ..Default::default()
            },
        );

        let title = widget_store.add(
            TextContainer::new().with_text(
                Text::new()
                    .push_segment(TextSegment {
                        string: String::from("Text/TextContainer Demo"),
                        colour: Colour::WHITE,
                        style: FontStyle::Bold,
                    })
                    .with_configuration(TextConfiguration {
                        line_wrapping: zui::LineWrapping::Word,
                        size_px: 64,
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

        let alignment_info = widget_store.add(
            TextContainer::new().with_text(
                Text::new()
                    .push_segment(TextSegment::new(
                        "Text can be aligned horizontally:",
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

        let alignment_left = widget_store.add(
            TextContainer::new().with_text(
                Text::new()
                    .push_segment(TextSegment::new(
                        "To the ",
                        Colour::WHITE,
                    ))
                    .push_segment(TextSegment {
                        string: String::from("left"),
                        colour: Colour::WHITE,
                        style: FontStyle::Italic,
                    })
                    .push_segment(TextSegment::new(
                        ",",
                        Colour::WHITE,
                    ))
                    .with_configuration(TextConfiguration {
                        line_wrapping: zui::LineWrapping::Word,
                        horizontal_alignment: zui::TextAlignmentHorizontal::Left,
                        ..Default::default()
                    }),
            ).with_background_colour(Some(zui::named_colours::RedTape)),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1f32))),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::NONE,
                )),
                ..Default::default()
            },
        );
        let alignment_centre = widget_store.add(
            TextContainer::new().with_text(
                Text::new()
                    .push_segment(TextSegment::new(
                        "in the ",
                        Colour::WHITE,
                    ))
                    .push_segment(TextSegment {
                        string: String::from("centre"),
                        colour: Colour::WHITE,
                        style: FontStyle::Italic,
                    })
                    .push_segment(TextSegment::new(
                        ",",
                        Colour::WHITE,
                    ))
                    .with_configuration(TextConfiguration {
                        line_wrapping: zui::LineWrapping::Word,
                        horizontal_alignment: zui::TextAlignmentHorizontal::Centre,
                        ..Default::default()
                    }),
            ).with_background_colour(Some(zui::named_colours::GreenEnvy)),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1f32))),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::NONE,
                )),
                ..Default::default()
            },
        );
        let alignment_right = widget_store.add(
            TextContainer::new().with_text(
                Text::new()
                    .push_segment(TextSegment::new(
                        "or to the ",
                        Colour::WHITE,
                    ))
                    .push_segment(TextSegment {
                        string: String::from("right"),
                        colour: Colour::WHITE,
                        style: FontStyle::Italic,
                    })
                    .push_segment(TextSegment::new(
                        ".",
                        Colour::WHITE,
                    ))
                    .with_configuration(TextConfiguration {
                        line_wrapping: zui::LineWrapping::Word,
                        horizontal_alignment: zui::TextAlignmentHorizontal::Right,
                        ..Default::default()
                    }),
            ).with_background_colour(Some(zui::named_colours::BlueChip)),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1f32))),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::NONE,
                )),
                ..Default::default()
            },
        );

        let vertical_alignment_info = widget_store.add(
            TextContainer::new().with_text(
                Text::new()
                    .push_segment(TextSegment::new(
                        "Text can also be alligned vertically:",
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

        let vertical_alignment_container = widget_store.add(
            Container::new().with_background(Some(Colour::BLACK)),
            EntryOverrideDescriptor {
                children: Some(EntryChildren::new(Axis::Horizontal)),
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1f32))),
                height_constraint: Some(SpanConstraint::ParentHeight(ParentHeight::new(0.1f32))),
                ..Default::default()
            },
        );

        let vertical_alignment_bottom = widget_store.add(
            TextContainer::new().with_text(
                Text::new()
                    .push_segment(TextSegment::new(
                        "At the bottom,",
                        Colour::WHITE,
                    ))
                    .with_configuration(TextConfiguration {
                        line_wrapping: zui::LineWrapping::Word,
                        horizontal_alignment: zui::TextAlignmentHorizontal::Centre,
                        vertical_alignment: zui::TextAlignmentVertical::Bottom,
                        ..Default::default()
                    }),
            ).with_background_colour(Some(zui::named_colours::RedTape)),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(0.2f32))),
                height_constraint: Some(SpanConstraint::ParentHeight(ParentHeight::new(1f32))),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::NONE,
                )),
                ..Default::default()
            },
        );

        let vertical_alignment_centre = widget_store.add(
            TextContainer::new().with_text(
                Text::new()
                    .push_segment(TextSegment::new(
                        "in the ",
                        Colour::WHITE,
                    ))
                    .push_segment(TextSegment {
                        string: String::from("centre"),
                        colour: Colour::WHITE,
                        style: FontStyle::Italic,
                    })
                    .push_segment(TextSegment::new(
                        ".",
                        Colour::WHITE,
                    ))
                    .with_configuration(TextConfiguration {
                        line_wrapping: zui::LineWrapping::Word,
                        horizontal_alignment: zui::TextAlignmentHorizontal::Centre,
                        vertical_alignment: zui::TextAlignmentVertical::Centre,
                        ..Default::default()
                    }),
            ).with_background_colour(Some(zui::named_colours::GreenEnvy)),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(0.2f32))),
                height_constraint: Some(SpanConstraint::ParentHeight(ParentHeight::new(1f32))),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::vh(0f32, 1f32),
                )),
                ..Default::default()
            },
        );

        let vertical_alignment_top = widget_store.add(
            TextContainer::new().with_text(
                Text::new()
                    .push_segment(TextSegment::new(
                        "or at the top.",
                        Colour::WHITE,
                    ))
                    .with_configuration(TextConfiguration {
                        line_wrapping: zui::LineWrapping::Word,
                        horizontal_alignment: zui::TextAlignmentHorizontal::Centre,
                        vertical_alignment: zui::TextAlignmentVertical::Top,
                        ..Default::default()
                    }),
            ).with_background_colour(Some(zui::named_colours::BlueChip)),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(0.2f32))),
                height_constraint: Some(SpanConstraint::ParentHeight(ParentHeight::new(1f32))),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::NONE
                )),
                ..Default::default()
            },
        );

        let styling_info = widget_store.add(
            TextContainer::new().with_text(
                Text::new()
                    .push_segment(TextSegment {
                        string: String::from("Text segments can be "),
                        colour: Colour::WHITE,
                        style: FontStyle::Regular
                    })
                    .push_segment(TextSegment {
                        string: String::from("different "),
                        colour: Colour::WHITE,
                        style: FontStyle::Bold
                    })
                    .push_segment(TextSegment {
                        string: String::from("styles"),
                        colour: Colour::WHITE,
                        style: FontStyle::Italic
                    })
                    .push_segment(TextSegment {
                        string: String::from(" if bold or italic fonts have been loaded. Text segments can also be "),
                        colour: Colour::WHITE,
                        style: FontStyle::Regular
                    })
                    .push_segment(TextSegment {
                        string: String::from("different"),
                        colour: zui::named_colours::GreenEnvy,
                        style: FontStyle::Regular
                    })
                    .push_segment(TextSegment {
                        string: String::from(" colours"),
                        colour: zui::named_colours::RedTape,
                        style: FontStyle::Regular
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


        _ = widget_store.widget_add_child(&root, title);

        _ = widget_store.widget_add_child(&root, alignment_info);
        _ = widget_store.widget_add_child(&root, alignment_left);
        _ = widget_store.widget_add_child(&root, alignment_centre);
        _ = widget_store.widget_add_child(&root, alignment_right);

        _ = widget_store.widget_add_child(&root, vertical_alignment_info);
        _ = widget_store.widget_add_child(&root, vertical_alignment_container);

        _ = widget_store.widget_add_child(&vertical_alignment_container, vertical_alignment_bottom);
        _ = widget_store.widget_add_child(&vertical_alignment_container, vertical_alignment_centre);
        _ = widget_store.widget_add_child(&vertical_alignment_container, vertical_alignment_top);

        _ = widget_store.widget_add_child(&root, styling_info);

        root
    }
}
