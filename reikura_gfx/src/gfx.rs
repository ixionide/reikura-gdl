use crate::backend::GraphicBackend;

pub struct GraphicEngine {
    backend: Box<dyn GraphicBackend>,
}

impl std::ops::DerefMut for GraphicEngine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.backend
    }
}

impl std::ops::Deref for GraphicEngine {
    type Target = Box<dyn GraphicBackend>;

    fn deref(&self) -> &Self::Target {
        &self.backend
    }
}
