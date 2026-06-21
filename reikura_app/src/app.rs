use std::{path::PathBuf, sync::Arc};

use anyhow::anyhow;
use reikura_gdl::Manifest;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{StartCause, WindowEvent},
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes, WindowId},
};

pub struct ReikuraApp {
    manifest: Manifest,
    window: Option<Arc<dyn Window>>,
}

impl ReikuraApp {
    pub fn new(app_path: PathBuf) -> anyhow::Result<Self> {
        let suf_ext = |path: &PathBuf| {
            let ext = path.extension();
            ext.is_some_and(|ext| ext.eq_ignore_ascii_case("suf"))
        };

        let suf_path = app_path
            .read_dir()?
            .filter_map(Result::ok)
            .map(|it| it.path())
            .filter(|it| it.is_file())
            .find(suf_ext)
            .ok_or(anyhow!("no suf file found"))?;

        let manifest = Manifest::parse(suf_path)?;

        Ok(Self {
            manifest,
            window: None,
        })
    }

    pub fn window_attributes(&self) -> WindowAttributes {
        let (w, h) = self.manifest.view_size;

        WindowAttributes::default()
            .with_title(&self.manifest.title)
            .with_surface_size(PhysicalSize::new(w, h))
    }
}

impl ApplicationHandler for ReikuraApp {
    fn new_events(&mut self, event_loop: &dyn ActiveEventLoop, cause: StartCause) {
        match cause {
            StartCause::Init => {
                let attr = self.window_attributes();
                let window = event_loop
                    .create_window(attr)
                    .expect("failed to create window");

                window.set_visible(true);
                self.window = Some(Arc::from(window));
            }
            _ => (),
        }
    }

    fn can_create_surfaces(&mut self, _event_loop: &dyn ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _event_loop: &dyn ActiveEventLoop,
        _window_id: WindowId,
        _event: WindowEvent,
    ) {
        ()
    }
}
