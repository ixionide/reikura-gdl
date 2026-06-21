use reikura_app::ReikuraApp;
use winit::event_loop::EventLoopBuilder;

fn main() {
    let app_path = {
        let current_exe = std::env::current_exe().unwrap();
        current_exe.parent().unwrap().to_owned()
    };

    let reikura_app = ReikuraApp::new(app_path).unwrap();
    let event_loop = EventLoopBuilder::default().build().unwrap();

    event_loop.run_app(reikura_app).unwrap();
}
