use fltk::{app, button::Button, frame::Frame, group::Flex, prelude::*, window::Window};

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Oxy);
    let mut wind = Window::default().with_size(400, 300).with_label("Saage");
    let mut col = Flex::default_fill().column();
    col.set_margins(120, 80, 120, 80);
    let mut frame = Frame::default();
    let mut but = Button::default().with_label("Click me!");
    col.fixed(&but, 40);
    col.end();
    wind.end();
    wind.show();

    but.set_callback(move |_| frame.set_label("Hello world"));

    app.run().unwrap();
}