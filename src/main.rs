use gio::prelude::*;
use gtk::prelude::*;
use std::env::args;

fn main()
{
    let application = gtk::Application::new(
        Some("com.shinglyu.catsay-gui-glade"),
        Default::default()).expect("Failed to initialize GTK");
    application.connect_activate(|app|
    {
        build_ui(app);
    });
    application.run(&args().collect::<Vec<_>>());
}

fn build_ui(app: &gtk::Application)
{
    let glade_src = include_str!("layout.glade"); // Load the conents of layout.glade into this string variable
    // - function or associated item not found in `gtk::Builder`
    // - help: there is an associated function with a similar name: `from_string`
    //let builder = gtk::Builder::new_from_string(glade_src);
    let builder = gtk::Builder::from_string(glade_src); // Create GTK widget objects from the XML contained in the string
    let window: gtk::Window = builder.get_object("applicationwindow1").unwrap(); // Get the Window object by ID (see layout.glade)
    // - expected enum `Option`, found `&gtk::Application`
    // - help: try using a variant of the expected enum: `Some(app)`
    //window.set_application(app);
    window.set_application(Some(app)); // Tell the Window which application object it belongs to
    window.show_all();
}
