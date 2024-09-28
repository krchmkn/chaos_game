use piston_window::*;
use rand::Rng;

struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Chaos Game", [800, 800])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let vertices = vec![
        Point { x: 400.0, y: 50.0 },
        Point { x: 50.0, y: 750.0 },
        Point { x: 750.0, y: 750.0 },
    ];

    let mut rng = rand::thread_rng();
    let mut current_point = Point { x: 400.0, y: 400.0 };

    while let Some(event) = window.next() {
        if let Some(_) = event.render_args() {
            window.draw_2d(&event, |c, g, _| {
                clear([1.0, 1.0, 1.0, 1.0], g);
                for _ in 0..10000 {
                    let vertex_index = rng.gen_range(0..3);
                    current_point.x = (current_point.x + vertices[vertex_index].x) / 2.0;
                    current_point.y = (current_point.y + vertices[vertex_index].y) / 2.0;

                    rectangle(
                        [0.0, 0.0, 0.0, 1.0],
                        [current_point.x, current_point.y, 1.0, 1.0],
                        c.transform,
                        g,
                    );
                }
            });
        }
    }
}
