pub trait Renderable {
    fn to_vertices(
        &self,
        viewport_dimensions_px: glam::Vec2,
    ) -> (
        Vec<super::renderer::SimpleVertex>,
        Vec<super::text_renderer::TextVertex>,
    );
}
