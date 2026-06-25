use std::{collections::HashMap, num::NonZeroU32, sync::Arc};

use anyhow::bail;
use winit::window::Window;

use crate::{
    Rect,
    backend::{BlitParam, GraphicBackend, MAX_IMAGE, software::Surface},
};

pub struct Renderer {
    softbuffer_ctx: softbuffer::Context<Arc<dyn Window>>,
    softbuffer_surface: Option<softbuffer::Surface<Arc<dyn Window>, Arc<dyn Window>>>,
    target_surface: Option<u8>,
    screen_surface: Surface,
    surfaces: HashMap<u8, Surface>,
    damaged: Option<Rect>,
}

impl Renderer {
    pub fn new(window: Arc<dyn Window>, width: u16, height: u16) -> anyhow::Result<Self> {
        let softbuffer_ctx = match softbuffer::Context::new(window) {
            Ok(ctx) => ctx,
            Err(err) => bail!("failed to create softbuffer context: {err}"),
        };

        Ok(Self {
            softbuffer_ctx,
            softbuffer_surface: None,
            target_surface: None,
            screen_surface: Surface::new_black(width, height),
            surfaces: HashMap::with_capacity(MAX_IMAGE),
            damaged: None,
        })
    }

    pub fn update_screen(&mut self) -> Option<Rect> {
        let dst = &mut self.screen_surface;
        let target = self.target_surface?;
        let (damaged, src) = self.damaged.take().zip(self.surfaces.get(&target))?;

        dst.blit_copy(damaged, damaged, src)
            .expect("failed to blit screen surface");

        Some(damaged)
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

        surface.clear(color);
        self.maybe_damaged(id, rect);

        Ok(())
    }

    fn blit_copy_image(&mut self, param: BlitParam) -> anyhow::Result<()> {
        if param.src_id == param.dst_id {
            todo!();
        }

        let [Some(src), Some(dst)] = self
            .surfaces
            .get_disjoint_mut([&param.src_id, &param.dst_id])
        else {
            bail!(
                "fail to get src:{} or dst:{} image",
                param.src_id,
                param.dst_id
            );
        };

        dst.blit_copy(param.src_rect(), param.dst_rect(), src)?;
        self.maybe_damaged(param.dst_id, param.dst_rect());

        Ok(())
    }

    fn blit_blend_image(&mut self, param: BlitParam) -> anyhow::Result<()> {
        if param.src_id == param.dst_id {
            todo!();
        }

        let [Some(src), Some(dst)] = self
            .surfaces
            .get_disjoint_mut([&param.src_id, &param.dst_id])
        else {
            bail!(
                "fail to get src:{} or dst:{} image",
                param.src_id,
                param.dst_id
            );
        };

        dst.blit_blend(param.src_rect(), param.dst_rect(), src)?;
        self.maybe_damaged(param.dst_id, param.dst_rect());

        Ok(())
    }

    fn _init(&mut self, _window: Arc<dyn Window>) -> anyhow::Result<()> {
        Ok(())
    }

    fn _render(&mut self) -> anyhow::Result<()> {
        let Some(_damage) = self.update_screen() else {
            return Ok(());
        };

        let Some(surface) = self.softbuffer_surface.as_mut() else {
            bail!("surface is destroyed");
        };

        match surface.buffer_mut() {
            // XXX: fix the pixel format and the size
            Ok(mut buffer) => {
                buffer.copy_from_slice(&self.screen_surface.pixels);
                buffer.present().map_err(|it| anyhow::anyhow!("{it}"))?
            }
            Err(err) => bail!("failed to get surface buffer: {err}"),
        }

        Ok(())
    }

    fn _resumed(&mut self) {}

    fn _suspended(&mut self) {}

    fn _create_surface(&mut self, window: Arc<dyn Window>) -> anyhow::Result<()> {
        match softbuffer::Surface::new(&self.softbuffer_ctx, window) {
            Ok(mut surface) => {
                let w = NonZeroU32::new(self.screen_surface.width as _).unwrap();
                let h = NonZeroU32::new(self.screen_surface.height as _).unwrap();
                surface.resize(w, h).unwrap();
                self.softbuffer_surface = Some(surface)
            }
            Err(err) => bail!("failed to create surface: {err}"),
        }

        Ok(())
    }

    fn _destroy_surface(&mut self) {
        self.softbuffer_surface = None;
    }
}
