use std::sync::Arc;

use winit::window::Window;

use crate::backend::BlitParam;

pub trait GraphicBackend {
    fn set_target(&mut self, id: u8, x: u32, y: u32);
    fn unset_target(&mut self);

    fn new_image(&mut self, id: u8, width: u32, height: u32) -> anyhow::Result<()>;
    fn load_image(&mut self, id: u8, width: u32, height: u32, data: &[u8]) -> anyhow::Result<()>;
    fn clear_image(&mut self, id: u8, color: (u8, u8, u8)) -> anyhow::Result<()>;
    fn blit_copy_image(&mut self, param: BlitParam) -> anyhow::Result<()>;
    fn blit_blend_image(&mut self, param: BlitParam) -> anyhow::Result<()>;

    fn _init(&mut self, window: Arc<dyn Window>) -> anyhow::Result<()>;
    fn _resized(&mut self, width: u32, height: u32) -> anyhow::Result<()>;
    fn _render(&mut self) -> anyhow::Result<()>;
    fn _resumed(&mut self);
    fn _suspended(&mut self);
    fn _create_surface(&mut self, window: Arc<dyn Window>) -> anyhow::Result<()>;
    fn _destroy_surface(&mut self);
}
