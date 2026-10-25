use std::sync::Arc;

use anyhow::{Result, anyhow, bail};
use winit::{dpi::PhysicalSize, event_loop::OwnedDisplayHandle, window::Window};

type Context = softbuffer::Context<OwnedDisplayHandle>;
type Surface = softbuffer::Surface<OwnedDisplayHandle, Arc<dyn Window>>;

pub struct Renderer {
    context: Context,
    surface: Option<Surface>,
}

impl Renderer {
    pub fn new(display_handle: OwnedDisplayHandle) -> Result<Self> {
        Ok(Self {
            context: Context::new(display_handle).map_err(|err| anyhow!("{err}"))?,
            surface: None,
        })
    }

    pub fn get_surface(&mut self) -> Option<&mut Surface> {
        self.surface.as_mut()
    }

    pub fn create_surface(&mut self, window: Arc<dyn Window>) -> Result<()> {
        let size = window.surface_size();

        match Surface::new(&self.context, window) {
            Ok(mut surface) => {
                let w = size.width.try_into()?;
                let h = size.height.try_into()?;
                surface.resize(w, h).map_err(|err| anyhow!("{err}"))?;
                self.surface = Some(surface);
            }
            Err(err) => bail!("failed to create surface: {err}"),
        }

        Ok(())
    }

    pub fn destroy_surface(&mut self) {
        self.surface = None;
    }

    pub fn resize_surface(&mut self, size: PhysicalSize<u32>) -> anyhow::Result<()> {
        let Some(surface) = &mut self.surface else {
            return Ok(());
        };

        let w = size.width.try_into()?;
        let h = size.height.try_into()?;
        surface.resize(w, h).map_err(|err| anyhow!("{err}"))?;

        Ok(())
    }
}
