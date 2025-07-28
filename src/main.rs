mod consts;
mod player;

use ::rand::{Rng, SeedableRng, rngs::StdRng};
use consts::*;
use macroquad::prelude::*;
use player::*;
use std::f32::consts::PI;

fn window_conf() -> Conf {
    Conf {
        window_title: "raycasting".to_owned(),
        fullscreen: true,
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandWithX11Fallback,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut rng = StdRng::from_os_rng();

    let mut map = [[true; COLUMNS]; ROWS];
    for row in &mut map {
        for tile in row {
            *tile = rng.random_range(0.0..=1.0) <= TILE_CHANCE;
        }
    }

    let mut player = Player {
        pos: vec2(50.0, 50.0),
        angle: 0.0,
    };

    let mut last_mouse_position: Vec2 = mouse_position().into();
    let mut yaw = 0.0;

    set_cursor_grab(true);
    show_mouse(false);

    let texture = Texture2D::from_file_with_format(include_bytes!("../image.png"), None);

    loop {
        let mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_position - last_mouse_position;

        last_mouse_position = mouse_position;

        yaw += mouse_delta.x * MOUSE_SENSITIVITY;
        player.angle = yaw.rem_euclid(2.0 * PI);

        let mut delta = Vec2::ZERO;
        let d = vec2(player.angle.cos(), player.angle.sin());

        if is_key_down(KeyCode::A) {
            delta -= d.perp();
        }
        if is_key_down(KeyCode::D) {
            delta += d.perp();
        }

        if is_key_down(KeyCode::W) {
            delta += d;
        }
        if is_key_down(KeyCode::S) {
            delta -= d;
        }

        delta = delta.normalize_or_zero() * PLAYER_STEP;

        let new_pos = player.pos + delta;
        if FIELD.contains(new_pos.with_x(player.pos.x))
            && !map[(new_pos.y / TILE_SIZE) as usize][(player.pos.x / TILE_SIZE) as usize]
        {
            player.pos.y = new_pos.y;
        }

        if FIELD.contains(new_pos.with_y(player.pos.y))
            && !map[(player.pos.y / TILE_SIZE) as usize][(new_pos.x / TILE_SIZE) as usize]
        {
            player.pos.x = new_pos.x;
        }

        player.cast(&texture, &map);

        // draw_line(
        //     player.pos.x,
        //     player.pos.y,
        //     player.pos.x + player.angle.cos() * 50.0,
        //     player.pos.y + player.angle.sin() * 50.0,
        //     5.0,
        //     YELLOW,
        // );

        // for row in 0..ROWS {
        //     for column in 0..COLUMNS {
        //         if map[row][column] {
        //             draw_rectangle_lines(
        //                 column as f32 * TILE_SIZE,
        //                 row as f32 * TILE_SIZE,
        //                 TILE_SIZE,
        //                 TILE_SIZE,
        //                 2.0,
        //                 WHITE,
        //             );
        //         }
        //     }
        // }
        //
        // draw_rectangle_lines(FIELD.x, FIELD.y, FIELD.w, FIELD.h, 5.0, WHITE);

        next_frame().await;
    }
}
