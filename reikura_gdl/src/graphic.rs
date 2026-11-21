pub mod blit;
pub mod color;
mod image;
mod transition;

pub use self::{
    blit::BlitParam,
    image::{Image, ImageRegistry, MAX_IMAGE},
};

use crate::{graphic::transition::Transition, vm::FRAME_INTERVAL_MS};
use anyhow::{Result, anyhow, bail};
use reikura_util::image::bgr_color;

pub struct Graphics {
    pub display: Image,
    transition_source: Image,
    transition: Option<Transition>,
    _x: u32,
    _y: u32,
    images: ImageRegistry,
    pub request_redraw: bool,
}

impl Graphics {
    pub fn new(view_width: u32, view_height: u32) -> Self {
        Self {
            display: Image::new(view_width, view_height),
            transition_source: Image::new(view_width, view_height),
            transition: None,
            _x: 0,
            _y: 0,
            images: ImageRegistry::new(),
            request_redraw: false,
        }
    }

    #[inline]
    fn update_display(&mut self) {
        if let Some(main_image) = self.images.get_ref(None) {
            let full_region = self.display.region();
            blit::copy(main_image, &mut self.display, full_region);
        } else {
            self.display.clear(color::BLACK);
        };

        self.request_redraw = true;
    }

    pub fn is_on_transition(&self) -> bool {
        self.transition.is_some()
    }

    pub fn update(&mut self, frame_step: u32) {
        let Some(main_image) = self.images.get_ref(None) else {
            return;
        };

        let Some(transition) = &mut self.transition else {
            return;
        };

        let frame_time = frame_step * FRAME_INTERVAL_MS;
        transition.advance(frame_time);
        let finished = transition.is_finished();
        let alpha = transition.alpha();
        let full_region = self.display.region();

        if finished {
            blit::copy(main_image, &mut self.display, full_region);
            self.transition = None;
        } else {
            blit::copy(&self.transition_source, &mut self.display, full_region);
            blit::blend_alpha(
                main_image,
                &mut self.display,
                transition.dirty_region,
                alpha,
            );
        }

        self.request_redraw = true;
    }

    pub fn unset_main_image(&mut self) {
        self.images.unset_main();
        self.update_display();
    }

    pub fn set_main_image(&mut self, id: u8, x: u32, y: u32) {
        self.images.set_main(id);
        self._x = x;
        self._y = y;

        self.update_display();
    }

    pub fn new_image(&mut self, id: u8, width: u32, height: u32) -> Result<()> {
        self.images.set(id, width, height, None)?;

        if self.images.main == Some(id) {
            self.update_display();
        }

        Ok(())
    }

    pub fn set_image(&mut self, id: u8, width: u32, height: u32, data: &[u8]) -> Result<()> {
        self.images.set(id, width, height, Some(data))?;

        if self.images.main == Some(id) {
            self.update_display();
        }

        Ok(())
    }

    pub fn clear_image(&mut self, id: Option<u8>, r: u8, g: u8, b: u8) -> Result<()> {
        let Some(image) = self.images.get_mut(id) else {
            bail!("failed to get image id: {id:?}");
        };

        image.clear(bgr_color(b, g, r));

        if self.images.main == id {
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
                .images
                .get_mut(Some(src_id))
                .ok_or_else(|| anyhow!("failed to get image {src_id}"))?;

            blit::copy_within(src, param);
        } else {
            let (src, dst) = self
                .images
                .get_pair(src_id, dst_id)
                .ok_or_else(|| anyhow!("failed to get image {src_id} & {dst_id:?}"))?;

            blit::copy(src, dst, param);
        }

        if dst_id.is_none() || self.images.main == dst_id {
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
            .images
            .get_pair(src_id, dst_id)
            .ok_or_else(|| anyhow!("failed to get image {src_id} & {dst_id:?}"))?;

        blit::blend(src, dst, param);

        if dst_id.is_none() || self.images.main == dst_id {
            self.update_display();
        }

        Ok(())
    }

    pub fn start_fade_in_image(
        &mut self,
        src_id: u8,
        param: BlitParam,
        duration_ms: u32,
    ) -> Result<()> {
        if self.transition.is_some() {
            bail!("transition already running");
        }

        let (src, main) = self
            .images
            .get_pair(src_id, None)
            .ok_or_else(|| anyhow!("failed to get image {src_id}"))?;
        let transition_region = self.transition_source.region();

        blit::copy(main, &mut self.transition_source, transition_region);
        blit::blend(src, main, param);
        self.transition = Some(Transition::new(duration_ms, param.dst_region()));

        Ok(())
    }

    pub fn start_fade_in_color(&mut self, r: u8, g: u8, b: u8, duration_ms: u32) -> Result<()> {
        if self.transition.is_some() {
            bail!("transition already running");
        }

        let main = self
            .images
            .get_mut(None)
            .ok_or_else(|| anyhow!("failed to get main image"))?;

        let transition_region = self.transition_source.region();
        blit::copy(main, &mut self.transition_source, transition_region);
        main.clear(bgr_color(b, g, r));

        self.transition = Some(Transition::new(duration_ms, transition_region));

        Ok(())
    }
}
