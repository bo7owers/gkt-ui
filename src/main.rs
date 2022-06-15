use rand::random;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Box, Orientation};

fn main() {
let app= Application::builder()
    .application_id("com.bo7owers.gkt-ui")
    .build();
app.connect_activate(create_ui);
    app.run();
}

fn create_ui(app: &Application){
    let button = Button::builder()
    .label("Sup?")
    .margin_top(15)
    .margin_bottom(15)
    .margin_start(15)
    .margin_end(15)
    .build();


    let window = ApplicationWindow::builder()
    .title("GTK UI Demo")
    .application(app)
    .child(&button)
    .build();

    button.connect_clicked(move |_| coin_toss());

    window.show();
}

fn coin_toss(){
    if random(){
        println!("Heads");
    } else {
        println!("Tails")
    }
}