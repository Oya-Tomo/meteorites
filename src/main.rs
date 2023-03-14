mod model;

use model::Meteorite;
use model::GRAVITY;
use nannou::prelude::*;

static mut meteos: Vec<Meteorite> = vec![];

fn main() {
    nannou::app(state).run();
}

struct State {}

fn state(app: &App) -> State {
    app.new_window().size(600, 600).view(view).build().unwrap();
    unsafe {
        for i in 0..100 {
            meteos.push(Meteorite::random((600, 600)));
        }
    }
    State {}
}

fn view(app: &App, model: &State, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    // check
    unsafe {
        for i in 0..meteos.len() {
            for l in i + 1..meteos.len() {
                let res = calc_grav(&meteos[i], &meteos[l]);
                meteos[i].velocity.x += res.0 .0;
                meteos[i].velocity.y += res.0 .1;
                meteos[l].velocity.x += res.1 .0;
                meteos[l].velocity.y += res.1 .1;
            }
        }
        for i in 0..meteos.len() {
            meteos[i].update();
            draw.ellipse()
                .color(WHITE)
                .w(meteos[i].size)
                .h(meteos[i].size)
                .x_y(meteos[i].position.x, meteos[i].position.y);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn calc_grav(targ1: &Meteorite, targ2: &Meteorite) -> ((f32, f32), (f32, f32)) {
    let diss = ((targ1.position.x - targ2.position.x).pow(2.0)
        + (targ1.position.y - targ2.position.y).pow(2.0))
    .sqrt();

    if diss > 1000.0 {
        return ((0.0, 0.0), (0.0, 0.0));
    }

    let sin1 = (targ2.position.y - targ1.position.y) / diss;
    let cos1 = (targ2.position.x - targ1.position.x) / diss;
    let sin2 = (targ1.position.y - targ2.position.y) / diss;
    let cos2 = (targ1.position.x - targ2.position.x) / diss;

    let ratio = 1000.0 - diss;

    let f1 = ratio * GRAVITY * targ2.size / 100000.0;
    let f2 = ratio * GRAVITY * targ1.size / 100000.0;

    return ((f1 * cos1, f1 * sin1), (f2 * cos2, f2 * sin2));
}
