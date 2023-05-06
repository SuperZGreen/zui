use super::Widget;

pub trait Scene {
    type Message: Copy + Clone;

    /// Implements the update logic when a new message is received,
    /// Returns true if a rebuild (call of view function) is required
    fn handle_message(&mut self, message: Self::Message) -> bool;

    /// Returns the root widget of the scene, window's aspect ratio included for user conveninence
    fn view(&self, aspect_ratio: f32) -> Widget<Self::Message>
    where
        <Self as Scene>::Message: std::marker::Copy;
}
