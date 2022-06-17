use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Label, Orientation};
use rand::random;

fn main() {
    let app = Application::builder()
        .application_id("com.bo7owers.gkt-ui")
        .build();
    app.connect_activate(create_ui);
    app.run();
}

fn create_ui(app: &Application) {
    let label = Label::builder()
        .label("How lucky are you? If the coin lands in Tails, you win; else, I win.")
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();

    let response = Label::builder()
        .label("")
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();

    let button = Button::builder()
        .label("Try me")
        .margin_top(15)
        .margin_bottom(15)
        .margin_start(15)
        .margin_end(15)
        .build();

    let content = Box::new(Orientation::Vertical, 0);
    content.append(&label);
    content.append(&response);
    content.append(&button);

    let window = ApplicationWindow::builder()
        .title("GTK UI Demo")
        .application(app)
        .child(&content)
        .build();

    button.connect_clicked(move |_| coin_toss(&response));

    window.show();
}

fn coin_toss(response: &Label) {
    if random() {
        response.set_text("Heads, I win!");
    } else {
        response.set_text("Tails, you win!");
    }
}
