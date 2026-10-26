#![allow(clippy::too_many_arguments)]

mod image;

pub use self::image::{Image, Images, MAX_IMAGE, color};

use anyhow::{Result, anyhow, bail};
use reikura_util::image::rgb_color;

pub struct Graphics {
    pub display: Image,
    _main_temp: Image,
    _x: u32,
    _y: u32,
    images: Images,
    pub request_redraw: bool,
}

impl Graphics {
    pub fn new(view_width: u32, view_height: u32) -> Self {
        Self {
            display: Image::new(view_width, view_height),
            _main_temp: Image::new(view_width, view_height),
            _x: 0,
            _y: 0,
            images: Images::new(),
            request_redraw: false,
        }
    }

    #[inline]
    fn update_display(&mut self) {
        let Some(main_image) = self.images.get_main_ref() else {
            panic!("main image is unset");
        };

        self.display
            .blit_copy(main_image, 0, 0, 0, 0, main_image.width, main_image.height);
        self.request_redraw = true
    }

    pub fn unset_main_image(&mut self) {
        self.images.unset_main();
        self.display.clear(color::BLACK);
        self.request_redraw = true;
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

    pub fn clear_image(&mut self, id: u8, (r, g, b): (u8, u8, u8)) -> Result<()> {
        let color = rgb_color(r, g, b);

        let Some(image) = self.images.get_mut(id) else {
            bail!("failed to get image id: {id}");
        };

        image.clear(color);

        if self.images.main == Some(id) {
            self.update_display();
        }

        Ok(())
    }

    pub fn blit_copy_image(
        &mut self,
        src_id: u8,
        dst_id: Option<u8>,
        src_x: i32,
        src_y: i32,
        width: i32,
        height: i32,
        dst_x: i32,
        dst_y: i32,
    ) -> Result<()> {
        let (src, dst) = self
            .images
            .get_pair(src_id, dst_id)
            .ok_or(anyhow!("failed to get image {src_id} & {dst_id:?}"))?;

        let (src_x, dst_x, width) = clip_axis(src_x, dst_x, width, src.width, dst.width);
        let (src_y, dst_y, height) = clip_axis(src_y, dst_y, height, src.height, dst.height);

        if width == 0 || height == 0 {
            return Ok(());
        }

        dst.blit_copy(src, src_x, src_y, dst_x, dst_y, width, height);

        if dst_id.is_none() || self.images.main == dst_id {
            self.update_display();
        }

        Ok(())
    }

    pub fn blit_blend_image(
        &mut self,
        src_id: u8,
        dst_id: Option<u8>,
        src_x: i32,
        src_y: i32,
        width: i32,
        height: i32,
        dst_x: i32,
        dst_y: i32,
    ) -> Result<()> {
        let (src, dst) = self
            .images
            .get_pair(src_id, dst_id)
            .ok_or(anyhow!("failed to get image {src_id} & {dst_id:?}"))?;

        let (src_x, dst_x, width) = clip_axis(src_x, dst_x, width, src.width, dst.width);
        let (src_y, dst_y, height) = clip_axis(src_y, dst_y, height, src.height, dst.height);

        if width == 0 || height == 0 {
            return Ok(());
        }

        dst.blit_blend(src, src_x, src_y, dst_x, dst_y, width, height);

        if dst_id.is_none() || self.images.main == dst_id {
            self.update_display();
        }

        Ok(())
    }
}

fn clip_axis(
    mut src_pos: i32,
    mut dst_pos: i32,
    mut len: i32,
    src_max: u32,
    dst_max: u32,
) -> (u32, u32, u32) {
    if src_pos.is_negative() {
        len += src_pos;
        src_pos = 0;
    }

    if dst_pos.is_negative() {
        len += dst_pos;
        src_pos -= dst_pos;
        dst_pos = 0;
    }

    let len = len.max(0) as u32;
    let src_pos = src_pos.max(0) as u32;
    let dst_pos = dst_pos.max(0) as u32;

    let src_bound = src_max.saturating_sub(src_pos);
    let dst_bound = dst_max.saturating_sub(dst_pos);

    let len = len.min(src_bound).min(dst_bound);

    (src_pos, dst_pos, len)
}

#[test]
fn test_clip_axis() {
    let (src, dst, len) = clip_axis(10, 20, 30, 100, 100);
    assert!(src == 10 && dst == 20 && len == 30);

    let (src, dst, len) = clip_axis(-10, 20, 30, 100, 100);
    assert!(src == 0 && dst == 20 && len == 20);

    let (src, dst, len) = clip_axis(10, -5, 20, 100, 100);
    assert!(src == 15 && dst == 0 && len == 15);

    let (src, dst, len) = clip_axis(-10, -5, 30, 100, 100);
    assert!(src == 5 && dst == 0 && len == 15);

    let (src, dst, len) = clip_axis(80, 10, 30, 100, 100);
    assert!(src == 80 && dst == 10 && len == 20);

    let (src, dst, len) = clip_axis(10, 90, 20, 100, 100);
    assert!(src == 10 && dst == 90 && len == 10);

    let (.., len) = clip_axis(-50, 0, 10, 100, 100);
    assert!(len == 0);
}
