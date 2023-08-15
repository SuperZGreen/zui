use zui::{
    premade_widgets::{Container, TextContainer},
    Axis, Colour, EntryChildren, EntryOverrideDescriptor, PaddingWeights, ParentHeight,
    ParentWidth, PositionConstraint, Scene, SpanConstraint, Text, TextConfiguration, TextSegment,
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
                children: Some(EntryChildren::new(Axis::Horizontal)),
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1f32))),
                height_constraint: Some(SpanConstraint::ParentHeight(ParentHeight::new(1f32))),
                ..Default::default()
            },
        );

        let info_text = widget_store.add(
            TextContainer::new().with_text(
                Text::new()
                    .push_segment(TextSegment::new(
                        "This is the container scene!",
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

        _ = widget_store.widget_add_child(&root, info_text);

        root
    }
}
