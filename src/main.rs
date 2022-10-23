use nannou::{color::named, prelude::*};

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
                    _ => (),
                }
            }
        }

        Event::Update(update) => events::update(app, model, update),
        _ => (),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(named::CORNSILK);

    for line in &model.lines {
        // draw.polyline()
        //     .weight(line.thickness)
        //     .points(line.points.clone())
        //     .color(BLACK);

        line.points.iter().for_each(|p| {
            draw.ellipse()
                .radius(line.thickness)
                .xy(*p)
                .color(BLACK);
        })
    }

    draw.to_frame(app, &frame).unwrap();
}
