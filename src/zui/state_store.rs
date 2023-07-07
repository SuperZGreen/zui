use core::hash::Hash;
use std::any::Any;
use std::fmt::Debug;

use rustc_hash::FxHashMap;

/// Holds the dynamic Widget state for a scene
pub struct StateStore<StateIdentifier>
where
    StateIdentifier: Eq + Hash + Debug,
{
    hash_map: FxHashMap<StateIdentifier, Box<dyn Any>>,
}

impl<StateIdentifier> StateStore<StateIdentifier>
where
    StateIdentifier: Eq + Hash + Debug,
{
    /// Creates a new, empty StateStore
    pub fn new() -> Self {
        Self {
            hash_map: FxHashMap::default(),
        }
    }

    /// Will insert, overriding the existing entry
    pub fn insert(&mut self, key: StateIdentifier, value: Box<dyn Any>) {
        self.hash_map.insert(key, value);
    }

    /// Will only insert if the entry does not already exist
    pub fn try_insert(&mut self, key: StateIdentifier, value: Box<dyn Any>) {
        self.hash_map.entry(key).or_insert(value);
    }

    /// Gets a reference to the key's value
    pub fn get(&self, key: &StateIdentifier) -> Option<&Box<dyn Any>> {
        self.hash_map.get(&key)
    }

    /// Gets a mutable reference to the key's value
    pub fn get_mut(&mut self, key: &StateIdentifier) -> Option<&mut Box<dyn Any>> {
        self.hash_map.get_mut(&key)
    }

    /// Gets from the StateStore and performs a downcast
    pub fn get_as<T: 'static>(&self, key: &StateIdentifier) -> Option<&T> {
        match self.hash_map.get(&key) {
            Some(da) => {
                match da.downcast_ref::<T>() {
                    Some(t) => Some(t),
                    None => {
                        warn!("failed to downcast T");
                        None
                    }
                }
            }
            None => {
                warn!("StateStore could not get with key: {key:?}");
                None
            }
        }
    }

    /// Gets from the StateStore and performs a downcast mutably
    pub fn get_as_mut<T: 'static>(&mut self, key: &StateIdentifier) -> Option<&mut T> {
        match self.hash_map.get_mut(&key) {
            Some(da) => {
                match da.downcast_mut::<T>() {
                    Some(t) => Some(t),
                    None => {
                        warn!("failed to downcast T");
                        None
                    }
                }
            }
            None => {
                warn!("StateStore could not get with key: {key:?}");
                None
            }
        }
    }

    /// Clears the underlying hash map
    pub fn clear(&mut self) {
        self.hash_map.clear();
    }
}
