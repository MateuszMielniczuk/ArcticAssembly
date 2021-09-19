use gtk::prelude::*;

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);

    window.set_title("Arctic Assembly");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let button = gtk::Button::with_label("Make boom!");

    window.add(&button);

    window.show_all();
}

fn main() {
    let app = gtk::Application::new(
        Some("com.mateuszmmi.arctic_assembly"),
        Default::default()
    );

    app.connect_activate(build_ui);

    app.run();
}