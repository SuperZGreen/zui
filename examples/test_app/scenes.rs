mod base;
mod container_demo;
mod text_demo;

pub use base::{BaseScene, BaseSceneMessage};
// pub use container_demo::ContainerScene;
// pub use text_demo::TextScene;

#[derive(Copy, Clone, Debug)]
pub enum SceneIdentifier {
    ContainerDemo,
    TextDemo,
    ButtonDemo,
    FillBarDemo,
}
