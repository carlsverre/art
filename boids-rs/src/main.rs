use nannou::prelude::*;

mod boid;
use boid::*;

const NUM_BOIDS: usize = 300;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    boids: Vec<Boid>,
}

fn model(app: &App) -> Model {
    let win = app.window_rect();
    Model {
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

    const SECONDS_PER_CYCLE: u64 = 5;
    let disperse = (update.since_start.as_secs() % SECONDS_PER_CYCLE * 2) > SECONDS_PER_CYCLE;

    // create copy of boids for reference
    let copy = model.boids.clone();

    for boid in &mut model.boids {
        boid.update(win, &copy, disperse);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(PURPLE);

    // draw boids
    for boid in &model.boids {
        boid.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
