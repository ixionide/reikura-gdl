use std::{collections::HashMap, num::NonZeroU32, ops::DerefMut, sync::Arc};

use anyhow::{anyhow, bail};
use winit::window::Window;

type WindowContext = softbuffer::Context<Arc<dyn Window>>;
type WindowSurface = softbuffer::Surface<Arc<dyn Window>, Arc<dyn Window>>;

use crate::{
    Rect,
    backend::{BlitParam, GraphicBackend, MAX_IMAGE, software::Surface},
};

pub struct Renderer {
    context: WindowContext,
    window_size: Option<(u32, u32)>,
    window_surface: Option<WindowSurface>,

    game_surface: Surface,
    target_surface: Option<u8>,
    surfaces: HashMap<u8, Surface>,
    damaged: Option<Rect>,
}

impl Renderer {
    pub fn new(window: Arc<dyn Window>, width: u32, height: u32) -> anyhow::Result<Self> {
        let context = match WindowContext::new(window) {
            Ok(ctx) => ctx,
            Err(err) => bail!("failed to create softbuffer context: {err}"),
        };

        Ok(Self {
            context,
            window_size: None,
            window_surface: None,
            target_surface: None,
            game_surface: Surface::new_black(width, height),
            surfaces: HashMap::with_capacity(MAX_IMAGE),
            damaged: None,
        })
    }

    pub fn update_screen(&mut self) -> Option<Rect> {
        let dst = &mut self.game_surface;
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
        let surface = Surface::new(width, height);

        self.maybe_damaged(id, surface.rect());
        self.surfaces.insert(id, surface);

        Ok(())
    }

    fn load_image(&mut self, id: u8, width: u32, height: u32, data: &[u8]) -> anyhow::Result<()> {
        let surface = Surface::from_bytes(width, height, data)?;

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

    fn _resized(&mut self, width: u32, height: u32) -> anyhow::Result<()> {
        let Some(width) = NonZeroU32::new(width) else {
            return Ok(());
        };

        let Some(height) = NonZeroU32::new(height) else {
            return Ok(());
        };

        if let Some(surface) = &mut self.window_surface {
            _ = surface
                .resize(width, height)
                .map_err(|it| anyhow!("{it}"))?;
        }

        // NB: do render here??

        self.window_size = Some((width.into(), height.into()));
        Ok(())
    }

    fn _render(&mut self) -> anyhow::Result<()> {
        let Some(_damage) = self.update_screen() else {
            return Ok(()); // state is clean no need to render
        };

        let game_surface = &self.game_surface;

        let Some((window_surface, (width, height))) =
            self.window_surface.as_mut().zip(self.window_size)
        else {
            bail!("surface is destroyed");
        };

        match window_surface.buffer_mut() {
            // XXX: fix the pixel channel
            Ok(mut buffer) => {
                if width == game_surface.width && height == game_surface.height {
                    buffer.copy_from_slice(&game_surface.pixels);
                } else {
                    let mut window_surface =
                        Surface::from_pixels(width, height, buffer.deref_mut())?;
                    // TODO: mantain aspect ratio
                    let window_rect = window_surface.rect();
                    let game_rect = game_surface.rect();

                    if window_rect.size() == game_rect.size() {
                        window_surface.blit_copy(game_rect, window_rect, &self.game_surface)?;
                    } else {
                        window_surface.blit_scale_copy(
                            game_rect,
                            window_rect,
                            &self.game_surface,
                        )?;
                    }
                }

                buffer.present().map_err(|err| anyhow::anyhow!("{err}"))?
            }
            Err(err) => bail!("failed to get surface buffer: {err}"),
        }

        Ok(())
    }

    fn _resumed(&mut self) {}

    fn _suspended(&mut self) {}

    fn _create_surface(&mut self, window: Arc<dyn Window>) -> anyhow::Result<()> {
        let size = window.surface_size();

        match WindowSurface::new(&self.context, window) {
            Ok(mut surface) => {
                let w = size.width.try_into()?;
                let h = size.height.try_into()?;

                surface.resize(w, h).map_err(|err| anyhow!("{err}"))?;

                self.window_surface = Some(surface);
                self.window_size = Some((w.into(), h.into()));
            }
            Err(err) => bail!("failed to create surface: {err}"),
        }

        Ok(())
    }

    fn _destroy_surface(&mut self) {
        self.window_surface = None;
        self.window_size = None;
    }
}
