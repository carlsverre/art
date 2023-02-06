use nannou::prelude::*;

const MAX_SPEED: f32 = 1.0;
const PERCEPTION_RADIUS: f32 = 100.0;
const STEERING_FORCE: f32 = 0.025;
const RADIUS: f32 = 4.0;

#[derive(PartialEq, Clone)]
pub struct Boid {
    // position is the boid's absolute point in space
    position: Vec2,

    // velocity is a unit vector
    velocity: Vec2,
}

impl Boid {
    pub fn new(position: Vec2, velocity: Vec2) -> Self {
        Boid {
            position,
            velocity: velocity.normalize(),
        }
    }

    pub fn update(&mut self, boundary_rect: Rect, boids: &[Boid], disperse: bool) {
        self.velocity += self.steering(boids, disperse);
        self.velocity = self.velocity.clamp_length_max(MAX_SPEED);

        self.position += self.velocity;

        // wrap around the screen x
        if self.position.x < boundary_rect.left() {
            self.position.x = boundary_rect.right();
        } else if self.position.x > boundary_rect.right() {
            self.position.x = boundary_rect.left();
        }
        // wrap around the screen y
        if self.position.y < boundary_rect.bottom() {
            self.position.y = boundary_rect.top();
        } else if self.position.y > boundary_rect.top() {
            self.position.y = boundary_rect.bottom();
        }
    }

    fn steering(&self, boids: &[Boid], disperse: bool) -> Vec2 {
        let mut align = Vec2::ZERO;
        let mut separate = Vec2::ZERO;
        let mut centroid = Vec2::ZERO;
        let mut flock_size = 0;

        for other in boids.iter() {
            if self == other {
                continue;
            }

            let distance = self.position.distance(other.position);
            if distance > PERCEPTION_RADIUS {
                continue;
            }

            let mut diff = other.position - self.position;
            diff /= (distance * distance).max(0.1);
            separate -= diff;

            align += other.velocity;
            centroid += other.position;
            flock_size += 1;
        }

        if flock_size > 0 {
            if align != Vec2::ZERO {
                align = align / flock_size as f32;
                align = align.clamp_length(MAX_SPEED, MAX_SPEED);
                align -= self.velocity;
                align = align.clamp_length_max(STEERING_FORCE);
                if disperse {
                    align *= -1.;
                }
            }

            if separate != Vec2::ZERO {
                separate = separate / flock_size as f32;
                separate = separate.clamp_length(MAX_SPEED, MAX_SPEED);
                separate -= self.velocity;
                separate = separate.clamp_length_max(STEERING_FORCE);
            }

            if centroid != Vec2::ZERO {
                centroid = centroid / flock_size as f32;
                centroid -= self.position;
                centroid = centroid.clamp_length(MAX_SPEED, MAX_SPEED);
                centroid -= self.velocity;
                centroid = centroid.clamp_length_max(STEERING_FORCE);
                if disperse {
                    centroid *= -1.;
                }
            }
        } else {
            // TODO: Consider having the boids move randomly if they are alone
        }

        align + separate + centroid
    }

    pub fn draw(&self, draw: &Draw) {
        draw.tri()
            .xy(self.position)
            .points(
                Point2::new(RADIUS, 0.0),
                Point2::new(-RADIUS, -RADIUS),
                Point2::new(-RADIUS, RADIUS),
            )
            .w_h(RADIUS * 2., RADIUS * 2.)
            .rotate(self.velocity.angle())
            .hsv(1.0, 1.0, 1.0);
    }
}
