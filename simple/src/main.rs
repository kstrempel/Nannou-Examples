use nannou::prelude::*;

struct Bird {
    pos: Vec2,
    force: Vec2,
    color: Hsl
}

struct Model {
    birds: Vec<Bird>
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(1024, 1024).view(view).build().unwrap();
    let mut birds = Vec::new();
    for _ in 1..500 {
        let bird = Bird { 
            pos: vec2(0.0, 0.0),
            force: vec2( random_f32() - 0.5, random_f32() - 0.5),
            color: hsl(random(), 0.5, 0.3)
        };
        birds.push(bird);
    }

    Model {
        birds: birds
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let rect = app.window_rect();
    for bird in model.birds.iter_mut() {
        bird.pos += bird.force * vec2(3.0,3.0);
        if bird.pos.x < rect.left() || bird.pos.x > rect.right(){ 
            bird.force = bird.force * vec2(-1.0,1.0)
        }
        if bird.pos.y < rect.bottom() || bird.pos.y > rect.top() { 
            bird.force = bird.force * vec2(1.0,-1.0)
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.rect()
        .w_h(1024.0, 1024.0)
        .color(srgba(0.0,0.0,0.0,0.05));

    for bird in model.birds.iter() {
        draw.ellipse()
            .radius(5.0)
            .xy(bird.pos)
            .color(bird.color);
    }

    draw.to_frame(app, &frame).unwrap();
}
