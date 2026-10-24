mod surface;

use anyhow::{Result, bail};
use reikura_util::image::rgb_color;

pub use self::surface::{Surface, color};

pub const MAX_IMAGE: usize = 128;

pub struct Graphics {
    pub display: Surface,
    #[allow(unused)]
    main_temp: Surface,
    main_image_id: Option<u8>,
    main_image_x: u32,
    main_image_y: u32,
    images: [Option<Surface>; MAX_IMAGE],
    pub request_redraw: bool,
}

impl Graphics {
    pub fn new(view_width: u32, view_height: u32) -> Self {
        Self {
            display: Surface::new(view_width, view_height),
            main_temp: Surface::new(view_width, view_height),
            main_image_id: None,
            main_image_x: 0,
            main_image_y: 0,
            images: std::array::from_fn(|_| None),
            request_redraw: false,
        }
    }

    #[inline]
    #[allow(unused)]
    fn get_image_ref(&self, id: u8) -> Option<&Surface> {
        self.images[id as usize & MAX_IMAGE].as_ref()
    }

    #[inline]
    fn get_image_mut(&mut self, id: u8) -> Option<&mut Surface> {
        self.images[id as usize & MAX_IMAGE].as_mut()
    }

    #[inline]
    #[allow(unused)]
    fn get_image_pair(&mut self, src_id: u8, dst_id: u8) -> Option<(&mut Surface, &mut Surface)> {
        let indices = [src_id as usize, dst_id as usize];

        match self.images.get_disjoint_mut(indices) {
            Ok([Some(src), Some(dst)]) => Some((src, dst)),
            _ => None,
        }
    }

    #[inline]
    fn get_display_pair(&mut self) -> Option<(&mut Surface, &mut Surface)> {
        let main_image_id = self.main_image_id? as usize & MAX_IMAGE;
        let main_image = self.images[main_image_id].as_mut()?;

        Some((main_image, &mut self.display))
    }

    #[inline]
    fn update_display(&mut self) {
        let Some((main_image, display)) = self.get_display_pair() else {
            todo!();
        };

        display.blit_copy(0, 0, main_image.width, main_image.height, 0, 0, main_image);
        self.request_redraw = true
    }

    pub fn unset_main_image(&mut self) {
        self.main_image_id = None;
        self.display.clear(color::BLACK);
        self.request_redraw = true;
    }

    pub fn set_main_image(&mut self, id: u8, x: u32, y: u32) {
        self.main_image_id = Some(id);
        self.main_image_x = x;
        self.main_image_y = y;

        self.update_display();
    }

    pub fn new_image(&mut self, id: u8, width: u32, height: u32) {
        self.images[id as usize & MAX_IMAGE] = Some(Surface::new(width, height));

        if self.main_image_id == Some(id) {
            self.update_display();
        }
    }

    pub fn set_image(&mut self, id: u8, width: u32, height: u32, data: &[u8]) -> Result<()> {
        self.images[id as usize & MAX_IMAGE] = Some(Surface::from_bytes(width, height, data)?);

        if self.main_image_id == Some(id) {
            self.update_display();
        }

        Ok(())
    }

    pub fn clear_image(&mut self, id: u8, (r, g, b): (u8, u8, u8)) -> Result<()> {
        let color = rgb_color(r, g, b);

        let Some(image) = self.get_image_mut(id) else {
            bail!("failed to get image id: {id}");
        };

        image.clear(color);

        if self.main_image_id == Some(id) {
            self.update_display();
        }

        Ok(())
    }
}
