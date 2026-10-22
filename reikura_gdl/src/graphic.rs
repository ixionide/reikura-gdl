mod surface;

use anyhow::{Result, bail};
use reikura_util::image::rgb_color;

pub use self::surface::Surface;

pub const MAX_IMAGE: usize = 128;

pub struct Graphics {
    #[allow(unused)]
    display_image: Surface,
    #[allow(unused)]
    temp_image: Surface,
    main_image_id: Option<u8>,
    main_image_x: u32,
    main_image_y: u32,
    images: [Option<Surface>; MAX_IMAGE],
}

impl Graphics {
    #[inline]
    fn get_image_ref(&self, id: u8) -> Option<&Surface> {
        if id as usize >= MAX_IMAGE {
            return None;
        }

        self.images[id as usize].as_ref()
    }

    #[inline]
    fn get_image_mut(&mut self, id: u8) -> Option<&mut Surface> {
        if id as usize >= MAX_IMAGE {
            return None;
        }

        self.images[id as usize].as_mut()
    }

    #[inline]
    fn get_image_pair(&mut self, src_id: u8, dst_id: u8) -> Option<(&Surface, &mut Surface)> {
        let indices = [src_id as usize, dst_id as usize];

        match self.images.get_disjoint_mut(indices) {
            Ok([Some(src), Some(dst)]) => Some((src, dst)),
            _ => None,
        }
    }

    pub fn unset_main_image(&mut self) {
        self.main_image_id = None;
    }

    pub fn set_main_image(&mut self, id: u8, x: u32, y: u32) {
        self.main_image_id = Some(id);
        self.main_image_x = x;
        self.main_image_y = y;
    }

    pub fn clear_image(&mut self, id: u8, (r, g, b): (u8, u8, u8)) -> Result<()> {
        let color = rgb_color(r, g, b);

        let Some(image) = self.get_image_mut(id) else {
            bail!("failed to get image id: {id}");
        };

        image.clear(color);

        Ok(())
    }
}
