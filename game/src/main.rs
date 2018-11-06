pub mod app;

fn main() {
    let mut app = app::App::new();
    app.start();
    app.clear_screen();

    while app.state == app::AppState::Running {
        match app.get_event() {
            app::EventApp::Pass => {}
            app::EventApp::Quit => app.state = app::AppState::Exiting
        }
        app.clear_screen();
    }
    app.stop();
}
