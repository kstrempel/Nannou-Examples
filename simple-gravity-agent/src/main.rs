use nannou::geom::Vec2;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Clone, Debug, Copy)]
struct Agent {
    pos: Point2,
    lifetime: f32,
    steer: Vec2,
    speed: f32,
    radius: f32,
}

impl Agent {
    fn new() -> Agent {
        Agent {
            pos: new_random_pos(),
            lifetime: 1.0,
            steer: Vec2::new(random_f32() - 0.5, random_f32() - 0.5),
            speed: random_f32() * 5.0 + 5.0,
            radius: random_f32() * 20.0 + 5.0,
        }
    }
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
    let mut agents = Vec::new();
    agents.push(Agent::new());
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
    model.lifetime = model.lifetime + 1.0;

    let mut agents = Vec::new();
    for a in model.agents.iter() {
        if a.lifetime < 1000.0 {
            let direction = (a.pos - pt2(0.0, 0.0)).normalize();
            let steer = (direction + a.steer).normalize() * a.speed;
            let newpos = a.pos - steer;
            let mut new_agent = a.clone();
            new_agent.pos = newpos;
            new_agent.lifetime = a.lifetime + 1.0;
            new_agent.steer = steer;
            agents.push(new_agent);
        }
    }

    if model.lifetime as u32 % 10 == 0 {
        agents.push(Agent::new())
    }

    model.agents = agents;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.rect()
        .w_h(WINDOW_SIZE, WINDOW_SIZE)
        .color(srgba(0.8, 0.8, 0.7, 0.02));

    for a in model.agents.iter() {
        draw.ellipse()
            .xy(a.pos)
            .radius(a.radius)
            .color(srgb8(0, 0, 0));

        draw.ellipse()
            .xy(a.pos)
            .radius(a.radius - 5.0)
            .stroke(srgb8(40, 255, 0))
            .stroke_weight(3.0)
            .color(srgb8(0, 0, 0));
    }

    draw.to_frame(app, &frame).unwrap();
}
