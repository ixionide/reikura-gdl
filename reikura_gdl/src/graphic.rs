mod surface;

use anyhow::{Result, bail};
use reikura_util::image::rgb_color;

pub use self::surface::{Surface, color};

pub const MAX_IMAGE: usize = 128;

pub struct Images {
    main: Option<u8>,
    images: [Option<Surface>; MAX_IMAGE],
}

impl Images {
    pub fn new() -> Self {
        Self {
            main: None,
            images: std::array::from_fn(|_| None),
        }
    }

    #[inline]
    pub fn unset_main(&mut self) {
        self.main = None;
    }

    #[inline]
    pub fn set_main(&mut self, id: u8) {
        self.main = Some(id & MAX_IMAGE as u8);
    }

    #[inline]
    pub fn get_ref(&self, id: u8) -> Option<&Surface> {
        self.images[id as usize & MAX_IMAGE].as_ref()
    }

    #[inline]
    pub fn get_mut(&mut self, id: u8) -> Option<&mut Surface> {
        self.images[id as usize & MAX_IMAGE].as_mut()
    }

    #[inline]
    pub fn set(&mut self, id: u8, width: u32, height: u32, data: Option<&[u8]>) -> Result<()> {
        let image = match data {
            Some(bytes) => Surface::from_bytes(width, height, bytes)?,
            None => Surface::new(width, height),
        };

        self.images[id as usize & MAX_IMAGE] = Some(image);

        Ok(())
    }

    #[inline]
    pub fn get_main_ref(&mut self) -> Option<&Surface> {
        self.images[self.main? as usize].as_ref()
    }

    #[inline]
    pub fn get_main_mut(&mut self) -> Option<&mut Surface> {
        self.images[self.main? as usize].as_mut()
    }

    #[inline]
    pub fn get_image_pair(
        &mut self,
        src_id: u8,
        dst_id: Option<u8>,
    ) -> Option<(&mut Surface, &mut Surface)> {
        let indices = [src_id as usize, dst_id.or(self.main)? as usize];

        match self.images.get_disjoint_mut(indices) {
            Ok([Some(src), Some(dst)]) => Some((src, dst)),
            _ => None,
        }
    }
}

pub struct Graphics {
    pub display: Surface,
    _main_temp: Surface,
    _x: u32,
    _y: u32,
    images: Images,
    pub request_redraw: bool,
}

impl Graphics {
    pub fn new(view_width: u32, view_height: u32) -> Self {
        Self {
            display: Surface::new(view_width, view_height),
            _main_temp: Surface::new(view_width, view_height),
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
