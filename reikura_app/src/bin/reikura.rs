use reikura_app::ReikuraApp;
use winit::event_loop::EventLoop;

fn main() {
    let app_path = {
        let current_exe = std::env::current_exe().unwrap();
        current_exe.parent().unwrap().to_owned()
    };

    let reikura_app = ReikuraApp::new(app_path).unwrap();
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    event_loop.run_app(reikura_app).unwrap();
}
