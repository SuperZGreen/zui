use winit::dpi::PhysicalPosition;
use zui::{
    premade_widgets::{Button, Container, ContainerBackground, TextContainer},
    Axis, Colour, EntryChildren, EntryOverrideDescriptor, PaddingWeights, ParentHeight,
    ParentWidth, PositionConstraint, Scene, SpanConstraint, Text, TextConfiguration, TextSegment,
    WidgetId, WidgetStore,
};

use crate::UiMessage;

use rand::Rng;

use super::{container_demo, text_demo, SceneIdentifier};

#[derive(Copy, Clone, Debug)]
pub enum BaseSceneMessage {
    MoveCursor(Option<PhysicalPosition<f64>>),
    AddSidebarText,
    ClearSidebarText,
    ChangeScene(SceneIdentifier),
    UpdateFrameTime(f32),
}

pub struct BaseScene {
    /// The WidgetId of the display area for the demos
    display_area: Option<WidgetId>,

    /// The WidgetId of the sidebar container
    sidebar: Option<WidgetId>,

    /// The WidgetId of the frame time TextContainer
    frame_time_id: Option<WidgetId>,

    counter: i32,

    // The id of the widget that tracks the cursor
    cursor_widget_id: Option<WidgetId>,

    // The buttons on the sidebar that lead to the demo scenes
    demo_navigation_buttons: Vec<WidgetId>,

    // The current child scene
    child_scene: Option<Box<dyn Scene<Message = UiMessage>>>,
}

impl BaseScene {
    pub fn new() -> Self {
        Self {
            display_area: None,

            sidebar: None,

            frame_time_id: None,

            counter: 0i32,

            cursor_widget_id: None,

            demo_navigation_buttons: Vec::new(),

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
            UiMessage::Exit => (Some(UiMessage::Exit), false),

            UiMessage::ToggleFullscreen => (Some(UiMessage::ToggleFullscreen), false),

            UiMessage::BaseSceneMessage(bsm) => match bsm {
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
                                // creating a new instance of the scene
                                let mut scene = Box::new(container_demo::ContainerScene::new());

                                // initing the scene
                                let root_widget = scene.init(widget_store);

                                // adding the scene's root widget as a child of display area
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

                BaseSceneMessage::UpdateFrameTime(frame_time) => {
                    if let Some(frame_time_id) = self.frame_time_id {
                        if let Some(widget_entry) = widget_store.get_mut(&frame_time_id) {
                            let text_container: &mut TextContainer =
                                widget_entry.widget.as_any_mut().downcast_mut().unwrap();

                            text_container.set_text(
                                Text::new()
                                    .push_segment(TextSegment::new(
                                        &format!("frame_time: {:.2} ms", frame_time * 1000f32),
                                        Colour::WHITE,
                                    ))
                                    .with_configuration(TextConfiguration {
                                        size_px: 16,
                                        ..Default::default()
                                    }),
                            );
                        }
                    }

                    (None, true)
                }
            },
            // _ => (Some(message), false),
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
            Container::new()
                .with_background(Some(ContainerBackground::Colour(zui::named_colours::Black))),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(0.2f32))),
                height_constraint: Some(SpanConstraint::ParentHeight(ParentHeight::new(1f32))),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::NONE,
                )),
                ..Default::default()
            },
        );

        let container_demo_button = widget_store.add(
            Button::new(
                UiMessage::BaseSceneMessage(BaseSceneMessage::ChangeScene(
                    SceneIdentifier::ContainerDemo,
                )),
                zui::named_colours::BonneNuit,
                zui::named_colours::Botanical,
            )
            .with_text(Text::new().push_segment(TextSegment::new("Container Demo", Colour::WHITE))),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1f32))),
                height_constraint: Some(SpanConstraint::FitContents),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::NONE,
                )),
                ..Default::default()
            },
        );

        let button_demo_button = widget_store.add(
            Button::new(
                UiMessage::BaseSceneMessage(BaseSceneMessage::ChangeScene(
                    SceneIdentifier::ButtonDemo,
                )),
                zui::named_colours::BonneNuit,
                zui::named_colours::Botanical,
            )
            .with_text(Text::new().push_segment(TextSegment::new("ButtonDemo", Colour::WHITE))),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1f32))),
                height_constraint: Some(SpanConstraint::FitContents),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::NONE,
                )),
                ..Default::default()
            },
        );

        let toggle_fullscreen_button = widget_store.add(
            Button::new(
                UiMessage::ToggleFullscreen,
                zui::named_colours::Amazon,
                zui::named_colours::AmbrosialOceanside,
            )
            .with_text(
                Text::new().push_segment(TextSegment::new("Toggle Fullscreen", Colour::WHITE)),
            ),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1f32))),
                height_constraint: Some(SpanConstraint::FitContents),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::new(10f32, 0f32, 0f32, 0f32),
                )),
                ..Default::default()
            },
        );

        let exit_button = widget_store.add(
            Button::new(
                UiMessage::Exit,
                // UiMessage::BaseSceneMessage(BaseSceneMessage::ChangeScene(SceneIdentifier::TextDemo)),
                zui::named_colours::EcstaticRed,
                // zui::named_colours::FabulousFuchsia,
                zui::named_colours::FabulousFuchsia,
            )
            .with_text(Text::new().push_segment(TextSegment::new("Exit Demo App", Colour::WHITE))),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1f32))),
                height_constraint: Some(SpanConstraint::FitContents),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::NONE,
                )),
                ..Default::default()
            },
        );

        let text_demo_button = widget_store.add(
            Button::new(
                UiMessage::BaseSceneMessage(BaseSceneMessage::ChangeScene(
                    SceneIdentifier::TextDemo,
                )),
                zui::named_colours::BonneNuit,
                zui::named_colours::Botanical,
            )
            .with_text(Text::new().push_segment(TextSegment::new("Text Demo", Colour::WHITE))),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1f32))),
                height_constraint: Some(SpanConstraint::FitContents),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::NONE,
                )),
                ..Default::default()
            },
        );

        let info_text = widget_store.add(
            TextContainer::from(
                "Press keys 1-3 to swap the active scene. 'a' will add text to this sidebar, and \
                'd' will delete it. Press 'x' to exit!",
            ),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1f32))),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::NONE,
                )),
                ..Default::default()
            },
        );

        let frame_time_text = widget_store.add(
            TextContainer::from("Frame time: N/A"),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1f32))),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::NONE,
                )),
                ..Default::default()
            },
        );

        let display_area_background = widget_store.add(
            Container::new().with_background(Some(ContainerBackground::Colour(
                zui::named_colours::Corbeau,
            ))),
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
            Container::new().with_background(Some(ContainerBackground::Colour(
                zui::named_colours::CapitalBlue,
            ))),
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
                Text::new()
                    .push_segment(TextSegment::new(
                        "DemoSceneTextClippingTest!DemoSceneTextClippingTest!DemoSceneTextClippingT\
                        est!DemoSceneTextClippingTest!DemoSceneTextClippingTest!DemoSceneTextClippi\
                        ngTest!DemoSceneTextClippingTest!",
                        Colour::WHITE,
                    ))
                    .with_configuration(TextConfiguration {
                        line_wrapping: zui::LineWrapping::None,
                        horizontal_alignment: zui::TextAlignmentHorizontal::Centre,
                        vertical_alignment: zui::text::TextAlignmentVertical::Centre,
                        ..Default::default()
                    }),
            ),
            EntryOverrideDescriptor {
                width_constraint: Some(SpanConstraint::ParentWidth(ParentWidth::new(1f32))),
                height_constraint: Some(SpanConstraint::ParentHeight(ParentHeight::new(1f32))),
                position_constraint: Some(PositionConstraint::ParentDetermined(
                    PaddingWeights::NONE,
                )),
                ..Default::default()
            },
        );

        let cursor = widget_store.add(
            Container::new().with_background(Some(ContainerBackground::Colour(
                zui::named_colours::LemonGrass,
            ))),
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
        self.frame_time_id = Some(frame_time_text);

        _ = widget_store.widget_add_child(&sidebar, frame_time_text);
        _ = widget_store.widget_add_child(&sidebar, info_text);

        _ = widget_store.widget_add_child(&sidebar, container_demo_button);
        _ = widget_store.widget_add_child(&sidebar, text_demo_button);
        _ = widget_store.widget_add_child(&sidebar, button_demo_button);

        self.demo_navigation_buttons.push(container_demo_button);
        self.demo_navigation_buttons.push(text_demo_button);
        self.demo_navigation_buttons.push(button_demo_button);

        _ = widget_store.widget_add_child(&sidebar, toggle_fullscreen_button);
        _ = widget_store.widget_add_child(&sidebar, exit_button);

        _ = widget_store.widget_add_child(&root, sidebar);
        _ = widget_store.widget_add_child(&root, display_area_background);
        _ = widget_store.widget_add_child(&root, cursor);

        _ = widget_store.widget_add_child(&display_area, text_none);
        _ = widget_store.widget_add_child(&display_area_background, display_area);

        root
    }
}
