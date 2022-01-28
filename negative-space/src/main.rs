use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    sectors: Vec<Vec<Point2>>,
}

fn broken_circle(radius: f32, size: f32) -> Vec<Vec<Point2>> {
    let mut circle_inner = Vec::new();
    let mut circle_outer = Vec::new();
    let mut rad = 0.0;
    loop {
        if rad > PI * 2.0 {
            break;
        }

        circle_inner.push(pt2(
            rad.sin() * radius, 
            rad.cos() * radius));
        circle_outer.push(pt2(
            rad.sin() * (radius + size),
            rad.cos() * (radius + size),
        ));

        rad += 0.01;
    }

    vec![[circle_inner, circle_outer].concat()]
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model {
        _window,
        sectors: Vec::new(),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let max_radius = (app.window_rect().h() / 2.0) as i32;
    let mut result = Vec::new();
    for i in (10..max_radius).step_by(40) {
        result = [result, broken_circle(i as f32, 15.0)].concat();
    }
    model.sectors = result;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    for sector in model.sectors.iter() {
        draw.polygon()
            .points_colored(sector.iter().map(|&p| (p, STEELBLUE)));
    }

    draw.to_frame(app, &frame).unwrap();
}
