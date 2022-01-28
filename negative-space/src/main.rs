use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    sectors: Vec<Vec<Point2>>
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { 
        _window,
        sectors: Vec::new()
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut result = Vec::new();
    let mut circle_inner = Vec::new();
    let mut circle_outer = Vec::new();
    let mut rad = 0.0;
    loop {
        if rad > PI*2.0 {
            break;
        }

        circle_inner.push(pt2(rad.sin() * 10.0, rad.cos() * 10.0));
        circle_outer.push(pt2(rad.sin() * 15.0, rad.cos() * 15.0));

        rad += 0.01;
    }
    result.push(circle_inner.extend(circle_outer));
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
