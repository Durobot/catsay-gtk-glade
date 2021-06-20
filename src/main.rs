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

    // Inputs
    let message_input: gtk::Entry = builder
        .get_object("message_input").unwrap();
    let is_dead_switch: gtk::Switch = builder
        .get_object("is_dead_switch").unwrap();
    // Submit button
    let button: gtk::Button = builder
        .get_object("generate_btn").unwrap();
    // Outputs
    let message_output: gtk::Label = builder
        .get_object("message_output").unwrap();
    let image_output: gtk::Image = builder
        .get_object("image_output").unwrap();
    let image_output_clone = image_output.clone(); // because Gtk-rs is a wrapper around the C GTK library, doing
                                                   // a Rust clone on an Gtk-rs object only copies the pointer
    // We are not cloning message_input and message_output simply because we don’t need to use them after we define
    // the callback function. If we did need to use them after moving them into the callback, we would clone them just like
    // we did for image_output.

    //button.connect_clicked(|_| // error[E0373]: closure may outlive the current function, but it borrows `message_input`,
                                 // which is owned by the current function
    button.connect_clicked(move |_| // help: to force the closure to take ownership of `message_input`
    {                               //        (and any other referenced variables), use the `move` keyword
        message_output.set_text(&format!("{}\n   \\\n     \\", message_input.get_text().as_str()));
        let is_dead = is_dead_switch.get_active();
        if is_dead
        { image_output.set_from_file("./images/cat_dead.png"); }
        else
        { image_output.set_from_file("./images/cat.png"); }
        //image_output_clone.show();
        image_output.show();
    });
    window.show_all();
    // We want the image to be initially hidden, which is why we call hide() AFTER show_all()
    //image_output.hide(); // error[E0382]: borrow of moved value: `image_output` (i.e. after the closure above with move modifier)
    image_output_clone.hide();
}
