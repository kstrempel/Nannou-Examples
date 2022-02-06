use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Agent {
    pos: Point2,
    lifetime: f32,
}

struct Model {
    _window: window::Id,
    agents: Vec<Agent>,
    lifetime: f32,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(WINDOW_SIZE as u32, WINDOW_SIZE as u32)
        .view(view)
        .build()
        .unwrap();
    let agents = Vec::new();
    Model {
        _window,
        agents,
        lifetime: 1.0,
    }
}

const WINDOW_SIZE: f32 = 1024.0;

fn new_random_window() -> f32 {
    (random_f32() - 0.5) * WINDOW_SIZE
}

fn new_random_pos() -> Point2 {
    Point2::new(new_random_window(), new_random_window())
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let lifetime = model.lifetime + 1.0;
    model.lifetime = lifetime;

    let mut agents = Vec::new();
    for a in model.agents.iter() {
        if a.lifetime < (WINDOW_SIZE * 10.0) {
            agents.push(Agent {
                pos: a.pos,
                lifetime: a.lifetime + 1.0,
            });
        }
    }

    if lifetime as u32 % 100 == 0 {
        agents.push(Agent {
            pos: new_random_pos(),
            lifetime: 1.0,
        })
    }

    model.agents = agents;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for a in model.agents.iter() {
        draw.ellipse()
            .stroke_weight(1.0)
            .stroke(BLACK)
            .xy(a.pos)
            .radius(a.lifetime as f32)
            .color(hsla(a.lifetime / 500.0, 0.5, 0.5, 0.01));
    }

    draw.to_frame(app, &frame).unwrap();
}
