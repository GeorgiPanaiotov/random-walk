use nannou::prelude::*;
use random_walk::{Direction, RandomWalk};

mod random_walk;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn main() {
    nannou::app(model)
        .size(WIDTH, HEIGHT)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    walker: RandomWalk,
    directions: [Direction; 4],
}

fn model(app: &App) -> Model {
    let walker = RandomWalk::new();
    let directions = [
        Direction::UP,
        Direction::DOWN,
        Direction::LEFT,
        Direction::RIGHT,
    ];

    app.draw().background().color(BLACK);
    Model { walker, directions }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let rnd = random_range(0, 4);
    model.walker.update(model.directions[rnd]);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    model.walker.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}
