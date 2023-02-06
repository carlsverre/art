int count = 100;

color COLOR_BG = #F3C59B;
color COLOR_LINE = #8E3B06;

float max_turn_radius = 5 * PI / 180;

// each point is a x/y coordinate (floats)
PVector[] points;

// each direction is a angle in radians
float[] directions;

int MINIMUM_LINE_RADIUS = 100;
int ZONE_DIMENSION = int(640 / 3);

void setup() {
  size(640, 360);

  points = new PVector[count];
  directions = new float[count];

  for (int i = 0; i < points.length; i++) {
    points[i] = new PVector(random(width), random(height));
    directions[i] = random(TWO_PI);
  }
}

void draw() {
  background(COLOR_BG);

  // move all the points
  for (int idx = 0; idx < points.length; idx++) {
    PVector point = points[idx];
    float direction = directions[idx];

    // move point in direction
    point.x += cos(direction);
    point.y += sin(direction);

    // randomly shift the direction +- max turn radius
    directions[idx] += random(max_turn_radius);

    // points reflect off the edge of the screen
    // boolean reflect = point.x > width || point.x < 0 || point.y > height || point.y < 0;
    // if (reflect) {
    //   directions[idx] += PI;
    // }
  }

  // render points and line
  for (int i = 0; i < points.length; i++) {
    PVector point = points[i];

    Target[] targets = new Target[3];

    // draw the line to other points
    for (int j = i + 1; j < points.length; j++) {
      PVector point_j = points[j];
      float dist = point.dist(point_j);

      // add target if distance is within radius and it's smaller than other targets
      if (dist < MINIMUM_LINE_RADIUS) {
        for (int k = 0; k < targets.length; k++) {
          if (targets[k] == null || dist < targets[k].distance) {
            targets[k] = new Target(point_j, dist);
            break;
          }
        }
      }
    }

    for (int foo = 0; foo < targets.length; foo++) {
      if (targets[foo] == null) {
        continue;
      }
      float distance = targets[foo].distance;
      // adjust the stroke alpha by the log scale of the distance over MINIMUM_LINE_RADIUS
      stroke(color(COLOR_LINE, int(255 * (log(MINIMUM_LINE_RADIUS - distance + 1) / log(MINIMUM_LINE_RADIUS + 1)))));

      line(
        int(point.x), int(point.y),
        int(targets[foo].point.x), int(targets[foo].point.y)
      );
    }
  }
}
