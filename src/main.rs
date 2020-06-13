extern crate gtk;
use gtk::prelude::*;
use gtk::{Button, Window, WindowType};
use gtk::{ButtonsType, DialogFlags, MessageType, MessageDialog};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("v100");
    window.set_default_size(350, 70);
    let button = Button::new_with_label("Calculate");
    window.add(&button);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    button.connect_clicked(|_| {
        let mut v_staro = 0.0f64;
        let mut v_novo = 0.0f64;
        let korak = 0.00001f64;
        let mut vreme = korak;
        let snaga = 100.0f64;
        let masa = 20.0f64;
        let korakE = snaga * korak;
    
        loop {
            v_novo = (2.0 * korakE / masa + v_staro.powf(2.0)).sqrt();
            if v_novo > 0.5 { break }
            v_staro = v_novo;
            vreme += korak;
        }
    
    	let dlg = MessageDialog::new(None::<&Window>,
                       DialogFlags::empty(),
                       MessageType::Info,
                       ButtonsType::Ok,
                       &format!("vreme je(sec) {}",vreme));
        dlg.run();
        dlg.destroy();
    });

    gtk::main();
}

