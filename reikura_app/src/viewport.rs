pub struct Viewport {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Viewport {
    pub fn new(w: u32, h: u32) -> Self {
        Self { x: 0, y: 0, w, h }
    }

    pub fn letterbox(window_size: (u32, u32), view_size: (u32, u32)) -> Self {
        let window_w = window_size.0 as f32;
        let window_h = window_size.1 as f32;
        let view_w = view_size.0 as f32;
        let view_h = view_size.1 as f32;

        let window_scale = window_w / window_h;
        let view_scale = view_w / view_h;

        if window_scale < view_scale {
            let w = window_w as u32;
            let h = (window_w / view_scale) as u32;
            let x = 0;
            let y = (window_size.1 - h) / 2;
            Self { x, y, w, h }
        } else {
            let w = (window_h * view_scale) as u32;
            let h = window_h as u32;
            let x = (window_size.0 - w) / 2;
            let y = 0;
            Self { x, y, w, h }
        }
    }

    pub fn map_position(&self, view_size: (u32, u32), mouse_pos: (f32, f32)) -> (i32, i32) {
        let (mouse_x, mouse_y) = mouse_pos;

        let (min_x, min_y) = (self.x as f32, self.y as f32);

        let (local_x, local_y) = (mouse_x - min_x, mouse_y - min_y);
        let (view_w, view_h) = (view_size.0 as f32, view_size.1 as f32);

        let virtual_x = ((local_x / self.w as f32) * view_w).round() as i32;
        let virtual_y = ((local_y / self.h as f32) * view_h).round() as i32;

        (virtual_x, virtual_y)
    }
}
