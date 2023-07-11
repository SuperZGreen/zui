use crate::Rectangle;

/// This is a fast bin packer

/// The input of the packer
pub struct InputItem {
    pub width: u32,
    pub height: u32,
}

pub struct Output {
    /// The positions of the input items in pixels
    pub items: Vec<OutputItem>,

    /// The span of the texture in pixels
    pub span: u32,
}

/// The output of the packer
pub struct OutputItem {
    pub rectangle: Rectangle<u32>,
    pub input_item_index: usize,
}

pub struct Packer {
    /// The largest width of the texture's span/dimension that is acceptable before the packer will
    /// fail. In pixels
    max_span: u32,
    // TODO: padding doesn't really seem neccesary for the text renderer, likely because it's
    // 'perfectly' pixel aligned. This will be nice to have for images in the future though!
    //
    // /// The border of pixels that should be allowed for each packed input item
    // padding: u32,
}

impl Packer {
    /// Creates a new Packer
    pub fn new(max_span: u32) -> Self {
        Self { max_span }
    }

    /// Packs the packer
    pub fn pack(&self, input_items: &[InputItem]) -> Option<Output> {
        // The assumed area that the texture will take
        let naiive_area = Self::calculate_input_items_area(input_items) * 2;

        // The initially assumed span of the texture
        let naiive_span = (naiive_area as f32).sqrt() as u32;

        let standard_span = 256u32;
        let standard_spans_required = naiive_span / 256u32 + 1;

        let initial_span = standard_spans_required * standard_span;
        let mut span = initial_span;

        for _ in 0..5 {
            match self.try_pack(span, input_items) {
                Some(output) => return Some(output),
                None => {
                    warn!("failed to pack with span: {span}, retrying!");
                }
            };

            span *= 2;

            if span > self.max_span {
                warn!("could not pack as span > max_span!");
                return None;
            }
        }

        None
    }

    /// Tries to pack with the given span, returning None if fails
    fn try_pack(&self, span: u32, input_items: &[InputItem]) -> Option<Output> {
        // x-first, y-down
        let mut x_counter = 0u32;
        let mut y_counter = 0u32;
        let mut line_max_height = 0u32;
        let mut output_items = Vec::new();

        for (index, item) in input_items.iter().enumerate() {
            // if happily in the line
            if x_counter + item.width <= span {
                // TODO: IDENTICAL TO LATER
                // adding to output
                output_items.push(OutputItem {
                    rectangle: Rectangle::new(
                        x_counter,
                        x_counter + item.width,
                        y_counter,
                        y_counter + item.height,
                    ),
                    input_item_index: index,
                });

                // incrementing counters
                x_counter += item.width;

                // updating the largest height tracker
                if item.height > line_max_height {
                    line_max_height = item.height;
                }
            }
            // if doesn't fit in the line
            else {
                // TODO
                x_counter = 0u32;
                y_counter += line_max_height;
                line_max_height = 0u32;

                // item will never fit
                if x_counter + item.width > span {
                    return None;
                } else {
                    // TODO: IDENTICAL TO BEFORE
                    // adding to output
                    output_items.push(OutputItem {
                        rectangle: Rectangle::new(
                            x_counter,
                            x_counter + item.width,
                            y_counter,
                            y_counter + item.height,
                        ),
                        input_item_index: index,
                    });

                    // incrementing counters
                    x_counter += item.width;

                    // updating the largest height tracker
                    if item.height > line_max_height {
                        line_max_height = item.height;
                    }
                }
            }
        }

        Some(Output {
            span,
            items: output_items,
        })
    }

    /// Gives the summed area of all input items
    fn calculate_input_items_area(input_items: &[InputItem]) -> u32 {
        let mut cumulative_area = 0u32;
        for item in input_items {
            cumulative_area += item.width * item.height;
        }
        cumulative_area
    }
}
