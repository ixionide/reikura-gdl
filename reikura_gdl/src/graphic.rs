mod image;

use anyhow::{Result, bail};
use reikura_util::image::rgb_color;

pub use self::image::{Image, color};

pub const MAX_IMAGE: usize = 128;

pub struct Images {
    main: Option<u8>,
    images: [Option<Image>; MAX_IMAGE],
}

impl Images {
    fn new() -> Self {
        Self {
            main: None,
            images: std::array::from_fn(|_| None),
        }
    }

    #[inline]
    fn unset_main(&mut self) {
        self.main = None;
    }

    #[inline]
    fn set_main(&mut self, id: u8) {
        self.main = Some(id & MAX_IMAGE as u8);
    }

    #[inline]
    #[allow(dead_code)]
    fn get_ref(&self, id: u8) -> Option<&Image> {
        self.images[id as usize & MAX_IMAGE].as_ref()
    }

    #[inline]
    fn get_mut(&mut self, id: u8) -> Option<&mut Image> {
        self.images[id as usize & MAX_IMAGE].as_mut()
    }

    #[inline]
    fn set(&mut self, id: u8, width: u32, height: u32, data: Option<&[u8]>) -> Result<()> {
        let image = match data {
            Some(bytes) => Image::from_bytes(width, height, bytes)?,
            None => Image::new(width, height),
        };

        self.images[id as usize & MAX_IMAGE] = Some(image);

        Ok(())
    }

    #[inline]
    fn get_main_ref(&mut self) -> Option<&Image> {
        self.images[self.main? as usize].as_ref()
    }

    #[inline]
    #[allow(dead_code)]
    fn get_main_mut(&mut self) -> Option<&mut Image> {
        self.images[self.main? as usize].as_mut()
    }

    #[inline]
    #[allow(dead_code)]
    fn get_image_pair(
        &mut self,
        src_id: u8,
        dst_id: Option<u8>,
    ) -> Option<(&mut Image, &mut Image)> {
        let indices = [src_id as usize, dst_id.or(self.main)? as usize];

        match self.images.get_disjoint_mut(indices) {
            Ok([Some(src), Some(dst)]) => Some((src, dst)),
            _ => None,
        }
    }
}

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
            .blit_copy(0, 0, main_image.width, main_image.height, 0, 0, main_image);
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
}
