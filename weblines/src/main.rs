use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Line(usize, usize);

struct Model {
    lines: Vec<Line>,
    points: Vec<Vector2>,
    drag: bool,
    mover: Mover,
}

struct Mover {
    velocity: Vector2,
    acceleration: Vector2,
    top_speed: f32,
}

impl Mover {
    fn new() -> Self {
        let velocity = vec2(0.0, 0.0);
        let acceleration = vec2(0.0, 0.0);
        let top_speed = 0.5;
        Mover {
            velocity,
            acceleration,
            top_speed,
        }
    }

    fn update(&mut self, mouse: Point2, position: &mut Point2) {
        // Computer a vector that points from position to mouse
        self.acceleration = mouse - *position;
        // Set magnitude of acceleration
        self.acceleration = self.acceleration.normalize() * 0.02;
        // Velocity chages according to acceleration
        self.velocity += self.acceleration;
        // Limit the velocity by top_speed
        self.velocity = self.velocity.limit_magnitude(self.top_speed);
        // Update position
        position.x += self.velocity.x;
        position.y += self.velocity.y;
    }
}

fn model(app: &App) -> Model {
    app.new_window().event(event).view(view).build().unwrap();
    let mover = Mover::new();
    Model {
        lines: Vec::new(),
        points: Vec::new(),
        drag: false,
        mover,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for point in model.points.iter_mut() {
        model.mover.update(pt2(app.mouse.x, app.mouse.y), point);
    }
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MousePressed(MouseButton::Right) => {
            model.lines.clear();
            model.points.clear();
        }
        MousePressed(MouseButton::Left) => {
            model.drag = true;
        }
        MouseReleased(MouseButton::Left) => {
            model.drag = false;
        }
        MouseMoved(pos) => {
            if model.drag {
                for (i, point) in model.points.iter().enumerate() {
                    let distance = point.distance2(pos);
                    let next_position = model.points.len();
                    if distance < 500.0 {
                        model.lines.push(Line(next_position, i));
                    }
                }
                model.points.push(pos);
            }
        }
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    for line in model.lines.iter() {
        draw.line().start(model.points[line.0])
            .end(model.points[line.1])
            .color(rgba(0.3, 0.3, 0.3, 0.3));
    }

    draw.to_frame(app, &frame).unwrap();
}
