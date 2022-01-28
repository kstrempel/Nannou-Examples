use nannou::prelude::*;

fn main() {
    nannou::app(model).loop_mode(LoopMode::Wait).run();
}

struct Thing {
    sectors: Vec<Vec<Point2>>,
    rotate: f32,
    color: Hsl
}


struct Model {
    _window: window::Id,
    things: Vec<Thing>
}

fn broken_circle(radius: f32, size: f32) -> Vec<Vec<Point2>> {
    let mut circle_inner = Vec::new();
    let mut circle_outer = Vec::new();
    let mut rad = 0.0;
    loop {
        if rad > PI * 1.5 {
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
        .view(view)
        .mouse_pressed(mouse_pressed)
        .build()
        .unwrap();
    let mut model = Model {
        _window,
        things: Vec::new(),
    };

    update_model(app, &mut model);

    model
}

fn mouse_pressed(app: &App, model: &mut Model, _mouse_button: MouseButton) {
    update_model(app, model);
}

fn update_model(app: &App, model: &mut Model) {
    let max_radius = (app.window_rect().h() / 2.0) as i32;
    let mut result = Vec::new();
    for i in (50..max_radius).step_by(20) {
        result.push(Thing { 
            sectors: broken_circle(i as f32, 10.0),
            rotate: random_f32() * TAU,
            color: hsl(random_f32(), 0.7, 0.1)
        });
    }
    model.things = result;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let wr = app.window_rect();
    let left  = Rect::from_x_y_w_h(-1.0 * wr.w() / 2.0, 0.0, wr.w(), wr.h());
    let right = Rect::from_x_y_w_h( wr.w() / 2.0, 0.0, wr.w(), wr.h());

    draw.background().color(STEELBLUE);

    draw.scissor(left)
        .rect()
        .x_y(left.x(), left.y())
        .w_h(left.w(), left.h())
        .color(gray(0.7));

    draw.scissor(right)
        .rect()
        .x_y(right.x(), right.y())
        .w_h(right.w(), right.h())
        .color(BLACK);
    
    for t in model.things.iter() {
        for s in t.sectors.iter() {
            draw.scissor(left)
                .polygon()
                .points_colored(s.iter().map(|&p| (p, t.color)))
                .rotate(t.rotate);

            draw.scissor(right)
                .polygon()
                .points_colored(s.iter().map(|&p| (p, WHITE)))
                .rotate(t.rotate);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
