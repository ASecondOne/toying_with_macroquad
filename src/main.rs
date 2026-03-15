
use macroquad::prelude::*;

struct Pos {
    x: f32,
    y: f32,
}

impl Pos {
    fn new(x: f32, y: f32) -> Pos {
        Pos { x, y }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Demo".to_string(),
        window_width: 1000,
        window_height: 1000,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let origin = Pos::new(500.0, 500.0);
    let mut goal = Pos::new(origin.x, screen_height() - 500.0);

    loop {
        clear_background(BLACK);

        if goal.x < screen_width() {
            goal.x = goal.x + 10.0;
        }

        draw_line(origin.x, origin.y, goal.x, goal.y, 1.0, RED);

        draw_circle(origin.x, origin.y, 2.0, WHITE);

        std::thread::sleep(std::time::Duration::from_millis(20));

        next_frame().await;
    }
}
