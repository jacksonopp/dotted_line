use crate::models::Model;
use nannou::prelude::*;

pub fn update(_app: &App, _model: &mut Model, _update: Update) {
    // Do stuff
}

pub fn screenshot(app: &App) {
    // let r = format!("-{}", random_range(0, 1000000));
    let r = random_range(6, 9);
    let mut ext = String::from("-");

    // let forbidden = vec![')','(','*','&','^','%','$','#','@','!','~','.'];

    for _ in 0..r {
        let rand_char = random_ascii();
        ext.push(rand_char)
    }

    app.main_window()
        .capture_frame(app.exe_name().unwrap_or("file-".to_owned()) + ext.as_str() + ".png");
}
