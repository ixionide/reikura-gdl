mod surface;

pub use self::surface::Surface;

pub const MAX_SURFACE: usize = 128;

pub struct GraphicManager {
    #[allow(unused)]
    display_surface: Surface,
    #[allow(unused)]
    temp_surface: Surface,
    pub target_surface: Option<u8>,
    pub surfaces: [Option<Surface>; MAX_SURFACE],
}

impl GraphicManager {
    pub fn new(w: u32, h: u32) -> Self {
        GraphicManager {
            display_surface: Surface::new(w, h),
            temp_surface: Surface::new(w, h),
            target_surface: None,
            surfaces: std::array::from_fn(|_| None),
        }
    }

    pub fn surface_pair(&mut self, src_id: u8, dst_id: u8) -> Option<(&Surface, &mut Surface)> {
        let pair = [src_id as usize, dst_id as usize];

        let Ok([src, dst]) = self.surfaces.get_disjoint_mut(pair) else {
            return None;
        };

        src.as_ref().zip(dst.as_mut())
    }
}
