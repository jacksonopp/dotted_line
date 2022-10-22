use nannou::{prelude::*, color::named};

mod models;
use models::Model;

mod events;

fn main() {
    nannou::app(model).event(event).run()
}

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .size(1920, 1080)
        .view(view)
        .build()
        .unwrap();
    
    Model::new(window)
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { id: _, simple } => {
            if let Some(event) = simple {
                match event {
                    KeyReleased(Key::S) => events::screenshot(app),
                    _ => ()
                }
            }
        },

        Event::Update(update) => events::update(app, model, update),
        _ => ()
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(named::CORNSILK);
    draw.ellipse()
        .x_y(0.0, 0.0)
        .color(named::GREEN)
        .radius(20.0);

    draw.to_frame(app, &frame).unwrap();

}