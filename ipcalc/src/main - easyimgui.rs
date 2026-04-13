use easy_imgui_window::{easy_imgui::{self as imgui, IntoCStr}, winit, AppHandler, Application, Args, EventResult};
use winit::{event::WindowEvent, event_loop::EventLoop};

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let mut main = AppHandler::<App>::default();
    main.attributes().title = String::from("IPCalc");

    event_loop.run_app(&mut main).unwrap();
}

struct App;

impl Application for App {
    type UserEvent = ();
    type Data = ();
    fn new(_: Args<()>) -> App {
        App
    }
    fn window_event(&mut self, args: Args<()>, _event: WindowEvent, res: EventResult) {
        if res.window_closed {
            args.event_loop.exit();
        }
    }
}

impl imgui::UiBuilder for App {
    fn do_ui(&mut self, ui: &imgui::Ui<Self>) {
        #[cfg(feature = "docking")]
        {
            ui.dock_space_over_viewport(0, imgui::DockNodeFlags::None);
        }

        // ui.show_demo_window(None);
		// ui.set_window_size(imgui::vec2(770.0, 264.0), imgui::Cond::Always);
		// ui.set_cursor_pos(imgui::vec2(12.0, 35.0));
		ui.input_text_config(imgui::lbl("IP Address"), &mut String::from("172.16.1.32"));

    }
}