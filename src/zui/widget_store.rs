mod entry;

pub use entry::{Entry, EntryChildren, EntryDefaultDescriptor, EntryOverrideDescriptor};
use rustc_hash::FxHashSet;

use std::collections::VecDeque;

use crate::{
    typeface::SymbolKey, zui::span_constraint::IntoPixelSpan, Axis,
    Context, PositionConstraint, Rectangle, SpanConstraint, Widget,
};

use super::{
    primitives::Dimensions,
    render_layer::RenderLayer,
    simple_renderer::SimpleVertex,
    text_renderer::TextVertex,
    widget::{Boundary, BoundaryType, Layout, LayoutBoundaries},
};

/// The ID of a Widget that is stored in the WidgetStore, this is used to access the corresponding
/// WidgetEntry for the desired Widget
#[derive(Copy, Clone)]
pub struct WidgetId {
    index: usize,
}

impl std::fmt::Display for WidgetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.index)
    }
}

/// Holds the widget state for a scene
pub struct WidgetStore<Message> {
    widgets: Vec<Option<Entry<Message>>>,
}

impl<Message> WidgetStore<Message> {
    /// Creates a new empty WidgetStore
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
        }
    }

    /// Adds a Widget to the WidgetStore, returns the WidgetId to fetch the widget later
    pub fn add(
        &mut self,
        widget: impl Into<Box<dyn Widget<Message>>>,
        entry_override_descriptor: EntryOverrideDescriptor,
    ) -> WidgetId {
        let widget = widget.into();

        let mut entry_descriptor = widget.entry_default_descriptor();
        entry_descriptor.apply_overrides(entry_override_descriptor);

        // the widget entry that will be inserted
        let widget_entry = Some(Entry {
            widget,
            layout: Layout::new(),
            children: entry_descriptor.children,
            width_constraint: entry_descriptor.width_constraint,
            height_constraint: entry_descriptor.height_constraint,
            position_constraint: entry_descriptor.position_constraint,
        });

        // trying to get the index of a free entry
        match Self::get_available_entry_index(&self.widgets) {
            Some(available_index) => {
                *self.widgets.get_mut(available_index).unwrap() = widget_entry;
                WidgetId {
                    index: available_index,
                }
            }
            None => {
                let index = self.widgets.len();
                self.widgets.push(widget_entry);
                WidgetId { index }
            }
        }
    }

    /// Gets a reference to the WidgetEntry
    pub fn get(&self, widget_id: &WidgetId) -> Option<&Entry<Message>> {
        match self.widgets.get(widget_id.index) {
            Some(widget_entry_opt) => widget_entry_opt.as_ref(),
            None => None,
        }
    }

    /// Gets a mutable reference to the WidgetEntry
    pub fn get_mut(&mut self, widget_id: &WidgetId) -> Option<&mut Entry<Message>> {
        match self.widgets.get_mut(widget_id.index) {
            Some(widget_entry_opt) => widget_entry_opt.as_mut(),
            None => None,
        }
    }

    /// Removes an entry, returns Err(()) if the entry was already removed/does not exist
    pub fn remove(&mut self, widget_id: &WidgetId) -> Result<(), ()> {
        match self.widgets.get_mut(widget_id.index) {
            Some(we) => {
                *we = None;
                Ok(())
            }
            None => {
                return Err(());
            }
        }
    }

    /// Iterates through the entries
    pub fn iter(&self) -> WidgetStoreIter<Message> {
        WidgetStoreIter {
            widgets: &self.widgets,
            index: 0,
        }
    }

    /// Iterates through the entries
    pub fn iter_mut(&mut self) -> WidgetStoreIterMut<Message> {
        WidgetStoreIterMut {
            len: self.widgets.len(),
            widgets: &mut self.widgets,
            index: 0,
        }
    }

    /// Gets a free entry (ie. allocated WidgetEntry that is not in use, returns None if there are
    /// no free entries, and a new entry will have to be pushed
    fn get_available_entry_index(widgets: &Vec<Option<Entry<Message>>>) -> Option<usize> {
        for (index, widget_entry) in widgets.iter().enumerate() {
            if widget_entry.is_none() {
                return Some(index);
            }
        }
        None
    }

    /// Collects the text SymbolKeys of all Widgets in the provided FxHashSet
    pub fn collect_text(&self, symbol_keys: &mut FxHashSet<SymbolKey>) {
        for entry in self.iter() {
            entry.widget.collect_text(symbol_keys);
        }
    }

    //
    //  Widget handling
    //

    // /// Handles event for the given Widget
    // pub fn widget_handle_event(&mut self, widget_id: &WidgetId, event: &Event, context: &Context) {
    //     todo!()
    // }

    /// Sets the dimensions of the given Widget
    pub fn widget_set_dimensions(
        &mut self,
        _widget_id: &WidgetId,
        _dimensions_px: Option<Dimensions<f32>>,
    ) -> Result<(), ()> {
        todo!()
    }

    /// Sets a child Widget for the widget
    pub fn widget_add_child(
        &mut self,
        parent_widget_id: &WidgetId,
        child_widget_id: WidgetId,
    ) -> Result<(), ()> {
        if parent_widget_id.index == child_widget_id.index {
            error!("can not set widget child to self!");
            return Err(());
        }

        let entry = match self.get_mut(parent_widget_id) {
            Some(we) => we,
            None => {
                warn!("failed to find widget with id: {parent_widget_id}!");
                return Err(());
            }
        };

        match entry.children.as_mut() {
            Some(children) => {
                children.ids.push(child_widget_id);
                Ok(())
            }
            None => {
                warn!("Failed to push child for widget: {parent_widget_id}, widget accepts no children!");
                Err(())
            }
        }
    }

    /// Unsets a child Widget for the widget
    pub fn widget_remove_child(
        &mut self,
        _parent_widget_id: &WidgetId,
        _child_widget_id: WidgetId,
    ) -> Result<(), ()> {
        todo!()
    }

    /// Builds the vertices by tree, by the root widget id
    pub fn widget_to_vertices(
        &self,
        root_widget_id: &WidgetId,
        context: &Context,
        simple_vertices: &mut Vec<SimpleVertex>,
        text_vertices: &mut Vec<TextVertex>,
        render_layers: &mut VecDeque<RenderLayer>,
    ) -> Result<(), ()> {
        let entry = match self.get(root_widget_id) {
            Some(we) => we,
            None => {
                warn!("failed to find widget with id: {root_widget_id}!");
                return Err(());
            }
        };

        let region = match entry.layout.clip_rectangle_px {
            Some(region) => region,
            None => {
                warn!("widget {root_widget_id} has no clip_rectangle_px!");
                return Err(());
            }
        };

        entry.widget.to_vertices(
            region,
            context,
            simple_vertices,
            text_vertices,
            render_layers,
        );

        let child_ids = match entry.children.as_ref() {
            Some(children) => &children.ids,
            None => {
                return Ok(());
            }
        };

        drop(entry);

        for child_id in child_ids {
            _ = self.widget_to_vertices(
                child_id,
                context,
                simple_vertices,
                text_vertices,
                render_layers,
            );
        }

        Ok(())
    }

    /// Sets the horizontal SpanConstraint for a WidgetEntry
    pub fn widget_set_width_contraint(
        &mut self,
        widget_id: &WidgetId,
        width_constraint: SpanConstraint,
    ) -> Result<(), ()> {
        let entry = match self.get_mut(widget_id) {
            Some(we) => we,
            None => {
                warn!("failed to find widget with id: {widget_id}!");
                return Err(());
            }
        };

        entry.width_constraint = width_constraint;

        Ok(())
    }

    /// Sets the vertical SpanConstraint for a WidgetEntry
    pub fn widget_set_height_contraint(
        &mut self,
        widget_id: &WidgetId,
        height_constraint: SpanConstraint,
    ) -> Result<(), ()> {
        let entry = match self.get_mut(widget_id) {
            Some(we) => we,
            None => {
                warn!("failed to find widget with id: {widget_id}!");
                return Err(());
            }
        };

        entry.height_constraint = height_constraint;

        Ok(())
    }

    /// Sets the PositionConstraint of the Widget
    pub fn widget_set_position_constraint(
        &mut self,
        widget_id: &WidgetId,
        position_constraint: PositionConstraint,
    ) -> Result<(), ()> {
        let entry = match self.get_mut(widget_id) {
            Some(we) => we,
            None => {
                warn!("failed to find widget with id: {widget_id}!");
                return Err(());
            }
        };

        entry.position_constraint = position_constraint;

        Ok(())
    }

    /// Calculates the total used span of the children along the parent's axis, not including
    /// Span::ParentWeights of course! Returns (span, weights)
    fn children_sum_span_and_weights(&self, axis: Axis, child_ids: &[WidgetId]) -> (i32, f32) {
        let mut span = 0i32;
        let mut weights = 0f32;
        for child_id in child_ids {
            let entry = match self.get(child_id) {
                Some(we) => we,
                None => {
                    warn!("failed to find widget with id: {child_id}!");
                    continue;
                }
            };

            // only summing for ParentDetermined children
            match entry.position_constraint {
                PositionConstraint::ParentDetermined(_) => {}
                _ => continue,
            };

            let minimum_dimensions = match entry.layout.minimum_dimensions_px {
                Some(min_dims) => min_dims,
                None => {
                    warn!("could not find layout for widget with id: {child_id}!");
                    continue;
                }
            };

            span += minimum_dimensions.span_by_axis(axis);

            // adding widget's weight
            let axis_constraint = match axis {
                Axis::Vertical => entry.height_constraint,
                Axis::Horizontal => entry.width_constraint,
            };

            let parent_weight = match axis_constraint {
                SpanConstraint::ParentWeight(pw) => pw,
                _ => 0f32,
            };

            weights += parent_weight;

            // adding widget's padding weight
            let padding_weights = match entry.position_constraint {
                PositionConstraint::ParentDetermined(pw) => pw.sum_by_axis(axis),
                _ => 0f32,
            };

            weights += padding_weights;
        }

        (span, weights)
    }

    /// Places a widget and all of its child widgets too
    pub fn widget_place(&mut self, widget_id: &WidgetId, region: Rectangle<i32>, context: &Context) -> Result<(), ()> {
        // getting the entry
        let entry = match self.get_mut(widget_id) {
            Some(we) => we,
            None => {
                warn!("failed to find widget with id: {widget_id}!");
                return Err(());
            }
        };

        //
        // placing self
        //

        entry.layout.clip_rectangle_px = Some(region);

        //
        //  Placing children
        //

        // getting children, early exit if widget does not accept children
        let (axis, child_ids) = match entry.children.as_ref() {
            Some(children) => (children.axis, children.ids.clone()),
            None => {
                return Ok(());
            }
        };

        drop(entry);

        // important: updating children's layouts if this wasn't already done initially
        let layout_boundaries = LayoutBoundaries::new(
            Boundary::new(BoundaryType::Static, region.width()),
            Boundary::new(BoundaryType::Static, region.height()),
        );
        for child_id in child_ids.iter() {
            self.widget_try_update_minimum_dimensions(child_id, &layout_boundaries, context);
        }

        // getting the entry again..
        let entry = match self.get_mut(widget_id) {
            Some(we) => we,
            None => {
                warn!("failed to find widget with id: {widget_id}!");
                return Err(());
            }
        };

        // calculating pixels per parent weight
        let free_pixels = entry
            .layout
            .minimum_dimensions_px
            .unwrap()
            .span_by_axis(axis);

        let (children_span, children_weights) =
            self.children_sum_span_and_weights(axis, &child_ids);

        let pixels_per_parent_weight = if children_weights == 0f32 {
            0f32
        } else {
            (free_pixels - children_span) as f32 / children_weights
        };

        let mut cursor = glam::Vec2::new(region.x_min as f32, region.y_max as f32);

        for child_id in child_ids {
            // getting the child entry
            let child_entry = match self.get(&child_id) {
                Some(child) => child,
                None => {
                    error!("could not find child with id: {child_id}!");
                    continue;
                }
            };

            // getting the child dimensions
            let child_dimensions = match child_entry.layout.minimum_dimensions_px {
                Some(cd) => cd,
                None => {
                    warn!("child with id: {child_id} has no dimensions!");
                    continue;
                }
            };

            // getting child start and end offset
            // TODO: needs to change based on axis
            let start_padding_offset = match child_entry.position_constraint {
                PositionConstraint::ParentDetermined(pw) => pw.top * pixels_per_parent_weight,
                _ => 0f32,
            };

            let end_padding_offset = match child_entry.position_constraint {
                PositionConstraint::ParentDetermined(pw) => pw.bottom * pixels_per_parent_weight,
                _ => 0f32,
            };

            // incrementing cursor by start offset
            // TODO: needs to account for axis, subpixel movements etc
            cursor.y -= start_padding_offset;

            // determining child region based on position constraint
            let child_region = match child_entry.position_constraint {
                PositionConstraint::ParentDetermined(_) => {
                    let child_region_min_x = cursor.x.round() as i32;
                    let child_region_max_y = cursor.y.round() as i32;

                    // determining the child region
                    let child_region = Rectangle::new(
                        child_region_min_x,
                        child_region_min_x + child_dimensions.width,
                        child_region_max_y - child_dimensions.height,
                        child_region_max_y,
                    );

                    // incrementing the cursor
                    match axis {
                        Axis::Vertical => {
                            cursor.y -= child_dimensions.height as f32;
                        }
                        Axis::Horizontal => {
                            cursor.x += child_dimensions.width as f32;
                        }
                    }

                    child_region
                }
                PositionConstraint::Floating(x, y) => {
                    let child_half_width = child_dimensions.width / 2i32;
                    let child_half_height = child_dimensions.height / 2i32;

                    Rectangle::new(
                        x - child_half_width,
                        // has to be done like this as odd number widths/heights will have a pixel
                        // dropped when dividing them by two
                        x - child_half_width + child_dimensions.width,
                        y - child_half_height,
                        // has to be done like this as odd number widths/heights will have a pixel
                        // dropped when dividing them by two
                        y - child_half_height + child_dimensions.height,
                    )
                }
            };

            // incrementing cursor by end offset
            // TODO: needs to account for axis, subpixel movements etc
            cursor.y -= end_padding_offset;

            // placing the child
            _ = self.widget_place(&child_id, child_region, context);
        }

        Ok(())
    }

    /// Clears all layouts
    pub fn clear_layouts(&mut self) {
        for entry_opt in self.widgets.iter_mut() {
            let entry = match entry_opt {
                Some(entry) => entry,
                None => continue,
            };

            entry.layout = Layout::new();
        }
    }

    /// Returns the internal dimensions of the widget, and updates it's internal layout. Will call
    /// recursively on its children if uses SpanConstraint::FitChildren. Will also call
    /// Widget::calculate_minimum_dimensions
    pub fn widget_try_update_minimum_dimensions(
        &mut self,
        widget_id: &WidgetId,
        layout_boundaries: &LayoutBoundaries,
        context: &Context,
    ) -> Dimensions<i32> {
        // getting the WidgetEntry
        let entry = match self.get(widget_id) {
            Some(we) => we,
            None => {
                warn!("failed to find widget with id: {widget_id}!");
                return Dimensions::new(0i32, 0i32);
            }
        };

        // early exit if minimum dimensions already calculated
        match entry.layout.minimum_dimensions_px {
            Some(min_dims) => return min_dims,
            None => {}
        }

        let width_constraint = entry.width_constraint;
        let height_constraint = entry.height_constraint;

        // This can be set if widget is SpanConstraint::FitChildren below
        let mut widget_dimensions = Dimensions::new(0, 0);

        // figuring out if either the width or height constraint requires calculation of the
        // children's dimensions
        if matches!(width_constraint, SpanConstraint::FitChildren)
            || matches!(height_constraint, SpanConstraint::FitChildren)
        {
            // cloning the children ids and axis
            let (child_ids, axis) = match entry.children.as_ref() {
                Some(children) => (children.ids.clone(), children.axis),
                None => {
                    warn!("entry has no Children!");
                    return Dimensions::new(0i32, 0i32);
                }
            };

            // getting the cumulate dimensions of the children
            let mut width_counter = 0i32;
            let mut height_counter = 0i32;
            for child_id in child_ids {
                let child_dims = self.widget_try_update_minimum_dimensions(
                    &child_id,
                    &layout_boundaries,
                    context,
                );

                match axis {
                    Axis::Vertical => {
                        width_counter = width_counter.max(child_dims.width);
                        height_counter += child_dims.height;
                    }
                    Axis::Horizontal => {
                        width_counter = width_counter.max(child_dims.width);
                        height_counter += child_dims.height;
                    }
                };
            }

            widget_dimensions = Dimensions::new(width_counter, height_counter);
        }

        // TODO: check all the i32 casts
        let width_px = match width_constraint {
            SpanConstraint::ViewWidth(vw) => vw.into_pixel_span(context).round() as i32,
            SpanConstraint::ViewHeight(vh) => vh.into_pixel_span(context).round() as i32,
            SpanConstraint::ViewMin(vm) => vm.into_pixel_span(context).round() as i32,
            SpanConstraint::Pixels(p) => p.round() as i32,
            SpanConstraint::FitContents => todo!(),
            SpanConstraint::FitChildren => widget_dimensions.width,
            SpanConstraint::ParentWeight(_) => todo!(),
            SpanConstraint::ParentWidth(pw) => pw.into_pixel_span(layout_boundaries.into()) as i32,
            SpanConstraint::ParentHeight(ph) => ph.into_pixel_span(layout_boundaries.into()) as i32,
        };

        let height_px = match height_constraint {
            SpanConstraint::ViewWidth(vw) => vw.into_pixel_span(context).round() as i32,
            SpanConstraint::ViewHeight(vh) => vh.into_pixel_span(context).round() as i32,
            SpanConstraint::ViewMin(vm) => vm.into_pixel_span(context).round() as i32,
            SpanConstraint::Pixels(p) => p.round() as i32,
            SpanConstraint::FitContents => todo!(),
            SpanConstraint::FitChildren => widget_dimensions.width,
            SpanConstraint::ParentWeight(_) => todo!(),
            SpanConstraint::ParentWidth(pw) => pw.into_pixel_span(layout_boundaries.into()) as i32,
            SpanConstraint::ParentHeight(ph) => ph.into_pixel_span(layout_boundaries.into()) as i32,
        };

        // getting final dimensions
        let dims = Dimensions::new(width_px, height_px);

        // getting the WidgetEntry
        let entry = match self.get_mut(widget_id) {
            Some(we) => we,
            None => {
                warn!("failed to find widget with id: {widget_id}!");
                return Dimensions::new(0i32, 0i32);
            }
        };

        // updating self
        entry.layout.minimum_dimensions_px = Some(dims);

        // returning
        dims
    }
}

pub struct WidgetStoreIter<'a, Message> {
    widgets: &'a Vec<Option<Entry<Message>>>,
    index: usize,
}

impl<'a, Message> Iterator for WidgetStoreIter<'a, Message> {
    type Item = &'a Entry<Message>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // getting the current entry
            let entry = self.widgets.get(self.index);

            // incrementing the counter
            self.index += 1;

            match entry {
                Some(Some(we)) => {
                    return Some(we);
                }
                Some(None) => {
                    continue;
                }
                None => {
                    return None;
                }
            }
        }
    }
}

pub struct WidgetStoreIterMut<'a, Message> {
    widgets: &'a mut [Option<Entry<Message>>],
    len: usize,
    index: usize,
}

impl<'a, Message> Iterator for WidgetStoreIterMut<'a, Message> {
    type Item = &'a mut Entry<Message>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // bounds checking
            if self.index >= self.len {
                return None;
            }

            // getting the current entry, using a raw pointer due to aliasing issues with mutable
            // iterators
            let entry: *mut Option<Entry<Message>> = &mut self.widgets[self.index];

            // incrementing the counter
            self.index += 1;

            unsafe {
                let entry_mut: &mut Option<Entry<Message>> = &mut *entry;

                if let Some(we) = entry_mut {
                    return Some(we);
                }
            }
        }
    }
}
