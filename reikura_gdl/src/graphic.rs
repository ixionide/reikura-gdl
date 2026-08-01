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
    pub fn surface_pair(&mut self, src_id: u8, dst_id: u8) -> Option<(&Surface, &mut Surface)> {
        let indices = [src_id as usize, dst_id as usize];

        match self.surfaces.get_disjoint_mut(indices) {
            Ok([Some(src), Some(dst)]) => Some((src, dst)),
            _ => None,
        }
    }
}
