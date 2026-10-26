use std::{ops::DerefMut, path::PathBuf, sync::Arc};

use anyhow::anyhow;
use reikura_gdl::{Manifest, Vm, graphic};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{StartCause, WindowEvent},
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes, WindowId},
};

use crate::{Renderer, Viewport};

pub struct ReikuraContext {
    pub window: Arc<dyn Window>,
    pub renderer: Renderer,
    pub vm: Vm,
}

pub struct ReikuraApp {
    manifest: Manifest,
    viewport: Viewport,
    context: Option<ReikuraContext>,
    request_redraw: bool,
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
            .ok_or(anyhow!("no .suf found"))?;

        let manifest = Manifest::parse(suf_path)?;
        let (w, h) = manifest.view_size;

        Ok(Self {
            manifest,
            viewport: Viewport::new(w, h),
            context: None,
            request_redraw: true,
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
            let display_handle = event_loop.owned_display_handle();
            let window: Arc<dyn Window> = event_loop
                .create_window(attr)
                .expect("failed to create window")
                .into();
            let vm = Vm::new(self.manifest.clone()).expect("failed to start vm");
            let renderer =
                Renderer::new(display_handle).expect("failed to create renderer context");

            window.set_visible(true);
            window.request_redraw();

            self.context = Some(ReikuraContext {
                window,
                renderer,
                vm,
            })
        }
    }

    fn can_create_surfaces(&mut self, _event_loop: &dyn ActiveEventLoop) {
        let Some(ctx) = &mut self.context else {
            return;
        };

        ctx.renderer
            .create_surface(Arc::clone(&ctx.window))
            .unwrap();
    }

    fn destroy_surfaces(&mut self, _event_loop: &dyn ActiveEventLoop) {
        let Some(ctx) = &mut self.context else {
            return;
        };

        ctx.renderer.destroy_surface();
    }

    fn window_event(
        &mut self,
        event_loop: &dyn ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(ctx) = &mut self.context else {
            return;
        };

        let ReikuraContext {
            window,
            renderer,
            vm,
        } = ctx;

        if window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                if vm.gfx.request_redraw || self.request_redraw {
                    let vwpt = &self.viewport;
                    let surface = renderer.get_surface().unwrap();
                    let mut surface_buffer = surface.buffer_mut().unwrap();
                    let (width, height) = window.surface_size().into();

                    let mut surface_image =
                        graphic::Image::from_buffer(width, height, surface_buffer.deref_mut())
                            .unwrap();

                    surface_image.clear(graphic::color::BLACK);
                    surface_image.blit_copy(&vm.gfx.display, 0, 0, vwpt.x, vwpt.y, vwpt.w, vwpt.h);

                    surface_buffer.present().unwrap();
                    vm.gfx.request_redraw = false;
                    self.request_redraw = false;
                }

                vm.update().unwrap();
                window.request_redraw();
            }
            WindowEvent::SurfaceResized(size) => {
                if [size.width, size.height].contains(&0) {
                    return;
                }

                renderer.resize_surface(size).unwrap();
                self.viewport = Viewport::letterbox(size.into(), self.manifest.view_size);
                self.request_redraw = true;
            }
            WindowEvent::PointerMoved { position, .. } => {
                let mouse_pos = self
                    .viewport
                    .map_position(self.manifest.view_size, position.into());
                vm.input.mouse_pos = Some(mouse_pos);
            }
            WindowEvent::PointerLeft { .. } | WindowEvent::Focused(false) => {
                vm.input.mouse_pos = None;
            }
            _ => (),
        }
    }
}
