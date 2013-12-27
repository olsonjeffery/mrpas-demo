// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.
//
// Ported from the libtcod implementation of
// Mingo's Restrictive Precise Angle Shadowcasting (MRPAS)
// http://roguebasin.roguelikedevelopment.org/index.php?title=Restrictive_Precise_Angle_Shadowcasting

use map::{Map, Tile};

fn compute_quad(map: &mut Map, position: (uint, uint),
                max_radius: uint, start_angle: &mut [f64], end_angle: &mut [f64],
               dx: int, dy: int) {
    let (position_x, position_y) = position;
    // octant: vertical edge
    {
        let mut iteration = 1 as int;
        let mut done = false;
        let mut total_obstacles = 0;
        let mut obstacles_in_last_line = 0;
        let mut min_angle = 0.0;
        let mut x = 0 as int;
        let mut y = (position_y as int + dy) as int;
        let mut c = 0;
        let wsize = map.size;
        let mut slopes_per_cell = 0.0;
        let mut half_slopes = 0.0;
        let mut processed_cell = 0;
        let mut c = 0;
        let mut minx = 0;
        let mut maxx = 0;
        // do while there are unblocked slopes left and the algo is within
        // the map's boundaries
        // scan progressive lines/columns from the focal-point outwards
        if y < 0 || y >= wsize as int {
            done = true; }
        while !done {
            // process cells in the line
            slopes_per_cell = 1.0 / (iteration as f64 + 1.0);
            half_slopes = slopes_per_cell * 0.5;
            processed_cell = (min_angle / slopes_per_cell) as int;
            let cpx = position_x as int + iteration;
            let cnx = position_x as int - iteration;
            minx = 0i.max(&cnx);
            maxx = (wsize as int - 1).min(&cpx);
            done = true;
            x = (position_x as int + (processed_cell * dx)) as int;
            while x >= minx && x <= maxx {
                c = x + (y * wsize as int);
                if c < 0 || c >= (wsize * wsize) as int {
                    fail!("idx:{:?} outside of map bounds.. shouldn't happen", c);
                }
                let mut visible = true;
                let mut start_slope = processed_cell as f64 * slopes_per_cell;
                let mut center_slope = start_slope + half_slopes;
                let mut end_slope = start_slope + slopes_per_cell;
                if obstacles_in_last_line > 0 && !map.tiles[c].visible {
                    let mut idx = 0;
                    while visible && idx < obstacles_in_last_line {
                        if map.tiles[c].allow_los {
                            if center_slope > start_angle[idx] &&
                                center_slope < end_angle[idx] {
                                visible = false;
                            }
                        } else if (start_slope >= start_angle[idx]) &&
                            (end_slope <= end_angle[idx]) {
                            visible = false;
                        }
                        let zy = x + ((y-dy) * wsize as int);
                        let zy_tile_trans = {
                            map.tiles[zy as uint].allow_los
                        };
                        let zyx = (x-dx) + ((y-dy) * wsize as int);
                        let zyx_tile_trans = {
                            &map.tiles[zyx as uint].allow_los
                        };
                        if (visible &&
                            (!map.tiles[zy].visible || !zy_tile_trans) &&
                            ((x - dx >= 0) && (x - dx < wsize as int) && ((!map.tiles[zyx].visible) ||
                                                                   (!zyx_tile_trans)))) {
                            visible = false;
                        }
                        idx += 1;
                    }
                }
                if visible {
                    let mut c_tile = &mut map.tiles[c];
                    c_tile.visible = true;
                    done = false;
                    if !c_tile.allow_los {
                        if min_angle >= start_slope { min_angle = end_slope; }
                        else {
                            start_angle[total_obstacles] = start_slope;
                            end_angle[total_obstacles] = end_slope;
                            total_obstacles += 1;
                        }
                    }
                }
                processed_cell += 1;
                x += dx;
            }
            if iteration == max_radius as int {
                done = true; }
            iteration += 1;
            obstacles_in_last_line = total_obstacles;
            y += dy;
            if y < 0 || y >= wsize as int {
                done = true; }
            if min_angle == 1.0 {
                done = true; }
        }
    }
    // octant: horizontal edge
    {
        let mut iteration = 1 as int;
        let mut done = false;
        let mut total_obstacles = 0;
        let mut obstacles_in_last_line = 0;
        let mut min_angle = 0.0;
        let mut y = 0 as int;
        let mut x = (position_x as int + dx) as int;
        let mut c = 0;
        let wsize = map.size;
        let mut slopes_per_cell = 0.0;
        let mut half_slopes = 0.0;
        let mut processed_cell = 0;
        let mut c = 0;
        let mut miny = 0;
        let mut maxy = 0;
        // do while there are unblocked slopes left and the algo is within
        // the map's boundaries
        // scan progressive lines/columns from the focal-point outwards
        if x < 0 || x >= wsize as int {
            done = true; }
        while !done {
            // process cells in the line
            slopes_per_cell = 1.0 / (iteration as f64 + 1.0);
            half_slopes = slopes_per_cell * 0.5;
            processed_cell = (min_angle / slopes_per_cell) as int;
            let cpy = position_y as int + iteration;
            let cny = position_y as int - iteration;
            miny = 0i.max(&cny);
            maxy = (wsize as int - 1).min(&cpy);
            done = true;
            y = (position_y as int + (processed_cell * dy)) as int;
            while y >= miny && y <= maxy {
                c = x + (y * wsize as int);
                if c < 0 || c >= (wsize * wsize) as int {
                    fail!("idx:{:?} outside of map bounds.. shouldn't happen", c);
                }
                let mut visible = true;
                let start_slope: f64 = processed_cell as f64 * slopes_per_cell;
                let center_slope: f64 = start_slope + half_slopes;
                let end_slope: f64 = start_slope + slopes_per_cell;
                if obstacles_in_last_line > 0 && !map.tiles[c].visible {
                    let mut idx = 0;
                    while visible && idx < obstacles_in_last_line {
                        if map.tiles[c].allow_los {
                            if center_slope > start_angle[idx] &&
                                center_slope < end_angle[idx] {
                                visible = false;
                            }
                        } else if (start_slope >= start_angle[idx]) &&
                            (end_slope <= end_angle[idx]) {
                            visible = false;
                        }
                        let zy = x-dx + (y * wsize as int);
                        let zy_tile_trans = {
                            map.tiles[zy as uint].allow_los
                        };
                        let zyx = (x-dx) + ((y-dy) * wsize as int);
                        let zyx_tile_trans = {
                            map.tiles[zyx as uint].allow_los
                        };
                        if (visible &&
                            (!map.tiles[zy].visible|| !zy_tile_trans) &&
                            ((y - dy >= 0) && (y - dy < wsize as int) && ((!map.tiles[zyx].visible) ||
                                                                   (!zyx_tile_trans)))) {
                            visible = false;
                        }
                        idx += 1;
                    }
                }
                if visible {
                    let mut c_tile = &mut map.tiles[c];
                    c_tile.visible = true;
                    done = false;
                    if !c_tile.allow_los {
                        if min_angle >= start_slope {
                            min_angle = end_slope; }
                        else {
                            debug!("set a/to");
                            start_angle[total_obstacles] = start_slope;
                            end_angle[total_obstacles] = end_slope;
                            total_obstacles += 1;
                        }
                    } else { debug!("don't set min_angle"); }
                }
                processed_cell += 1;
                y += dy;
            }
            if iteration == max_radius as int {
                debug!("hdt: iteration == max_radius");
                done = true; }
            iteration += 1;
            obstacles_in_last_line = total_obstacles;
            x += dx;
            if x < 0 || x >= wsize as int {
                done = true; }
            if min_angle == 1.0 {
                done = true; }
        }
    }
}

pub fn compute(map: &mut Map, focus: (uint, uint), max_radius: uint,
            start_angle_buf: &mut [f64], end_angle_buf: &mut [f64]) {
    // always mark focus as visible
    let (fx, fy) = focus;
    map.tiles[fx+(fy * map.size)].visible = true;
    // SE quad
    compute_quad(map, focus, max_radius,
                 start_angle_buf, end_angle_buf, 1, 1);
    // NE quad
    compute_quad(map, focus, max_radius,
                 start_angle_buf, end_angle_buf, 1, -1);
    // SW quad
    compute_quad(map, focus, max_radius,
                 start_angle_buf, end_angle_buf, -1, 1);
    // NW quad
    compute_quad(map, focus, max_radius,
                 start_angle_buf, end_angle_buf, -1, -1);
}
