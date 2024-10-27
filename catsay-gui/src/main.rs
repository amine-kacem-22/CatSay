use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as gtkBox, Image, Label, Orientation};

fn main() {
    let app = Application::new(
        Some("com.aminekacem.catsay-gui"),
        Default::default()
    );
    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Catsay");
        window.set_default_size(350, 200); 
        let layout_box = gtkBox::new(Orientation::Vertical, 0);
        let label = Label::new(
            Some("meow \n   \\\n     \\")
        );
        layout_box.add(&label);
        let cat_image = Image::from_file(
            "./images/cat.png"
        );
        layout_box.add(&cat_image);
        window.add(&layout_box);
        window.show_all();
    });
    app.run();

}
