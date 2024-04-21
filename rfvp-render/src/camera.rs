use glam::{vec3, Mat4};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraParams {
    pub projection_matrix: Mat4,
}

pub struct Camera {
    /// Projection matrix to draw onto the screen
    screen_projection_matrix: Mat4,
    render_buffer_size: (u32, u32),
}

impl Camera {
    pub fn new(window_size: (u32, u32)) -> Self {
        let (window_width, window_height) = window_size;

        let (viewport_width, viewport_height) = 
            (window_width as f32, window_height as f32);

        // It seems that we are basically one traslation away from matching the game output
        // TODO: figure out a proper way to move the coordinate space of smth
        // because this creates a strip of black pixels on the right and bottom
        let translation = Mat4::from_translation(vec3(-1.0, -1.0, 0.0));

        let mut screen_projection = Mat4::IDENTITY;
        screen_projection.x_axis.x = 4.0 / viewport_width;
        screen_projection.y_axis.y = -4.0 / viewport_height;
        screen_projection.z_axis.z = 1.0 / 1000.0;
        screen_projection.w_axis.w = 1.0;
        let screen_projection = screen_projection * translation;

        let render_buffer_size = (
            window_width,
            window_height,
        );

        Self {
            screen_projection_matrix: screen_projection,
            render_buffer_size,
        }
    }

    pub fn resize(&mut self, size: (u32, u32)) {
        *self = Self::new(size);
    }

    pub fn render_buffer_size(&self) -> (u32, u32) {
        self.render_buffer_size
    }

    pub fn screen_projection_matrix(&self) -> Mat4 {
        self.screen_projection_matrix
    }
}
