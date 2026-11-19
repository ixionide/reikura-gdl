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
    registry: ImageRegistry,
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
            registry: ImageRegistry::new(),
            request_redraw: false,
        }
    }

    #[inline]
    fn update_display(&mut self) {
        if let Some(main_image) = self.registry.get_ref(None) {
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
        let Some(transition) = &mut self.transition else {
            return;
        };

        let frame_time = frame_step * FRAME_INTERVAL_MS;
        transition.advance(frame_time);
        let finished = transition.is_finished();
        let alpha = transition.alpha();
        let full_region = self.display.region();

        if finished {
            blit::copy(
                self.registry.get_ref(None).unwrap(),
                &mut self.display,
                full_region,
            );
        } else {
            blit::copy(&self.transition_source, &mut self.display, full_region);
            blit::blend_alpha(
                self.registry.get_ref(None).unwrap(),
                &mut self.display,
                transition.dirty_region,
                if finished { 255 } else { alpha },
            );
        }

        if finished {
            self.transition = None;
        }

        self.request_redraw = true;
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

    pub fn clear_image(&mut self, id: Option<u8>, r: u8, g: u8, b: u8) -> Result<()> {
        let Some(image) = self.registry.get_mut(id) else {
            bail!("failed to get image id: {id:?}");
        };

        image.clear(bgr_color(b, g, r));

        if self.registry.main == id {
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
                .get_mut(Some(src_id))
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
            .registry
            .get_pair(src_id, None)
            .ok_or(anyhow!("failed to get image {src_id}"))?;
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
            .registry
            .get_mut(None)
            .ok_or(anyhow!("failed to get main image"))?;

        let transition_region = self.transition_source.region();
        blit::copy(main, &mut self.transition_source, transition_region);
        main.clear(bgr_color(b, g, r));

        self.transition = Some(Transition::new(duration_ms, transition_region));

        Ok(())
    }
}
