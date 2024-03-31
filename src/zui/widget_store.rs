mod entry;

pub use entry::{Entry, EntryChildren, EntryDefaultDescriptor, EntryOverrideDescriptor};
use rustc_hash::FxHashSet;

use crate::{
    typeface::SymbolKey, zui::span_constraint::IntoPixelSpan, Axis, Context, PaddingWeights,
    PositionConstraint, Rectangle, SpanConstraint, Widget,
};

use super::{
    primitives::Dimensions,
    render_layer::RenderLayer,
    widget::{Boundary, BoundaryType, LayoutBoundaries, OverflowState, PlacementInfo},
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
            placement_info: PlacementInfo::new(),
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

    /// Deletes an entry, returns Err(()) if the entry was already deleted/does not exist
    pub fn delete(&mut self, widget_id: &WidgetId) -> Result<(), ()> {
        match self.widgets.get_mut(widget_id.index) {
            Some(we) => {
                *we = None;
                Ok(())
            }
            None => Err(()),
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
    pub fn collect_text(&self) -> FxHashSet<SymbolKey> {
        let mut symbol_keys = FxHashSet::default();
        for entry in self.iter() {
            entry.widget.collect_text(&mut symbol_keys);
        }
        symbol_keys
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
        render_layers: &mut Vec<RenderLayer>,
        current_layer_index: usize,
        translation_px: glam::IVec2,
    ) -> Result<(), ()> {
        let entry = match self.get(root_widget_id) {
            Some(we) => we,
            None => {
                warn!("failed to find widget with id: {root_widget_id}!");
                return Err(());
            }
        };

        let region = match entry.placement_info.clip_rectangle_px {
            Some(region) => region,
            None => {
                warn!("widget {root_widget_id} has no clip_rectangle_px!");
                return Err(());
            }
        };

        // if the current widget is overflowing, creates a new layer and sets the children to render
        // to the new layer.
        let (children_layer_index, children_translation) =
            match entry.children.as_ref().map(|c| &c.overflow_state) {
                Some(OverflowState::None) => (current_layer_index, translation_px),
                Some(OverflowState::Overflowing { translation, .. }) => {
                    let translation = *translation;
                    if entry
                        .placement_info
                        .clip_rectangle_px
                        .is_some_and(|crpx| crpx.has_non_zero_area())
                    {
                        // creating a new render layer
                        let index = render_layers.len();
                        render_layers.push(RenderLayer::new(Some(region)));

                        // info!("translation_px: {translation_px:?}, sum: {}", translation_px + translation);

                        (index, translation_px + translation)
                    } else {
                        (current_layer_index, translation_px + translation)
                    }
                }
                None => (current_layer_index, translation_px),
            };

        // renders current widget on the same layer as its parents, allowing children to be on an
        // entirely new layer. TODO: think of a good reason for this, as opposed to putting the
        // widget on the same layer as its children.
        let parent_render_layer = &mut render_layers[current_layer_index];
        let parent_simple_vertices = &mut parent_render_layer.simple_vertices;
        let parent_text_vertices = &mut parent_render_layer.text_vertices;

        let parent_simple_vertices_len = parent_simple_vertices.len();
        let parent_text_vertices_len = parent_text_vertices.len();

        entry.widget.to_vertices(
            region,
            context,
            parent_simple_vertices,
            parent_text_vertices,
        );

        // updating the vertex positions of the widget using the translation
        for simple_vert in &mut parent_simple_vertices[parent_simple_vertices_len..] {
            simple_vert.translate_by_pixels(translation_px, context.viewport_dimensions_px);
        }

        for text_vert in &mut parent_text_vertices[parent_text_vertices_len..] {
            text_vert.translate_by_pixels(translation_px, context.viewport_dimensions_px);
        }

        // generating child vertices
        let child_ids = match entry.children.as_ref() {
            Some(children) => &children.ids,
            None => {
                return Ok(());
            }
        };

        for child_id in child_ids {
            _ = self.widget_to_vertices(
                child_id,
                context,
                render_layers,
                children_layer_index,
                children_translation,
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

    /// Places all the widgets of a widget tree, based on the given root widget.
    pub fn place_widgets(
        &mut self,
        root_widget_id: &WidgetId,
        context: &Context,
    ) -> Result<(), ()> {

        // getting the main viewport's layout boundaries
        let layout_boundaries = LayoutBoundaries {
            horizontal: Boundary::new(
                BoundaryType::Static,
                context.viewport_dimensions_px.width as i32,
            ),
            vertical: Boundary::new(
                BoundaryType::Static,
                context.viewport_dimensions_px.height as i32,
            ),
        };

        // clearing all layouts
        self.clear_all_placement_info();

        // calculating the child dimensions
        let dimensions = self.widget_try_update_minimum_dimensions(
            &root_widget_id,
            &layout_boundaries,
            context,
        );

        // getting the region of the entire viewport
        let region = Rectangle::new(
            0i32,
            dimensions.width,
            context.viewport_dimensions_px.height as i32 - dimensions.height,
            context.viewport_dimensions_px.height as i32,
        );

        // starting the recursive widget_place calls
        self.widget_place(root_widget_id, region, context)
    }

    /// Places a widget and then all of its child widgets too
    fn widget_place(
        &mut self,
        widget_id: &WidgetId,
        region: Rectangle<i32>,
        context: &Context,
    ) -> Result<(), ()> {
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

        // allowing widget to update its internals
        entry.widget.place(&region, context);

        entry.placement_info.clip_rectangle_px = Some(region);

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

        // important: updating children's layouts if this wasn't already done initially
        let layout_boundaries = LayoutBoundaries::new(
            Boundary::new(BoundaryType::Static, region.width()),
            Boundary::new(BoundaryType::Static, region.height()),
        );
        for child_id in child_ids.iter() {
            self.widget_try_update_minimum_dimensions(child_id, &layout_boundaries, context);
        }

        // calculating the number of pixels that each SpanConstraint::ParentWeight(1f32) will use
        let pixels_per_parent_weight =
            Self::calculate_pixels_per_parent_weight(self, &region, axis, &child_ids);

        // The cursor that tracks the placement of the children widgets
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
            let mut child_dimensions = match child_entry.placement_info.minimum_dimensions_px {
                Some(cd) => cd,
                None => {
                    warn!("child with id: {child_id} has no dimensions!");
                    continue;
                }
            };

            // the pixels perpendicular to the axis used
            let perpendicular_pixels_per_parent_weight =
                Self::calculate_pixels_per_parent_weight(self, &region, axis.other(), &[child_id]);

            // setting for SpanConstraint::ParentWeight for height constraint
            match (axis, child_entry.height_constraint) {
                (Axis::Vertical, SpanConstraint::ParentWeight(pw)) => {
                    child_dimensions.height = (pw * pixels_per_parent_weight).round() as i32;
                }
                (Axis::Horizontal, SpanConstraint::ParentWeight(pw)) => {
                    child_dimensions.height =
                        (pw * perpendicular_pixels_per_parent_weight).round() as i32;
                }
                _ => {}
            };

            // setting for SpanConstraint::ParentWeight for width constraint
            match (axis, child_entry.width_constraint) {
                (Axis::Vertical, SpanConstraint::ParentWeight(pw)) => {
                    child_dimensions.width =
                        (pw * perpendicular_pixels_per_parent_weight).round() as i32;
                }
                (Axis::Horizontal, SpanConstraint::ParentWeight(pw)) => {
                    child_dimensions.width = (pw * pixels_per_parent_weight).round() as i32;
                }
                _ => {}
            };

            // setting the children offsets, ie the offset of the widget caused by the
            // PaddingWeights perpendicular to the parent axis
            let child_offset_px = match axis {
                Axis::Vertical => {
                    let horizontal_offset = match child_entry.position_constraint {
                        PositionConstraint::ParentDetermined(pw) => {
                            (pw.left * perpendicular_pixels_per_parent_weight) as i32
                        }
                        PositionConstraint::Floating(_, _) => 0i32,
                    };

                    glam::IVec2::new(horizontal_offset, 0i32)
                }
                Axis::Horizontal => {
                    let vertical_offset = match child_entry.position_constraint {
                        PositionConstraint::ParentDetermined(pw) => {
                            -((pw.top * perpendicular_pixels_per_parent_weight) as i32)
                        }
                        PositionConstraint::Floating(_, _) => 0i32,
                    };

                    glam::IVec2::new(0i32, vertical_offset)
                }
            };

            // getting child padding weights, zero if none, ie PositionConstraint::Floating
            let child_padding_weights = match child_entry.position_constraint {
                PositionConstraint::ParentDetermined(pw) => pw,
                PositionConstraint::Floating(_, _) => PaddingWeights::NONE,
            };

            // incrementing cursor by start padding offset
            match axis {
                Axis::Vertical => {
                    cursor.y -= child_padding_weights.top * pixels_per_parent_weight;
                }
                Axis::Horizontal => {
                    cursor.x += child_padding_weights.left * pixels_per_parent_weight;
                }
            }

            // determining child region based on position constraint
            let child_region = match child_entry.position_constraint {
                PositionConstraint::ParentDetermined(_) => {
                    let child_region_min_x = cursor.x.round() as i32 + child_offset_px.x;
                    let child_region_max_y = cursor.y.round() as i32 + child_offset_px.y;

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
                    // let child_half_width = child_dimensions.width / 2i32;
                    // let child_half_height = child_dimensions.height / 2i32;

                    // Rectangle::new(
                    //     x - child_half_width,
                    //     // has to be done like this as odd number widths/heights will have a pixel
                    //     // dropped when dividing them by two
                    //     x - child_half_width + child_dimensions.width,
                    //     y - child_half_height,
                    //     // has to be done like this as odd number widths/heights will have a pixel
                    //     // dropped when dividing them by two
                    //     y - child_half_height + child_dimensions.height,
                    // )

                    Rectangle::new(
                        x,
                        // has to be done like this as odd number widths/heights will have a pixel
                        // dropped when dividing them by two
                        x + child_dimensions.width,
                        y,
                        // has to be done like this as odd number widths/heights will have a pixel
                        // dropped when dividing them by two
                        y + child_dimensions.height,
                    )
                }
            };

            // incrementing cursor by end padding offset
            match axis {
                Axis::Vertical => {
                    cursor.y -= child_padding_weights.bottom * pixels_per_parent_weight;
                }
                Axis::Horizontal => {
                    cursor.x += child_padding_weights.right * pixels_per_parent_weight;
                }
            }

            // placing the child
            _ = self.widget_place(&child_id, child_region, context);
        }

        // setting the overflowing flag if the cursor is past the end of the parent region
        let overflowing = match axis {
            Axis::Vertical => (cursor.y as i32) < region.y_min,
            Axis::Horizontal => cursor.x as i32 > region.x_max,
        };

        let entry = match self.get_mut(widget_id) {
            Some(we) => we,
            None => {
                warn!("failed to find widget with id: {widget_id}!");
                return Err(());
            }
        };

        if let Some(entry_children) = &mut entry.children {
            entry_children.overflow_state.update_state(
                overflowing,
                Dimensions::new(
                    cursor.x as i32 - region.x_min,
                    region.y_max - cursor.y as i32,
                ),
            );
        }

        Ok(())
    }

    /// Calculates the number of pixels per SpanConstraint::ParentWeight, given the parent axis,
    /// child ids, parent entry. Returns f32 as can be less than a pixel.
    fn calculate_pixels_per_parent_weight(
        &self,
        parent_region: &Rectangle<i32>,
        parent_axis: Axis,
        child_ids: &[WidgetId],
    ) -> f32 {
        // the number of pixels in the parent rectangle/region
        let parent_span = parent_region.span_by_axis(parent_axis);

        // the number of pixels used up by the parent widget's children
        let (children_span, children_weights) =
            self.children_sum_span_and_weights(parent_axis, &child_ids);

        // calculating the remaining pixels
        let free_pixels = if parent_span - children_span < 0 {
            0i32
        } else {
            parent_span - children_span
        };

        // getting final pixels per parent weight
        let pixels_per_parent_weight = if children_weights == 0f32 {
            0f32
        } else {
            free_pixels as f32 / children_weights
        };

        pixels_per_parent_weight
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

            let minimum_dimensions = match entry.placement_info.minimum_dimensions_px {
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

    /// Clears all layouts, Note: except for overflowing information
    pub fn clear_all_placement_info(&mut self) {
        for entry_opt in self.widgets.iter_mut() {
            let entry = match entry_opt {
                Some(entry) => entry,
                None => continue,
            };

            entry.placement_info = PlacementInfo::new();
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
        match entry.placement_info.minimum_dimensions_px {
            Some(min_dims) => return min_dims,
            None => {}
        }

        let width_constraint = entry.width_constraint;
        let height_constraint = entry.height_constraint;

        // figuring out if either the width or height constraint requires calculation of the
        // children's dimensions
        let children_dimensions = if matches!(width_constraint, SpanConstraint::FitChildren)
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
                        width_counter += child_dims.width;
                        height_counter = height_counter.max(child_dims.height);
                    }
                };
            }

            Dimensions::new(width_counter, height_counter)
        } else {
            Dimensions::new(0, 0)
        };

        // getting the WidgetEntry again..
        let entry = match self.get_mut(widget_id) {
            Some(we) => we,
            None => {
                warn!("failed to find widget with id: {widget_id}!");
                return Dimensions::new(0i32, 0i32);
            }
        };

        // TODO: check all the i32 casts
        let boundary_width_px = match width_constraint {
            SpanConstraint::ViewWidth(vw) => vw.into_pixel_span(context).round() as i32,
            SpanConstraint::ViewHeight(vh) => vh.into_pixel_span(context).round() as i32,
            SpanConstraint::ViewMin(vm) => vm.into_pixel_span(context).round() as i32,
            SpanConstraint::Pixels(p) => p.round() as i32,
            SpanConstraint::FitContents => layout_boundaries.horizontal.span_px,
            SpanConstraint::FitChildren => children_dimensions.width,
            // Silently pass through, is overidden during placement
            SpanConstraint::ParentWeight(_) => 0i32,
            SpanConstraint::ParentWidth(pw) => pw.into_pixel_span(layout_boundaries.into()) as i32,
            SpanConstraint::ParentHeight(ph) => ph.into_pixel_span(layout_boundaries.into()) as i32,
            SpanConstraint::Aspect(_) => layout_boundaries.horizontal.span_px,
        };

        let boundary_height_px = match height_constraint {
            SpanConstraint::ViewWidth(vw) => vw.into_pixel_span(context).round() as i32,
            SpanConstraint::ViewHeight(vh) => vh.into_pixel_span(context).round() as i32,
            SpanConstraint::ViewMin(vm) => vm.into_pixel_span(context).round() as i32,
            SpanConstraint::Pixels(p) => p.round() as i32,
            SpanConstraint::FitContents => layout_boundaries.vertical.span_px,
            SpanConstraint::FitChildren => children_dimensions.height,
            // Silently pass through, is overidden during placement
            SpanConstraint::ParentWeight(_) => 0i32,
            SpanConstraint::ParentWidth(pw) => pw.into_pixel_span(layout_boundaries.into()) as i32,
            SpanConstraint::ParentHeight(ph) => ph.into_pixel_span(layout_boundaries.into()) as i32,
            SpanConstraint::Aspect(_) => layout_boundaries.vertical.span_px,
        };

        // getting contents dimensions if fits contents
        let contents_dimensions = entry.widget.calculate_minimum_dimensions(
            &LayoutBoundaries::new(
                Boundary::new(BoundaryType::Static, boundary_width_px),
                Boundary::new(BoundaryType::Static, boundary_height_px),
            ),
            context,
        );

        // applying overrides if is FitContents
        let mut width_px = match width_constraint {
            SpanConstraint::FitContents => contents_dimensions.width,
            _ => boundary_width_px,
        };

        let mut height_px = match height_constraint {
            SpanConstraint::FitContents => contents_dimensions.height,
            _ => boundary_height_px,
        };

        // applying aspect overrides
        match width_constraint {
            SpanConstraint::Aspect(aspect) => {
                width_px = (height_px as f32 * aspect).round() as i32;
            }
            _ => {}
        };
        match height_constraint {
            SpanConstraint::Aspect(aspect) => {
                height_px = (width_px as f32 * aspect).round() as i32;
            }
            _ => {}
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
        entry.placement_info.minimum_dimensions_px = Some(dims);

        // returning
        dims
    }

    /// Deletes the children of the widget from the widget store, and unparents the deleted
    /// children. This functions recursively.
    pub fn widget_delete_children(&mut self, widget_id: &WidgetId) -> Result<(), ()> {
        // getting the WidgetEntry
        let entry = match self.get_mut(widget_id) {
            Some(we) => we,
            None => {
                warn!("failed to find widget with id: {widget_id}!");
                return Err(());
            }
        };

        let child_ids = match entry.children.as_mut() {
            Some(children) => {
                let ci = children.ids.clone();
                children.ids.clear();
                ci
            }
            None => return Ok(()),
        };

        for child_id in child_ids {
            _ = self.widget_delete_children(&child_id);
            _ = self.delete(&child_id);
        }

        Ok(())
    }

    // Scrolls the overflowing widgets that are under the cursor. Note: this is a naiive approach
    // that queries every widget entry
    pub fn scroll_under_cursor(&mut self, context: &Context, scroll_translation: glam::IVec2) {
        let Some(cursor_position) = context.cursor_position else {
            return;
        };

        for widget_entry_opt in self.widgets.iter_mut() {
            let Some(widget_entry) = widget_entry_opt else {
                continue;
            };

            let Some(clip_rectangle) = widget_entry.placement_info.clip_rectangle_px else {
                continue;
            };

            let Some(entry_children) = &mut widget_entry.children else {
                continue;
            };

            let OverflowState::Overflowing {
                translation: current_translation,
                children_dimensions,
            } = &mut entry_children.overflow_state
            else {
                continue;
            };

            if !clip_rectangle.contains_position(&cursor_position) {
                continue;
            }

            let new_translation = *current_translation + scroll_translation;

            current_translation.x = new_translation.x.clamp(
                0i32,
                i32::max(0i32, children_dimensions.width - clip_rectangle.width()),
            );
            current_translation.y = new_translation.y.clamp(
                0i32,
                i32::max(0i32, children_dimensions.height - clip_rectangle.height()),
            );
        }
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
