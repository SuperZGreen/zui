use winit::dpi::PhysicalPosition;
use zui::{
    premade_widgets::{Container, TextContainer},
    Axis, Colour, EntryChildren, EntryOverrideDescriptor, PaddingWeights, ParentHeight,
    ParentWidth, PositionConstraint, Scene, SpanConstraint, Text, TextConfiguration, TextSegment,
    WidgetId, WidgetStore,
};

use crate::UiMessage;

use rand::Rng;

use super::{container_demo, SceneIdentifier, text_demo};

#[derive(Copy, Clone)]
pub enum BaseSceneMessage {
    MoveCursor(Option<PhysicalPosition<f64>>),
    AddSidebarText,
    ClearSidebarText,
    ChangeScene(SceneIdentifier),
}

pub struct BaseScene {
    display_area: Option<WidgetId>,
    sidebar: Option<WidgetId>,
    counter: i32,
    cursor_widget_id: Option<WidgetId>,

    child_scene: Option<Box<dyn Scene<Message = UiMessage>>>,
}

impl BaseScene {
    pub fn new() -> Self {
        Self {
            display_area: None,

            sidebar: None,
            counter: 0i32,

            cursor_widget_id: None,

            child_scene: None,
        }
    }
}

impl Scene for BaseScene {
    type Message = UiMessage;

    fn handle_message(
        &mut self,
        widget_store: &mut WidgetStore<Self::Message>,
        message: Self::Message,
    ) -> (Option<Self::Message>, bool) {
        match message {
            UiMessage::BaseSceneMessage(msm) => match msm {
                BaseSceneMessage::MoveCursor(Some(physical_position)) => {
                    _ = widget_store.widget_set_position_constraint(
                        &self.cursor_widget_id.unwrap(),
                        PositionConstraint::Floating(
                            physical_position.x as i32,
                            physical_position.y as i32,
                        ),
                    );
                    (None, true)
                }
                BaseSceneMessage::MoveCursor(None) => (None, true),
                BaseSceneMessage::AddSidebarText => {
                    if let Some(body_id) = self.sidebar {
                        let mut rng = rand::thread_rng();

                        let background_colour = Colour::rgb(rng.gen::<f32>(), rng.gen(), rng.gen());

                        let text_colour =
                            if background_colour.r + background_colour.g + background_colour.b
                                > 1.5f32
                            {
                                Colour::BLACK
                            } else {
                                Colour::WHITE
                            };

                        let text = format!("this is line number: {}", self.counter);
                        self.counter += 1;

                        let text_widget = widget_store.add(
                            TextContainer::new()
                                .with_text(
                                    Text::new().push_segment(TextSegment::new(&text, text_colour)),
                                )
                                .with_background_colour(Some(background_colour)),
                            EntryOverrideDescriptor {
                                width_constraint: Some(SpanConstraint::ParentWidth(
                                    ParentWidth::new(1.0f32),
                                )),
                                ..Default::default()
                            },
                        );

                        _ = widget_store.widget_add_child(&body_id, text_widget);
                    }

                    (None, true)
                }
                BaseSceneMessage::ClearSidebarText => {
                    if let Some(body_id) = self.sidebar {
                        _ = widget_store.widget_delete_children(&body_id);
                    }

                    (None, true)
                }

                BaseSceneMessage::ChangeScene(scene_identifier) => {
                    if let Some(display_area) = self.display_area {
                        _ = widget_store.widget_delete_children(&display_area);

                        self.child_scene = match scene_identifier {
                            SceneIdentifier::ContainerDemo => {
                                let mut scene = Box::new(container_demo::ContainerScene::new());
                                let root_widget = scene.init(widget_store);
                                _ = widget_store.widget_add_child(&display_area, root_widget);
                                Some(scene)
                            }
                            SceneIdentifier::TextDemo => {
                                let mut scene = Box::new(text_demo::TextScene::new());
                                let root_widget = scene.init(widget_store);
                                _ = widget_store.widget_add_child(&display_area, root_widget);
                                Some(scene)
                            }
                            SceneIdentifier::ButtonDemo => None,
                            SceneIdentifier::FillBarDemo => None,
                        }
                    }

                    (None, true)
                }
            },

            _ => (Some(message), false),
        }
    }

    fn init(&mut self, widget_store: &mut WidgetStore<Self::Message>) -> WidgetId {
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

        let sidebar = widget_store.add(
            Container::new().with_background(Some(zui::named_colours::Black)),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(0.2f32))),
                height_constraint: Some(SpanConstraint::ParentHeight(ParentHeight::new(1f32))),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::NONE,
                )),
                ..Default::default()
            },
        );

        let info_text = widget_store.add(
            TextContainer::new().with_text(
                Text::new().push_segment(TextSegment::new(
                        "Press keys 1-3 to swap the active scene. 'a' will add text to this sidebar, and 'd' will delete it. Press 'x' to exit!"
                        , Colour::WHITE))
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

        let display_area_background = widget_store.add(
            Container::new().with_background(Some(zui::named_colours::Corbeau)),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWeight(1f32)),
                height_constraint: Some(SpanConstraint::ParentHeight(ParentHeight::new(1f32))),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::NONE,
                )),
                ..Default::default()
            },
        );

        let display_area = widget_store.add(
            Container::new().with_background(Some(zui::named_colours::CapitalBlue)),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWeight(5f32)),
                height_constraint: Some(SpanConstraint::ParentHeight(ParentHeight::new(1f32))),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::vh(0f32, 1f32),
                )),
                ..Default::default()
            },
        );

        let text_none = widget_store.add(
            TextContainer::new().with_text(
                Text::new().push_segment(TextSegment::new(
                        "No demo scene selected!"
                        , Colour::WHITE))
                .with_configuration(TextConfiguration {
                    line_wrapping: zui::LineWrapping::Word,
                    horizontal_alignment: zui::TextAlignmentHorizontal::Centre,
                    // vertical_alignment: zui::text::TextAlignmentVertical::Centre,
                    ..Default::default()
                }),
            ),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1f32))),
                // height_constraint: Some(SpanConstraint::ParentHeight(ParentHeight::new(1f32))),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::NONE
                )),
                ..Default::default()
            },
        );

        let cursor = widget_store.add(
            Container::new().with_background(Some(zui::named_colours::LemonGrass)),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::Pixels(32f32)),
                height_constraint: Some(SpanConstraint::Pixels(32f32)),
                position_constraint: Some(PositionConstraint::Floating(32, 32)),
                ..Default::default()
            },
        );

        self.cursor_widget_id = Some(cursor);
        self.display_area = Some(display_area);
        self.sidebar = Some(sidebar);

        _ = widget_store.widget_add_child(&sidebar, info_text);

        _ = widget_store.widget_add_child(&root, sidebar);
        _ = widget_store.widget_add_child(&root, display_area_background);
        _ = widget_store.widget_add_child(&root, cursor);

        _ = widget_store.widget_add_child(&display_area, text_none);
        _ = widget_store.widget_add_child(&display_area_background, display_area);

        root
    }
}
