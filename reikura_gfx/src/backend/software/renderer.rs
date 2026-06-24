use std::{collections::HashMap, sync::Arc};

use anyhow::bail;
use winit::window::Window;

use crate::{
    Rect,
    backend::{BlitParam, GraphicBackend, MAX_IMAGE, software::Surface},
};

pub struct Renderer {
    target_surface: Option<u8>,
    screen_surface: Surface,
    surfaces: HashMap<u8, Surface>,
    damaged: Option<Rect>,
}

impl Renderer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            target_surface: None,
            screen_surface: Surface::new_black(width, height),
            surfaces: HashMap::with_capacity(MAX_IMAGE),
            damaged: None,
        }
    }

    pub fn update_screen(&mut self) {
        let dst = &mut self.screen_surface;
        let src = self.target_surface.and_then(|id| self.surfaces.get(&id));

        if let Some((damaged, src)) = self.damaged.zip(src) {
            dst.blit_copy(damaged, damaged, src);
            self.damaged = None;
        }
    }

    #[inline]
    fn maybe_damaged(&mut self, id: u8, rect: Rect) {
        if self.target_surface == Some(id) {
            self.damaged = Some(rect);
        }
    }
}

impl GraphicBackend for Renderer {
    fn set_target(&mut self, id: u8, _x: u32, _y: u32) {
        self.target_surface = Some(id);
    }

    fn unset_target(&mut self) {
        self.target_surface = None;
    }

    fn new_image(&mut self, id: u8, width: u32, height: u32) -> anyhow::Result<()> {
        let surface = Surface::new(width as u16, height as u16);

        self.maybe_damaged(id, surface.rect());
        self.surfaces.insert(id, surface);

        Ok(())
    }

    fn load_image(&mut self, id: u8, width: u32, height: u32, data: &[u8]) -> anyhow::Result<()> {
        let surface = Surface::from_bytes(width as u16, height as u16, data)?;

        self.maybe_damaged(id, surface.rect());
        self.surfaces.insert(id, surface);

        Ok(())
    }

    fn clear_image(&mut self, id: u8, (r, g, b): (u8, u8, u8)) -> anyhow::Result<()> {
        let Some(surface) = self.surfaces.get_mut(&id) else {
            bail!("image {id} is empty");
        };

        let color = u32::from_le_bytes([r, g, b, 0xFF]);
        let rect = surface.rect();

        surface.fill(color);
        self.maybe_damaged(id, rect);

        Ok(())
    }

    fn blit_copy_image(&mut self, param: BlitParam) -> anyhow::Result<()> {
        if param.src_id == param.dst_id {
            todo!();
        }

        // i hate this
        let Some(mut dst) = self.surfaces.remove(&param.dst_id) else {
            bail!("dst image {} is empty", param.dst_id);
        };

        let Some(src) = self.surfaces.get(&param.src_id) else {
            bail!("src image {} is empty", param.src_id);
        };

        dst.blit_copy(param.src_rect(), param.dst_rect(), src);
        // i hate this
        self.surfaces.insert(param.dst_id, dst);
        self.maybe_damaged(param.dst_id, param.dst_rect());

        Ok(())
    }

    fn blit_blend_image(&mut self, param: BlitParam) -> anyhow::Result<()> {
        if param.src_id == param.dst_id {
            todo!();
        }

        // i hate this
        let Some(mut dst) = self.surfaces.remove(&param.dst_id) else {
            bail!("dst image {} is empty", param.dst_id);
        };

        let Some(src) = self.surfaces.get(&param.src_id) else {
            bail!("src image {} is empty", param.src_id);
        };

        dst.blit_blend(param.src_rect(), param.dst_rect(), src);
        // i hate this
        self.surfaces.insert(param.dst_id, dst);
        self.maybe_damaged(param.dst_id, param.dst_rect());

        Ok(())
    }

    fn _render(&mut self) -> anyhow::Result<()> {
        todo!()
    }

    #[allow(unused)]
    fn _resumed(&mut self, window: Arc<dyn Window>) -> anyhow::Result<()> {
        todo!()
    }

    fn _suspended(&mut self) {
        todo!()
    }
}
