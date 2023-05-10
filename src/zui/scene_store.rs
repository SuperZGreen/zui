use rustc_hash::FxHashMap;

use super::{Scene, SceneHandle};

/// A registry of all scenes identified by SceneIdentifiers, as well as containing the previous scene
struct SceneStore<SceneIdentifier, Message> {
    current_scene: Option<SceneIdentifier>,
    // previous_scene: Option<SceneIdentifier>,

    scenes: FxHashMap<SceneIdentifier, Box<dyn Scene<Message = Message>>>,
}

impl<SceneIdentifier, Message> SceneStore<SceneIdentifier, Message>{
    pub fn new() -> Self {
        Self {
            current_scene: None,
            scenes: FxHashMap::default(),
        }
    }
    
    // pub fn change_scene(&mut self) -> SceneHandle<Message> {
    //     // TODO
    //     todo!()
    // }
}
