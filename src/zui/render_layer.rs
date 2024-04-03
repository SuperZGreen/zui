use std::ops::Range;

use super::{
    primitives::Rectangle,
    renderers::{image_renderer::ImageVertex, SimpleVertex, TextVertex},
};

/// Render layers start out containing their own vectors of vertices, they are then 'joined' to a
/// single vector for each type of vertex. This is done so that all the vertices of a type can be
/// uploaded to the GPU at once, and bypass the issue of multiple overwriting create_buffer_init
/// calls in a single command encoder causing massive GPU memory leaks. TODO: chase this up and file
/// a bug reportwith the wgpu team.
pub struct RenderLayers {
    render_layers: Vec<RenderLayer>,
}

impl RenderLayers {
    /// Creates a new empty RenderLayers
    pub fn new() -> Self {
        Self {
            render_layers: Vec::new(),
        }
    }

    /// Creates a new RenderLayer and returns its index
    pub fn new_layer(&mut self, clip_rectangle_opt: Option<Rectangle<i32>>) -> usize {
        let index = self.render_layers.len();
        self.render_layers
            .push(RenderLayer::new(clip_rectangle_opt));
        index
    }

    /// Gets a mutable reference to a RenderLayer by index
    pub fn get_mut(&mut self, index: usize) -> &mut RenderLayer {
        &mut self.render_layers[index]
    }

    /// Consumes the RenderLayers and joins the layers' vertices vecs together to create a single
    /// vec for each type. This reduced the number of uploads to the GPU, and prevents a wgpu
    /// memory leak bug from occuring.
    pub fn unify(self) -> UnifiedRenderLayers {
        let mut simple_vertices = Vec::with_capacity(self.simple_vertices_used());
        let mut text_vertices = Vec::with_capacity(self.text_vertices_used());
        let mut image_vertices = Vec::with_capacity(self.image_vertices_used());

        let mut unified_layers = Vec::with_capacity(self.render_layers.len());

        for mut render_layer in self.render_layers {
            unified_layers.push(UnifiedLayerInfo {
                clip_rectangle_opt: render_layer.clip_rectangle_opt,
                simple_vertices_range: Range {
                    start: simple_vertices.len() as u32,
                    end: (simple_vertices.len() + render_layer.simple_vertices.len()) as u32,
                },
                text_vertices_range: Range {
                    start: text_vertices.len() as u32,
                    end: (text_vertices.len() + render_layer.text_vertices.len()) as u32,
                },
                image_vertices_range: Range {
                    start: image_vertices.len() as u32,
                    end: (image_vertices.len() + render_layer.image_vertices.len()) as u32,
                },
            });

            simple_vertices.append(&mut render_layer.simple_vertices);
            text_vertices.append(&mut render_layer.text_vertices);
            image_vertices.append(&mut render_layer.image_vertices);
        }

        UnifiedRenderLayers {
            layer_infos: unified_layers,
            simple_vertices,
            text_vertices,
            image_vertices,
        }
    }

    /// Gives the total number of simple vertices used by all layers
    fn simple_vertices_used(&self) -> usize {
        let mut vertices_used = 0usize;
        for render_layer in self.render_layers.iter() {
            vertices_used += render_layer.simple_vertices.len()
        }
        vertices_used
    }

    /// Gives the total number of text vertices used by all layers
    fn text_vertices_used(&self) -> usize {
        let mut vertices_used = 0usize;
        for render_layer in self.render_layers.iter() {
            vertices_used += render_layer.text_vertices.len()
        }
        vertices_used
    }

    /// Gives the total number of image vertices used by all layers
    fn image_vertices_used(&self) -> usize {
        let mut vertices_used = 0usize;
        for render_layer in self.render_layers.iter() {
            vertices_used += render_layer.image_vertices.len()
        }
        vertices_used
    }
}

/// Contains the concatenated vertices of each of the render layers, which are then uploaded to the
/// GPU once and rendered using ranges kept in the layer_infos.
pub struct UnifiedRenderLayers {
    layer_infos: Vec<UnifiedLayerInfo>,
    simple_vertices: Vec<SimpleVertex>,
    text_vertices: Vec<TextVertex>,
    image_vertices: Vec<ImageVertex>,
}

impl UnifiedRenderLayers {
    /// Iterates over the unified layer infos
    pub fn iter(&self) -> std::slice::Iter<'_, UnifiedLayerInfo> {
        self.layer_infos.iter()
    }

    pub fn simple_vertices(&self) -> &[SimpleVertex] {
        &self.simple_vertices
    }

    pub fn text_vertices(&self) -> &[TextVertex] {
        &self.text_vertices
    }

    pub fn image_vertices(&self) -> &[ImageVertex] {
        &self.image_vertices
    }
}

/// The render layer info for when the vertices are held in a single buffer by the
/// UnifiedRenderLayers struct
pub struct UnifiedLayerInfo {
    /// The clipping rectangle in viewport pixels, contents outside of this will not be rendered
    clip_rectangle_opt: Option<Rectangle<i32>>,

    /// The range of simple vertices to be used, u32 is used instead of usize for compatibility with
    /// wgpu
    simple_vertices_range: Range<u32>,

    /// The range of text vertices to be used, u32 is used instead of usize for compatibility with
    /// wgpu
    text_vertices_range: Range<u32>,

    /// The range of image vertices to be used, u32 is used instead of usize for compatibility with
    /// wgpu
    image_vertices_range: Range<u32>,
}

impl UnifiedLayerInfo {
    pub fn clip_rectangle_opt(&self) -> Option<Rectangle<i32>> {
        self.clip_rectangle_opt
    }

    pub fn simple_vertices_range(&self) -> Range<u32> {
        self.simple_vertices_range.clone()
    }

    pub fn text_vertices_range(&self) -> Range<u32> {
        self.text_vertices_range.clone()
    }

    pub fn image_vertices_range(&self) -> Range<u32> {
        self.image_vertices_range.clone()
    }
}

/// A layer the contains vertices of different types, that will be rendered on top of other layers.
/// The layer also may contain a clipping rectangle, which will prevent contents outside of this
/// rectangle from being rendered
pub struct RenderLayer {
    /// The clipping rectangle in viewport pixels, contents outside of this will not be rendered
    clip_rectangle_opt: Option<Rectangle<i32>>,

    /// The SimpleVertices that will be rendered on this layer
    simple_vertices: Vec<SimpleVertex>,

    /// The TextVertices that will be rendered on this layer
    text_vertices: Vec<TextVertex>,

    /// The ImageVertices that will be rendered on this layer
    image_vertices: Vec<ImageVertex>,
}

impl RenderLayer {
    fn new(clip_rectangle_opt: Option<Rectangle<i32>>) -> Self {
        Self {
            simple_vertices: Vec::new(),
            text_vertices: Vec::new(),
            image_vertices: Vec::new(),
            clip_rectangle_opt,
        }
    }

    pub fn simple_vertices_mut(&mut self) -> &mut Vec<SimpleVertex> {
        &mut self.simple_vertices
    }

    pub fn text_vertices_mut(&mut self) -> &mut Vec<TextVertex> {
        &mut self.text_vertices
    }

    /// Borrows all of the vertices types mutably at the same time
    pub fn vertices_mut(
        &mut self,
    ) -> (
        &mut Vec<SimpleVertex>,
        &mut Vec<TextVertex>,
        &mut Vec<ImageVertex>,
    ) {
        (
            &mut self.simple_vertices,
            &mut self.text_vertices,
            &mut self.image_vertices,
        )
    }
}
