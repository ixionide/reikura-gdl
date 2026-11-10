pub mod blit;
pub mod color;
mod image;

pub use self::{
    blit::BlitParam,
    image::{Image, ImageRegistry, MAX_IMAGE},
};

use crate::vm::FRAME_INTERVAL_MS;
use anyhow::{Result, anyhow, bail};
use reikura_util::image::bgr_color;

pub struct Graphics {
    pub display: Image,
    _main_temp: Image,
    _x: u32,
    _y: u32,
    registry: ImageRegistry,
    pub request_redraw: bool,
}

impl Graphics {
    pub fn new(view_width: u32, view_height: u32) -> Self {
        Self {
            display: Image::new(view_width, view_height),
            _main_temp: Image::new(view_width, view_height),
            _x: 0,
            _y: 0,
            registry: ImageRegistry::new(),
            request_redraw: false,
        }
    }

    #[inline]
    fn update_display(&mut self) {
        if let Some(main_image) = self.registry.get_main_ref() {
            let w = self.display.width as i32;
            let h = self.display.height as i32;
            blit::copy(main_image, &mut self.display, BlitParam::new(0, 0, w, h));
        } else {
            self.display.clear(color::BLACK);
        };

        self.request_redraw = true
    }

    pub fn update(&mut self, advance: u32) {
        // let _advance_ms = (advance << 4) + advance;
        let _advance_ms = advance * FRAME_INTERVAL_MS;
    }

    pub fn unset_main_image(&mut self) {
        self.registry.unset_main();
        self.update_display();
    }

    pub fn set_main_image(&mut self, id: u8, x: u32, y: u32) {
        self.registry.set_main(id);
        self._x = x;
        self._y = y;

        self.update_display();
    }

    pub fn new_image(&mut self, id: u8, width: u32, height: u32) -> Result<()> {
        self.registry.set(id, width, height, None)?;

        if self.registry.main == Some(id) {
            self.update_display();
        }

        Ok(())
    }

    pub fn set_image(&mut self, id: u8, width: u32, height: u32, data: &[u8]) -> Result<()> {
        self.registry.set(id, width, height, Some(data))?;

        if self.registry.main == Some(id) {
            self.update_display();
        }

        Ok(())
    }

    pub fn clear_image(&mut self, id: u8, (r, g, b): (u8, u8, u8)) -> Result<()> {
        let color = bgr_color(b, g, r);

        let Some(image) = self.registry.get_mut(id) else {
            bail!("failed to get image id: {id}");
        };

        image.clear(color);

        if self.registry.main == Some(id) {
            self.update_display();
        }

        Ok(())
    }

    pub fn blit_copy_image(
        &mut self,
        src_id: u8,
        dst_id: Option<u8>,
        param: BlitParam,
    ) -> Result<()> {
        if dst_id == Some(src_id) {
            let src = self
                .registry
                .get_mut(src_id)
                .ok_or(anyhow!("failed to get image {src_id}"))?;

            blit::copy_within(src, param);
        } else {
            let (src, dst) = self
                .registry
                .get_pair(src_id, dst_id)
                .ok_or(anyhow!("failed to get image {src_id} & {dst_id:?}"))?;

            blit::copy(src, dst, param);
        }

        if dst_id.is_none() || self.registry.main == dst_id {
            self.update_display();
        }

        Ok(())
    }

    pub fn blit_blend_image(
        &mut self,
        src_id: u8,
        dst_id: Option<u8>,
        param: BlitParam,
    ) -> Result<()> {
        let (src, dst) = self
            .registry
            .get_pair(src_id, dst_id)
            .ok_or(anyhow!("failed to get image {src_id} & {dst_id:?}"))?;

        blit::blend(src, dst, param);

        if dst_id.is_none() || self.registry.main == dst_id {
            self.update_display();
        }

        Ok(())
    }
}
