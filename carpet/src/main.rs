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
    background: Hsl
}

fn padding(rect: &Rect<f32>, gap: f32) -> Rect<f32> {
    Rect::from_x_y_w_h(rect.x(), rect.y(), rect.w() - gap, rect.h() - gap)
}

fn update_model(r: Rect<f32>) -> Vec<Thing> {
    let mut things = vec![];

    let drawing_choices = [
        ThingType::BOX,
        ThingType::BOXFILLED,
        ThingType::CIRCLE,
        ThingType::CIRCLEFILLED,
        ThingType::NOTHING,
    ];
    let drawing_weights = [1, 3, 1, 3, 5];
    let drawings_dist = WeightedIndex::new(&drawing_weights).unwrap();

    let color_choices = [
        hsl(random_f32(), 0.5, 0.5),
        hsl(random_f32(), 0.5, 0.5),
        hsl(random_f32(), 0.5, 0.5),
    ];
    let color_weights = [3, 2, 1];
    let color_dist = WeightedIndex::new(&color_weights).unwrap();

    let mut rng = thread_rng();

    for rect in r.subdivisions_iter() {
        for rect in rect.subdivisions_iter() {
            for rect in rect.subdivisions_iter() {
                for rect in rect.subdivisions_iter() {
                    let thing = Thing {
                        rect: padding(&rect, 20.0),
                        color: color_choices[color_dist.sample(&mut rng)],
                        drawtype: drawing_choices[drawings_dist.sample(&mut rng)],
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

    let r = padding(&app.window_rect(), 100.0);
    Model {
        _window,
        things: update_model(r),
        background: hsl(random_f32(), 0.5, 0.2)
    }
}

fn update(app: &App, model: &mut Model, _mouse_button: MouseButton) {
    let r = padding(&app.window_rect(), 100.0);
    model.things = update_model(r);
    model.background = hsl(random_f32(), 0.5, 0.2);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(model.background);
    for t in model.things.iter() {
        let r = t.rect;
        match t.drawtype {
            ThingType::BOX => {
                draw.rect()
                    .x_y(r.x(), r.y())
                    .w_h(r.w(), r.h())
                    .no_fill()
                    .stroke(t.color)
                    .stroke_weight(7.0);
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
                    .stroke_weight(7.0);
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
