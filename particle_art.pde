int count = 1000;

float max_turn_radius = 20 * PI / 180;

// each point is a x/y coordinate (floats)
PVector[] points;

// each direction is a angle in radians
PVector[] directions;

int MINIMUM_LINE_RADIUS = 100;
int ZONE_DIMENSION = int(640 / 3);

void setup() {
  fullScreen(P2D);
  noSmooth();

  colorMode(HSB, 360, 100, 100, 100);

  points = new PVector[count];
  directions = new PVector[count];

  for (int i = 0; i < points.length; i++) {
    points[i] = new PVector(random(width), random(height));
    directions[i] = new PVector(random(2)-1, random(2)-1);
  }
}

int MAX_STROKE_WEIGHT = 10;

boolean ToroidalDivorce (PVector v1, PVector v2)
{
    float dx = abs(v2.x - v1.x);
    float dy = abs(v2.y - v1.y);
    return (dx > 0.5*width) || (dy > 0.5*height); 
}

float ToroidalDistance (PVector v1, PVector v2)
{
    float dx = abs(v2.x - v1.x);
    float dy = abs(v2.y - v1.y);
 
    if (dx > 0.5*width) {
        dx = width - dx;
    }
 
    if (dy > 0.5*height) {
        dy = height - dy;
    }
 
    return sqrt(dx*dx + dy*dy);
}

void draw() {
  background(color(270, 50, 50));

  // move all the points
  for (int idx = 0; idx < points.length; idx++) {
    PVector point = points[idx];
    PVector direction = directions[idx];

    // move point in direction
    point.x += direction.x;
    point.y += direction.y;

    // randomly rotate the direction +- max turn radius
    // direction.rotate(random(max_turn_radius));

    point.x = (point.x + width) % width;
    point.y = (point.y + height) % height;

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
      float dist = ToroidalDistance(point, point_j);

      // add target if distance is within radius and it's smaller than other targets
      if (dist < MINIMUM_LINE_RADIUS) {
        for (int k = 0; k < targets.length; k++) {
          if (targets[k] == null || dist > targets[k].distance) {
            targets[k] = new Target(point_j, directions[j], dist);
            break;
          }
        }
      }
    }

    boolean hasTarget = true;
    for (int foo = 0; foo < targets.length; foo++) {
      if (targets[foo] == null) {
        hasTarget = false;
        continue;
      }

      float distance = targets[foo].distance;
      float contribution = 1- distance / MINIMUM_LINE_RADIUS;
      float angle = -1*PVector.angleBetween(directions[i], targets[foo].direction) * contribution;
      pushMatrix();
      translate(point.x, point.y);
      directions[i].rotate(min(angle, max_turn_radius));
      popMatrix();

      // adjust the stroke alpha by the log scale of the distance over MINIMUM_LINE_RADIUS
      int opacity = int(
        50 * (log(MINIMUM_LINE_RADIUS - distance + 1) / log(MINIMUM_LINE_RADIUS + 1))
      );

      float distanceModifier = distance / MINIMUM_LINE_RADIUS;
      int hue = (int(180 * distanceModifier) + 50) % 360;
      stroke(color(hue, 50, 30, opacity));
      strokeWeight(MAX_STROKE_WEIGHT - (MAX_STROKE_WEIGHT * distanceModifier));

      if (!ToroidalDivorce(point, targets[foo].point)) {
        line(
          int(point.x), int(point.y),
          int(targets[foo].point.x), int(targets[foo].point.y)
        );
      }
    }

    // draw the point
      stroke(color(0, 0, 100, 100));
      point(point.x, point.y);
  }
}
