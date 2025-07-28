use crate::consts::*;
use macroquad::prelude::*;
use std::f32::consts::PI;

#[derive(Clone, Copy)]
pub struct Player {
    pub pos: Vec2,
    pub angle: f32,
}

impl Player {
    pub fn cast(&self, texture: &Texture2D, map: &[[bool; COLUMNS]; ROWS]) {
        let step = FOV / screen_width();
        for point in 0..screen_width() as usize {
            let alpha =
                2.0 * PI - (-FOV / 2.0 + self.angle + step * point as f32).rem_euclid(2.0 * PI);
            let quadrant = (alpha / (PI / 2.0)).ceil() as usize;

            let mut ray_pos_v = self.pos;
            let mut ray_pos_h = self.pos;

            match quadrant {
                1 => {
                    let path_left = vec2(
                        ((self.pos.x / TILE_SIZE).ceil()) * TILE_SIZE - self.pos.x,
                        self.pos.y - ((self.pos.y / TILE_SIZE).floor()) * TILE_SIZE,
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
                        ((self.pos.x / TILE_SIZE).floor()) * TILE_SIZE - self.pos.x,
                        self.pos.y - ((self.pos.y / TILE_SIZE).floor()) * TILE_SIZE,
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
                        ((self.pos.x / TILE_SIZE).floor()) * TILE_SIZE - self.pos.x,
                        ((self.pos.y / TILE_SIZE).ceil()) * TILE_SIZE - self.pos.y,
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
                        ((self.pos.x / TILE_SIZE).ceil()) * TILE_SIZE - self.pos.x,
                        ((self.pos.y / TILE_SIZE).ceil()) * TILE_SIZE - self.pos.y,
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
                _ => panic!("{} {}", quadrant, alpha),
            }

            let hit;
            let horizontal;

            if self.pos.distance(ray_pos_v) < self.pos.distance(ray_pos_h) {
                hit = ray_pos_v;
                horizontal = false;
            } else {
                hit = ray_pos_h;
                horizontal = true;
            };

            let line_h = TILE_SIZE * screen_height() / (self.pos.distance(hit));
            let padding = if horizontal {
                hit.x - ((hit.x / TILE_SIZE).floor()) * TILE_SIZE
            } else {
                hit.y - ((hit.y / TILE_SIZE).floor()) * TILE_SIZE
            };
            draw_texture_ex(
                texture,
                point as f32,
                screen_height() / 2.0 - line_h / 2.0,
                Color::from_vec(
                    LIGHTGRAY.to_vec() * (1.0 - self.pos.distance(hit) / VISIBILITY).min(1.0),
                ),
                DrawTextureParams {
                    dest_size: Some(vec2(1.0, line_h)),
                    source: Some(Rect::new(padding, 0.0, 1.0, TILE_SIZE)),
                    ..Default::default()
                },
            );
            // draw_line(
            //     point as f32,
            //     screen_height() / 2.0 + line_h / 2.0,
            //     point as f32,
            //     screen_height() / 2.0 - line_h / 2.0,
            //     1.0,
            //     Color::from_vec(LIGHTGRAY.to_vec() * (1.0 - self.pos.distance(hit) / VISIBILITY)),
            // );
        }
    }
}
