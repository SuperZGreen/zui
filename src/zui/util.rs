/// Converts an arbitrary reference into a u8 slice
#[allow(dead_code)]
pub unsafe fn ref_as_u8_slice<T>(data: &T) -> &[u8] {
    std::slice::from_raw_parts((data as *const T) as *const u8, std::mem::size_of_val(data))
}

/// Converts an arbitrary slice into a u8 slice
#[allow(dead_code)]
pub unsafe fn slice_as_u8_slice<T>(data: &[T]) -> &[u8] {
    std::slice::from_raw_parts(data.as_ptr() as *const u8, std::mem::size_of_val(data))
}

/// Creates a shader module from a shader source file path
#[allow(dead_code)]
pub fn shader_module_from_file_path(
    device: &wgpu::Device,
    shader_file_path: &str,
    shader_module_label: &str,
) -> Result<wgpu::ShaderModule, String> {
    // getting shader source from file
    let shader_source = std::fs::read_to_string(shader_file_path).expect(&format!(
        "error reading shader source: {:?}",
        shader_file_path
    ));

    // creating shader module
    Ok(device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(shader_module_label),
        source: wgpu::ShaderSource::Wgsl(shader_source.into()),
    }))
}

/// Converts an X-axis value from wgpu's normalised device coordinates (NDC) to frame buffer coordinates
pub fn normalised_device_space_to_frame_buffer_space_x(
    screen_space_x: f32,
    viewport_width_px: f32,
) -> f32 {
    (screen_space_x / 2f32 + 0.5f32) * viewport_width_px - 0.5f32
}

/// Converts an Y-axis value from wgpu's normalised device coordinates (NDC) to frame buffer coordinates
pub fn normalised_device_space_to_frame_buffer_space_y(
    screen_space_y: f32,
    viewport_height_px: f32,
) -> f32 {
    (1f32 - (screen_space_y / 2f32 + 0.5f32)) * viewport_height_px - 0.5f32
}

/// Converts from viewport pixels (y-up (0,0) in bottom left corner) to NDC/screen space
pub fn viewport_px_to_normalised_device_coordinates(
    viewport_px_position: f32,
    viewport_px_span: u32,
) -> f32 {
    let normalised_position = viewport_px_position as f32 / viewport_px_span as f32;
    (normalised_position - 0.5f32) * 2f32
}
