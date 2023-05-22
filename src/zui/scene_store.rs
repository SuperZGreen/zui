use rustc_hash::FxHashMap;
use std::fmt::Debug;
use std::hash::Hash;

use super::{Scene, SceneHandle, Context};

/// A registry of all scenes identified by SceneIdentifiers, as well as containing the previous scene
pub struct SceneStore<SceneIdentifier, Message>
where
    Message: Copy + Clone,
    SceneIdentifier: Eq + Hash + Debug,
{
    current_scene_identifier: Option<SceneIdentifier>,

    scenes: FxHashMap<SceneIdentifier, SceneHandle<Message>>,
}

impl<SceneIdentifier, Message> SceneStore<SceneIdentifier, Message>
where
    Message: Copy + Clone,
    SceneIdentifier: Eq + Hash + Debug,
{
    /// Creates a new SceneStore
    pub fn new() -> Self {
        Self {
            current_scene_identifier: None,
            scenes: FxHashMap::default(),
        }
    }

    /// Adds a scene to the SceneStore
    pub fn add_scene(
        &mut self,
        scene_identifier: SceneIdentifier,
        scene: Box<dyn Scene<Message = Message>>,
    ) {
        self.scenes
            .insert(scene_identifier, SceneHandle::new(scene));
    }

    /// Changes the current scene of the SceneStore
    pub fn change_scene(&mut self, scene_identifier: SceneIdentifier, context: &Context) -> Result<(), ()> {
        match self.scenes.get_mut(&scene_identifier) {
            Some(scene_handle) => {
                self.current_scene_identifier = Some(scene_identifier);
                // scene_handle.queue_widget_recreation();
                scene_handle.rebuild_scene(context);
                Ok(())
            }
            None => {
                println!("Could not find requested scene: {:?}", scene_identifier);
                Err(())
            }
        }
    }

    pub fn current_scene_mut(&mut self) -> Option<&mut SceneHandle<Message>> {
        if let Some(current_scene_identitifer) = &self.current_scene_identifier {
            self.scenes.get_mut(current_scene_identitifer)
        } else {
            warn!("no current scene set!");
            None
        }
    }

    pub fn current_scene(&mut self) -> Option<&SceneHandle<Message>> {
        if let Some(current_scene_identitifer) = &self.current_scene_identifier {
            self.scenes.get(current_scene_identitifer)
        } else {
            warn!("no current scene set!");
            None
        }
    }
}
