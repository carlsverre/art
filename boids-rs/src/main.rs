use nannou::prelude::*;

mod boid;
use boid::*;

const NUM_BOIDS: usize = 2000;
const MAX_LINE_DIST: f32 = 50.0;
const SECONDS_PER_CYCLE: u64 = 5;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    disperse: bool,
    boids: Vec<Boid>,
}

fn model(app: &App) -> Model {
    let win = app.window_rect();
    Model {
        disperse: true,
        boids: (0..NUM_BOIDS)
            .map(|_| {
                let position = vec2(
                    random_range(win.left(), win.right()),
                    random_range(win.top(), win.bottom()),
                );
                let velocity = vec2(random_range(-1.0, 1.0), random_range(-1.0, 1.0));
                Boid::new(position, velocity)
            })
            .collect(),
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let win = app.window_rect();

    if update.since_start.as_secs() % SECONDS_PER_CYCLE == 0 {
        if model.disperse || random_f32() < 0.3 {
            model.disperse = !model.disperse;
        }
    }

    // create copy of boids for reference
    let copy = model.boids.clone();

    for boid in &mut model.boids {
        boid.update(win, &copy, model.disperse);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(hsv(270. / 360., 0.5, 0.1));

    // draw lines
    for (i, boid_a) in model.boids.iter().enumerate() {
        for boid_b in &model.boids[i..] {
            if boid_a == boid_b {
                continue;
            }

            let distance = boid_a.position.distance(boid_b.position);
            if distance > MAX_LINE_DIST || distance < MAX_LINE_DIST / 4.0 {
                continue;
            }

            let dist_frac = distance / MAX_LINE_DIST;
            let dist_frac_invert = 1.0 - dist_frac;

            let pos_a = boid_a.position.normalize();
            let hue = pos_a.y.atan2(pos_a.x) / (2.0 * PI);

            draw.line()
                .start(boid_a.position)
                .end(boid_b.position)
                .weight(3.0 * dist_frac_invert)
                .color(hsva(hue, 0.5, 0.3, dist_frac_invert));
        }

        draw.ellipse()
            .xy(boid_a.position)
            .radius(1.0)
            .hsva(0., 0., 1.0, 0.8);
    }

    draw.to_frame(app, &frame).unwrap();
}
