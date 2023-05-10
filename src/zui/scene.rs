use super::Widget;

pub trait Scene {
    type Message: Copy + Clone;

    /// Implements the update logic when a new message is received from a widget,
    /// Returns an external message, ie a command that is to be handled by the external context
    /// and a boolean that shows whether or not a scene rebuild is required
    fn handle_message(&mut self, message: Self::Message) -> (Option<Self::Message>, bool);

    /// Returns the root widget of the scene, window's aspect ratio included for user conveninence
    fn view(&self, aspect_ratio: f32) -> Widget<Self::Message>
    where
        <Self as Scene>::Message: std::marker::Copy;
}
