use ::rand::{Rng, SeedableRng, rngs::StdRng};
use macroquad::prelude::*;
use std::f32::consts::PI;

const ROWS: usize = 15;
const COLUMNS: usize = 20;
const TILE_SIZE: f32 = 64.0;

const TILE_CHANCE: f32 = 0.1;
const ANGLE_STEP: f32 = 0.05;
const PLAYER_RADIUS: f32 = 5.0;
const PLAYER_STEP: f32 = 5.0;
static FIELD: Rect = Rect::new(
    0.0,
    0.0,
    COLUMNS as f32 * TILE_SIZE,
    ROWS as f32 * TILE_SIZE,
);

const FOV: f32 = PI / 2.0;

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

struct Player {
    pos: Vec2,
    angle: f32,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut rng = StdRng::from_os_rng();

    let mut map = [[false; COLUMNS]; ROWS];
    for row in &mut map {
        for tile in row {
            *tile = rng.random_range(0.0..=1.0) <= TILE_CHANCE;
        }
    }

    for _ in 0..8 {
        set_fullscreen(true);
        next_frame().await;
    }

    let mut player = Player {
        pos: vec2(FIELD.x, FIELD.y),
        angle: 0.0,
    };

    loop {
        let mut delta = Vec2::ZERO;
        if is_key_down(KeyCode::A) {
            delta.x = -PLAYER_STEP;
        } else if is_key_down(KeyCode::D) {
            delta.x = PLAYER_STEP;
        }

        if is_key_down(KeyCode::W) {
            delta.y = -PLAYER_STEP;
        } else if is_key_down(KeyCode::S) {
            delta.y = PLAYER_STEP;
        }

        player.pos.x = (player.pos.x + delta.x).clamp(FIELD.x, FIELD.w);
        player.pos.y = (player.pos.y + delta.y).clamp(FIELD.y, FIELD.h);

        let right_pressed = is_key_down(KeyCode::Right);
        let left_pressed = is_key_down(KeyCode::Left);
        let signum = if right_pressed {
            1.0
        } else if left_pressed {
            -1.0
        } else {
            0.0
        };
        player.angle = (player.angle + ANGLE_STEP * signum).rem_euclid(2.0 * PI);

        // draw_circle(
        //     player.pos.x as f32,
        //     player.pos.y as f32,
        //     PLAYER_RADIUS as f32,
        //     WHITE,
        // );

        let step = FOV / screen_width();
        for point in 0..screen_width() as usize {
            let alpha =
                2.0 * PI - (-FOV / 2.0 + player.angle + step * point as f32).rem_euclid(2.0 * PI);
            let quadrant = match alpha {
                angle if (0.0..PI / 2.0).contains(&angle) => 1,
                angle if (PI / 2.0..PI).contains(&angle) => 2,
                angle if (PI..3.0 / 2.0 * PI).contains(&angle) => 3,
                angle if (3.0 / 2.0 * PI..=2.0 * PI).contains(&angle) => 4,
                _ => continue,
            };

            let mut ray_pos_v = player.pos;
            let mut ray_pos_h = player.pos;

            match quadrant {
                1 => {
                    let path_left = vec2(
                        ((player.pos.x / TILE_SIZE).ceil()) as f32 * TILE_SIZE - player.pos.x,
                        player.pos.y - ((player.pos.y / TILE_SIZE).floor()) as f32 * TILE_SIZE,
                    );

                    let b = -alpha.tan() * path_left.x;
                    ray_pos_v += vec2(path_left.x + 1.0, b);

                    loop {
                        if !FIELD.contains(ray_pos_v)
                            || map[(ray_pos_v.y / TILE_SIZE).floor() as usize]
                                [(ray_pos_v.x / TILE_SIZE).floor() as usize]
                        {
                            break;
                        }

                        ray_pos_v += vec2(TILE_SIZE, TILE_SIZE * -alpha.tan());
                    }

                    let b = path_left.y / alpha.tan();
                    ray_pos_h += vec2(b, -path_left.y - 1.0);

                    loop {
                        let pos_beyond_field = !FIELD.contains(ray_pos_h);
                        if pos_beyond_field
                            || map[((ray_pos_h.y) / TILE_SIZE).floor() as usize]
                                [(ray_pos_h.x / TILE_SIZE).floor() as usize]
                        {
                            if pos_beyond_field {
                                let extra_y = FIELD.y - ray_pos_h.y;
                                let extra_x = extra_y / -alpha.tan();
                                ray_pos_h += vec2(extra_x, extra_y);
                            }

                            break;
                        }

                        ray_pos_h += vec2(TILE_SIZE / alpha.tan(), -TILE_SIZE);
                    }
                }
                2 => {
                    let path_left = vec2(
                        ((player.pos.x / TILE_SIZE).floor()) as f32 * TILE_SIZE - player.pos.x,
                        player.pos.y - ((player.pos.y / TILE_SIZE).floor()) as f32 * TILE_SIZE,
                    );

                    let b = -alpha.tan() * path_left.x;
                    ray_pos_v += vec2(path_left.x - 1.0, b);

                    loop {
                        let pos_beyond_field = !FIELD.contains(ray_pos_v);
                        if pos_beyond_field
                            || map[(ray_pos_v.y / TILE_SIZE).floor() as usize]
                                [(ray_pos_v.x / TILE_SIZE).floor() as usize]
                        {
                            if pos_beyond_field {
                                let extra_x = FIELD.x - ray_pos_v.x;
                                let extra_y = extra_x * -alpha.tan();

                                ray_pos_v += vec2(extra_x, extra_y);
                            }
                            break;
                        }

                        ray_pos_v += vec2(-TILE_SIZE, TILE_SIZE * alpha.tan());
                    }

                    let b = path_left.y / alpha.tan();
                    ray_pos_h += vec2(b, -path_left.y - 1.0);

                    loop {
                        let pos_beyond_field = !FIELD.contains(ray_pos_h);
                        if pos_beyond_field
                            || map[((ray_pos_h.y) / TILE_SIZE).floor() as usize]
                                [(ray_pos_h.x / TILE_SIZE).floor() as usize]
                        {
                            if pos_beyond_field {
                                let extra_y = FIELD.y - ray_pos_h.y;
                                let extra_x = extra_y / -alpha.tan();
                                ray_pos_h += vec2(extra_x, extra_y);
                            }
                            break;
                        }

                        ray_pos_h += vec2(TILE_SIZE / alpha.tan(), -TILE_SIZE);
                    }
                }
                3 => {
                    let alpha = alpha - PI;
                    let path_left = vec2(
                        ((player.pos.x / TILE_SIZE).floor()) as f32 * TILE_SIZE - player.pos.x,
                        ((player.pos.y / TILE_SIZE).ceil()) as f32 * TILE_SIZE - player.pos.y,
                    );

                    let b = -alpha.tan() * path_left.x;
                    ray_pos_v += vec2(path_left.x - 1.0, b);

                    loop {
                        let pos_beyond_field = !FIELD.contains(ray_pos_v);
                        if pos_beyond_field
                            || map[(ray_pos_v.y / TILE_SIZE).floor() as usize]
                                [(ray_pos_v.x / TILE_SIZE).floor() as usize]
                        {
                            if pos_beyond_field {
                                let extra_x = FIELD.x - ray_pos_v.x;
                                let extra_y = extra_x * -alpha.tan();
                                ray_pos_v += vec2(extra_x, extra_y);
                            }

                            break;
                        }

                        ray_pos_v += vec2(-TILE_SIZE, TILE_SIZE * alpha.tan());
                    }

                    let b = path_left.y / -alpha.tan();
                    ray_pos_h += vec2(b, path_left.y);

                    loop {
                        let pos_beyond_field = !FIELD.contains(ray_pos_h);
                        if pos_beyond_field
                            || map[((ray_pos_h.y) / TILE_SIZE).floor() as usize]
                                [(ray_pos_h.x / TILE_SIZE).floor() as usize]
                        {
                            if pos_beyond_field {
                                let extra_y = ray_pos_h.y - FIELD.h;
                                let extra_x = extra_y / alpha.tan();
                                ray_pos_h += vec2(extra_x, -extra_y);
                            }
                            break;
                        }

                        ray_pos_h += vec2(TILE_SIZE / -alpha.tan(), TILE_SIZE);
                    }
                }
                4 => {
                    let path_left = vec2(
                        ((player.pos.x / TILE_SIZE).ceil()) as f32 * TILE_SIZE - player.pos.x,
                        ((player.pos.y / TILE_SIZE).ceil()) as f32 * TILE_SIZE - player.pos.y,
                    );

                    let b = -alpha.tan() * path_left.x;
                    ray_pos_v += vec2(path_left.x, b);

                    loop {
                        let pos_beyond_field = !FIELD.contains(ray_pos_v);
                        if pos_beyond_field
                            || map[(ray_pos_v.y / TILE_SIZE).floor() as usize]
                                [(ray_pos_v.x / TILE_SIZE).floor() as usize]
                        {
                            break;
                        }

                        ray_pos_v += vec2(TILE_SIZE, TILE_SIZE * -alpha.tan());
                    }

                    let b = path_left.y / -alpha.tan();
                    ray_pos_h += vec2(b, path_left.y);

                    loop {
                        if !FIELD.contains(ray_pos_h)
                            || map[((ray_pos_h.y) / TILE_SIZE).floor() as usize]
                                [(ray_pos_h.x / TILE_SIZE).floor() as usize]
                        {
                            break;
                        }

                        ray_pos_h += vec2(TILE_SIZE / -alpha.tan(), TILE_SIZE);
                    }
                }
                _ => unreachable!(),
            }

            let hit;
            let horizontal;
            if player.pos.distance(ray_pos_v) < player.pos.distance(ray_pos_h) {
                hit = ray_pos_v;
                horizontal = false;
            } else {
                hit = ray_pos_h;
                horizontal = true;
            };
            // draw_line(
            //     player.pos.x as f32,
            //     player.pos.y as f32,
            //     hit.x as f32,
            //     hit.y as f32,
            //     2.0,
            //     PURPLE,
            // );

            let line_h = TILE_SIZE * screen_height() / (player.pos.distance(hit));
            draw_line(
                point as f32,
                screen_height() / 2.0 + line_h / 2.0,
                point as f32,
                screen_height() / 2.0 - line_h / 2.0,
                1.0,
                if horizontal { DARKGREEN } else { GREEN },
            );
        }

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
