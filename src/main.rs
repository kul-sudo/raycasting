use ::rand::{Rng, SeedableRng, rngs::StdRng, seq::IndexedRandom};
use macroquad::prelude::*;
use parry2d::{
    math::{Isometry, Point, Vector},
    query::{Ray, RayCast, contact},
    shape::{Ball, Compound, Cuboid, SharedShape},
};
use std::f32::consts::PI;

const ROWS: usize = 15;
const COLUMNS: usize = 15;
const TILE_SIZE: f32 = 64.0;

const TILE_CHANCE: f32 = 0.1;
const ANGLE_STEP: f32 = 0.05;
const PLAYER_RADIUS: f32 = 5.0;
const PLAYER_STEP: f32 = 5.0;
const HEIGHT: f32 = 15.0;

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
    for row in 0..ROWS {
        for column in 0..COLUMNS {
            map[row][column] = rng.random_range(0.0..=1.0) <= TILE_CHANCE;
        }
    }

    for _ in 0..8 {
        set_fullscreen(true);
        next_frame().await;
    }

    let mut player = Player {
        pos: Vec2::splat(0.0),
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

        player.pos.x = (player.pos.x + delta.x).clamp(0.0, COLUMNS as f32 * TILE_SIZE);
        player.pos.y = (player.pos.y + delta.y).clamp(0.0, ROWS as f32 * TILE_SIZE);

        let right_pressed = is_key_down(KeyCode::Right);
        let left_pressed = is_key_down(KeyCode::Left);
        let signum = if right_pressed {
            -1.0
        } else if left_pressed {
            1.0
        } else {
            0.0
        };
        player.angle = (player.angle + ANGLE_STEP * signum).rem_euclid(2.0 * PI);

        draw_circle(
            player.pos.x as f32,
            player.pos.y as f32,
            PLAYER_RADIUS as f32,
            WHITE,
        );

        let step = FOV / screen_width();
        for point in 0..screen_width() as usize {
            let step_rad = (player.angle + step * point as f32).rem_euclid(2.0 * PI);
            let quadrant = match step_rad {
                angle if angle >= 0.0 && angle < PI / 2.0 => 1,
                angle if angle > PI / 2.0 && angle < PI => 2,
                angle if angle > PI && angle < 3.0 / 2.0 * PI => 3,
                angle if angle > 3.0 / 2.0 * PI && angle < 2.0 * PI => 4,
                _ => unreachable!(),
            };

            let mut ray_pos_v = player.pos;
            let mut ray_pos_h = player.pos;

            match quadrant {
                1 => {
                    let path_left = vec2(
                        ((player.pos.x / TILE_SIZE).ceil()) as f32 * TILE_SIZE - player.pos.x,
                        player.pos.y - ((player.pos.y / TILE_SIZE).floor()) as f32 * TILE_SIZE,
                    );

                    let b = -step_rad.tan() * path_left.x;
                    ray_pos_v += vec2(path_left.x + 1.0, b);

                    loop {
                        if !Rect::new(
                            0.0,
                            0.0,
                            COLUMNS as f32 * TILE_SIZE,
                            ROWS as f32 * TILE_SIZE,
                        )
                        .contains(ray_pos_v)
                            || map[(ray_pos_v.y / TILE_SIZE).floor() as usize]
                                [(ray_pos_v.x / TILE_SIZE).floor() as usize]
                        {
                            break;
                        }

                        ray_pos_v += vec2(TILE_SIZE, TILE_SIZE * -step_rad.tan());
                    }

                    let b = path_left.y / step_rad.tan();
                    ray_pos_h += vec2(b, -path_left.y - 1.0);

                    loop {
                        if !Rect::new(
                            0.0,
                            0.0,
                            COLUMNS as f32 * TILE_SIZE,
                            ROWS as f32 * TILE_SIZE,
                        )
                        .contains(ray_pos_h)
                            || map[((ray_pos_h.y) / TILE_SIZE).floor() as usize]
                                [(ray_pos_h.x / TILE_SIZE).floor() as usize]
                        {
                            break;
                        }

                        ray_pos_h += vec2(TILE_SIZE / step_rad.tan(), -TILE_SIZE);
                    }
                }
                2 => {
                    let path_left = vec2(
                        ((player.pos.x / TILE_SIZE).floor()) as f32 * TILE_SIZE - player.pos.x,
                        player.pos.y - ((player.pos.y / TILE_SIZE).floor()) as f32 * TILE_SIZE,
                    );

                    let b = -step_rad.tan() * path_left.x;
                    ray_pos_v += vec2(path_left.x - 1.0, b);

                    loop {
                        if !Rect::new(
                            0.0,
                            0.0,
                            COLUMNS as f32 * TILE_SIZE,
                            ROWS as f32 * TILE_SIZE,
                        )
                        .contains(ray_pos_v)
                            || map[(ray_pos_v.y / TILE_SIZE).floor() as usize]
                                [(ray_pos_v.x / TILE_SIZE).floor() as usize]
                        {
                            break;
                        }

                        ray_pos_v += vec2(-TILE_SIZE, TILE_SIZE * step_rad.tan());
                    }

                    let b = path_left.y / step_rad.tan();
                    ray_pos_h += vec2(b, -path_left.y - 1.0);

                    loop {
                        if !Rect::new(
                            0.0,
                            0.0,
                            COLUMNS as f32 * TILE_SIZE,
                            ROWS as f32 * TILE_SIZE,
                        )
                        .contains(ray_pos_h)
                            || map[((ray_pos_h.y) / TILE_SIZE).floor() as usize]
                                [(ray_pos_h.x / TILE_SIZE).floor() as usize]
                        {
                            break;
                        }

                        ray_pos_h += vec2(TILE_SIZE / step_rad.tan(), -TILE_SIZE);
                    }
                }
                3 => {
                    let step_rad = step_rad - PI;
                    let path_left = vec2(
                        ((player.pos.x / TILE_SIZE).floor()) as f32 * TILE_SIZE - player.pos.x,
                        player.pos.y - ((player.pos.y / TILE_SIZE).ceil()) as f32 * TILE_SIZE,
                    );

                    let b = -step_rad.tan() * path_left.x;
                    ray_pos_v += vec2(path_left.x - 1.0, b);
                    
                    loop {
                        if !Rect::new(
                            0.0,
                            0.0,
                            COLUMNS as f32 * TILE_SIZE,
                            ROWS as f32 * TILE_SIZE,
                        )
                        .contains(ray_pos_v)
                            || map[(ray_pos_v.y / TILE_SIZE).floor() as usize]
                                [(ray_pos_v.x / TILE_SIZE).floor() as usize]
                        {
                            break;
                        }

                        ray_pos_v += vec2(-TILE_SIZE, TILE_SIZE * step_rad.tan());
                    }
                    //
                    let b = path_left.y / step_rad.tan();
                    ray_pos_h += vec2(b, -path_left.y - 1.0);
                    ray_pos_h += Vec2::INFINITY;
                    //
                    // loop {
                    //     break;
                    //     if !Rect::new(
                    //         0.0,
                    //         0.0,
                    //         COLUMNS as f32 * TILE_SIZE,
                    //         ROWS as f32 * TILE_SIZE,
                    //     )
                    //     .contains(ray_pos_h)
                    //         || map[((ray_pos_h.y) / TILE_SIZE).floor() as usize]
                    //             [(ray_pos_h.x / TILE_SIZE).floor() as usize]
                    //     {
                    //         break;
                    //     }
                    //
                    //     ray_pos_h += vec2(TILE_SIZE / step_rad.tan(), -TILE_SIZE);
                    // }
                }
                _ => continue,
            }
            //
            // let hit = if c_h.length() < c_v.length() {
            //     c_v
            // } else {
            //     c_v
            // };
            //
            //

            let hit = if player.pos.distance(ray_pos_v) < player.pos.distance(ray_pos_h) {
                ray_pos_v
            } else {
                ray_pos_h
            };

            draw_line(
                player.pos.x as f32,
                player.pos.y as f32,
                hit.x as f32,
                hit.y as f32,
                2.0,
                PURPLE,
            );

            // let r = Vec2::new(step_rad.cos(), step_rad.sin());
            // let ray = Ray::new(
            //     Point::new(player.pos.x, player.pos.y),
            //     Vector::new(r.x, r.y),
            // );
            //
            // let mut end_pos = Vec2::new(player.pos.x, player.pos.y);
            // let time_tile = compound.cast_ray(&Isometry::identity(), &ray, f32::MAX, true);
            // let time_wall = frame.cast_ray(
            //     &Isometry::translation(frame.half_extents.x, frame.half_extents.y),
            //     &ray,
            //     f32::MAX,
            //     false,
            // );
            //
            // let time = match time_tile {
            //     Some(time_tile) => Some(match time_wall {
            //         Some(time_wall) => time_tile.min(time_wall),
            //         None => time_tile,
            //     }),
            //     None => time_wall,
            // };
            //
            // if let Some(t) = time {
            //     end_pos += r * t;
            //
            //     let row = (end_pos.y / TILE_SIZE) as usize;
            //     let column = (end_pos.x / TILE_SIZE) as usize;
            //
            //     let floor_y = end_pos.y - row as f32 * TILE_SIZE;
            //
            //     // draw_line(
            //     //     player.pos.x as f32,
            //     //     player.pos.y as f32,
            //     //     end_pos.x as f32,
            //     //     end_pos.y as f32,
            //     //     2.0,
            //     //     PURPLE,
            //     // );
            //
            //     let line_h = TILE_SIZE * screen_height() / player.pos.distance(end_pos);
            //
            //     draw_line(
            //         point as f32,
            //         screen_height() / 2.0 + line_h / 2.0,
            //         point as f32,
            //         screen_height() / 2.0 - line_h / 2.0,
            //         1.0,
            //         if floor_y > 0.0 && floor_y < TILE_SIZE {
            //             DARKGREEN
            //         } else {
            //             GREEN
            //         },
            //     );
            // }
        }

        for row in 0..ROWS {
            for column in 0..COLUMNS {
                if map[row][column] {
                    draw_rectangle_lines(
                        column as f32 * TILE_SIZE,
                        row as f32 * TILE_SIZE,
                        TILE_SIZE,
                        TILE_SIZE,
                        2.0,
                        WHITE,
                    );
                }
            }
        }

        draw_rectangle_lines(
            0.0,
            0.0,
            COLUMNS as f32 * TILE_SIZE,
            ROWS as f32 * TILE_SIZE,
            5.0,
            WHITE,
        );

        next_frame().await;
    }
}
