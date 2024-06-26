use crate::WidgetStore;

use super::widget_store::WidgetId;

pub trait Scene {
    type Message: Clone;

    /// Implements the update logic when a new message is received from a widget,
    /// Returns an external message, ie a command that is to be handled by the external context
    /// and a boolean that shows whether or not a scene rebuild is required
    fn handle_message(
        &mut self,
        widget_store: &mut WidgetStore<Self::Message>,
        message: Self::Message,
    ) -> (Option<Self::Message>, bool);

    /// Builds the initial scene, loading all of the widgets into the provided WidgetStore, and
    /// typically storing the returned WidgetIds in the Scene implementing struct. The root widget
    /// of the scene is returned, and used to subsequently build the scene
    fn init(&mut self, widget_store: &mut WidgetStore<Self::Message>) -> WidgetId;
}
