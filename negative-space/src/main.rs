use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Thing {
    sectors: Vec<Vec<Point2>>,
    rotate: f32,
    speed: f32,
    color: Hsl
}

struct Model {
    _window: window::Id,
    things: Vec<Thing>,
    rotate: f32
}

fn broken_circle(radius: f32, size: f32) -> Vec<Vec<Point2>> {
    let mut circle_inner = Vec::new();
    let mut circle_outer = Vec::new();
    let mut rad = 0.0;
    loop {
        if rad > (0.01 * TAU) {
            break;
        }

        circle_inner.push(pt2(rad.sin() * radius, rad.cos() * radius));
        circle_outer.push(pt2(
            rad.sin() * (radius + size),
            rad.cos() * (radius + size),
        ));

        rad += 0.01;
    }

    circle_outer.reverse();
    vec![[circle_inner, circle_outer].concat()]
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(512, 600)
        .view(view)
        .build()
        .unwrap();

    let max_radius = (app.window_rect().h() / 1.0) as i32;
    let mut result = Vec::new();
    for i in (50..max_radius).step_by(10) {
        result.push(Thing { 
            sectors: broken_circle(i as f32, 10.0),
            rotate: random_f32() * TAU,
            color: hsl(random_f32(), 0.7, 0.1),
            speed: (random_range(1.0, 5.0) as u32) as f32
        });
    }

    Model {
        _window,
        things: result,
        rotate: 0.0
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.rotate += 0.01;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let wr = app.window_rect();
    let left  = Rect::from_x_y_w_h(-1.0 * wr.w() / 2.0, 0.0, wr.w(), wr.h());
    let right = Rect::from_x_y_w_h( wr.w() / 2.0, 0.0, wr.w(), wr.h());

    draw.scissor(left)
        .rect()
        .x_y(left.x(), left.y())
        .w_h(left.w(), left.h())
        .color(hsla(0.5, 1.0, 0.5, 0.01));

    draw.scissor(right)
        .rect()
        .x_y(right.x(), right.y())
        .w_h(right.w(), right.h())
        .color(hsla(0.0,1.0,0.0,0.1));

    for t in model.things.iter() {
        for s in t.sectors.iter() {
            let rotate = t.rotate + (model.rotate * t.speed);
            draw.scissor(left)
                .polygon()
                .points_colored(s.iter().map(|&p| (p, t.color)))
                .rotate(rotate);

            draw.scissor(right)
                .polygon()
                .points_colored(s.iter().map(|&p| (p, WHITE)))
                .rotate(rotate);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
