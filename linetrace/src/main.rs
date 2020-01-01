use nannou::prelude::*;
use ndarray::{Array, Axis, stack, arr2};

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {

    let a = Array::linspace(0., 1., 5).into_shape((5,1)).unwrap();
    let b = Array::<f64, _>::ones((5, 1));

    let res = stack(Axis(1), &[b.view(), a.view()]);
    dbg!(a);
    dbg!(b);
    dbg!(&res);

    assert!(res == Ok(arr2(&[[1., 0.],
                 [1., 0.25],
                 [1., 0.5],
                 [1., 0.75],
                 [1., 1.]]))
    );
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, _model: &Model, frame: Frame) -> Frame {
    let draw = app.draw();

    draw.background().color(WHITE);

    for _ in 0..20 {
        draw.line()
            .start(Vector2::new(0.0, 1.0))
            .end(Vector2::new(10.0, 10.0))
            .color(BLACK);
    }

    draw.to_frame(app, &frame).expect("Errors");

    frame
}
