use std::{path::PathBuf, sync::Arc};

use anyhow::anyhow;
use reikura_gdl::{Manifest, Vm};
use reikura_gfx::GraphicEngine;
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
    vm: Option<Vm>,
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
            vm: None,
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
        if cause == StartCause::Init {
            let attr = self.window_attributes();
            let window: Arc<dyn Window> = event_loop
                .create_window(attr)
                .expect("failed to create window")
                .into();

            // TODO: logging
            let (w, h) = self.manifest.view_size;
            let gfx = GraphicEngine::new(window.clone(), w, h);
            let gfx = gfx.expect("failed to start gfx context");
            let vm = Vm::new(self.manifest.clone(), gfx).expect("failed to start vm");

            window.set_visible(true);
            window.request_redraw();
            self.window = Some(window);
            self.vm = Some(vm);
        }
    }

    fn can_create_surfaces(&mut self, _event_loop: &dyn ActiveEventLoop) {
        let Some(window) = &self.window else {
            return;
        };

        let Some(vm) = &mut self.vm else {
            return;
        };

        vm.gfx
            ._create_surface(window.clone())
            .expect("failed to create surface");
    }

    fn destroy_surfaces(&mut self, _event_loop: &dyn ActiveEventLoop) {
        let Some(vm) = &mut self.vm else {
            return;
        };

        vm.gfx._destroy_surface();
    }

    fn window_event(
        &mut self,
        event_loop: &dyn ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = &self.window else {
            return;
        };

        if window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                let Some(vm) = &mut self.vm else {
                    return;
                };

                vm.update().unwrap();
                vm.gfx._render().unwrap();
                window.request_redraw();
            }
            _ => (),
        }
    }
}
