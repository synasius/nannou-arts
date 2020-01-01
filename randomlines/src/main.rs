use nannou::prelude::*;
use nannou::rand;
use nannou::geom::Point2;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {
    lines: Vec<Vec<Point2>>,
    drag: bool,
}

fn model(app: &App) -> Model {
    app.new_window().event(event).view(view).build().unwrap();
    Model {
        lines: Vec::new(),
        drag: false,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for line in model.lines.iter_mut() {
        for p in line.iter_mut() {
            p.x += rand::random_range(-1.0, 1.0);
            p.y += rand::random_range(-1.0, 1.0);
        }
    }
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MousePressed(MouseButton::Right) => {
            model.lines.clear();
        }
        MousePressed(MouseButton::Left) => {
            model.drag = true;
            model.lines.push(Vec::new());
        }
        MouseReleased(MouseButton::Left) => {
            model.drag = false;
        }
        MouseMoved(pos) => {
            if model.drag {
                let line = model.lines.last_mut().unwrap();
                line.push(pos);
            }
        }
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    // FIXME: cannot use polyline until the
    // PR that introduces lyon for 2d drawing
    // is merged into nannou.
    //
    //draw.polyline().vertices(1.0, &model.lines);

    for line in model.lines.iter() {
        if line.len() > 1 {
            for p in line.windows(2) {
                draw.line()
                    .start(p[0])
                    .end(p[1])
                    .thickness(0.5)
                    .color(WHITE);
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
