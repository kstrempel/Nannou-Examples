use nannou::prelude::*;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

fn main() {
    nannou::app(model).run();
}

#[derive(Copy, Clone)]
enum ThingType {
    CIRCLE,
    CIRCLEFILLED,
    BOX,
    BOXFILLED,
    NOTHING,
}

struct Thing {
    rect: Rect,
    color: Hsl,
    drawtype: ThingType,
}

struct Model {
    _window: window::Id,
    things: Vec<Thing>,
}

fn shrink(rect: &Rect<f32>, gap: f32) -> Rect<f32> {
    Rect::from_x_y_w_h(rect.x(), rect.y(), rect.w() - gap, rect.h() - gap)
}

fn update_model(r: Rect<f32>) -> Vec<Thing> {
    let mut things = vec![];

    let choices = [
        ThingType::BOX,
        ThingType::BOXFILLED,
        ThingType::CIRCLE,
        ThingType::CIRCLEFILLED,
        ThingType::NOTHING,
    ];
    let weights = [2, 1, 2, 1, 1];
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = thread_rng();

    for rect in r.subdivisions_iter() {
        for rect in rect.subdivisions_iter() {
            for rect in rect.subdivisions_iter() {
                for rect in rect.subdivisions_iter() {
                    let thing = Thing {
                        rect: shrink(&rect, 20.0),
                        color: hsl(random_f32(), 0.5, 0.5),
                        drawtype: choices[dist.sample(&mut rng)],
                    };

                    things.push(thing);
                }
            }
        }
    }

    things
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(1024, 1024)
        .view(view)
        .mouse_pressed(update)
        .build()
        .unwrap();

    let r = shrink(&app.window_rect(), 100.0);
    Model {
        _window,
        things: update_model(r)
    }    
}

fn update(app: &App, model: &mut Model, _mouse_button: MouseButton) {
    let r = shrink(&app.window_rect(), 100.0);
    model.things = update_model(r)
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(srgb8(33, 33, 33));
    for t in model.things.iter() {
        let r = t.rect;
        match t.drawtype {
            ThingType::BOX => {
                draw.rect()
                    .x_y(r.x(), r.y())
                    .w_h(r.w(), r.h())
                    .no_fill()
                    .stroke(t.color)
                    .stroke_weight(2.5);
            }
            ThingType::BOXFILLED => {
                draw.rect()
                    .x_y(r.x(), r.y())
                    .w_h(r.w(), r.h())
                    .color(t.color);
            }
            ThingType::CIRCLE => {
                draw.ellipse()
                    .x_y(r.x(), r.y())
                    .radius(r.w() / 2.0)
                    .no_fill()
                    .stroke(t.color)
                    .stroke_weight(2.5);
            }
            ThingType::CIRCLEFILLED => {
                draw.ellipse()
                    .x_y(r.x(), r.y())
                    .radius(r.w() / 2.0)
                    .color(t.color);
            }
            ThingType::NOTHING => {}
        };
    }

    draw.to_frame(app, &frame).unwrap();
}
