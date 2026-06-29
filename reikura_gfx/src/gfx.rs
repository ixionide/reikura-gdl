use std::sync::Arc;

use winit::window::Window;

use crate::backend::{GraphicBackend, SoftwareRenderer};

pub struct GraphicEngine {
    backend: Box<dyn GraphicBackend>,
}

impl GraphicEngine {
    pub fn new(window: Arc<dyn Window>, width: u32, height: u32) -> anyhow::Result<Self> {
        // TODO: made hw renderer
        let renderer = SoftwareRenderer::new(window, width, height)?;

        Ok(Self {
            backend: Box::new(renderer),
        })
    }
}

impl std::ops::Deref for GraphicEngine {
    type Target = Box<dyn GraphicBackend>;

    fn deref(&self) -> &Self::Target {
        &self.backend
    }
}

impl std::ops::DerefMut for GraphicEngine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.backend
    }
}
