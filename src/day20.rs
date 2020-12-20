use crate::{day::Day, util};
use itertools::iproduct;
use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
};

#[derive(Debug)]
pub struct Tile {
    id: usize,
    data: [[bool; 10]; 10],
}

impl Tile {
    fn orientations(&self) -> Orientations<'_> {
        Orientations {
            tile: self,
            next_rotate: 0,
            next_flip: 0,
        }
    }
}

#[derive(Clone, Copy)]
struct OrientedTile<'a> {
    tile: &'a Tile,
    rotate: usize,
    flip_x: bool,
    flip_y: bool,
}

impl<'a> OrientedTile<'a> {
    fn cell(&self, x: usize, y: usize) -> bool {
        let (prerot_x, prerot_y) = match self.rotate.rem_euclid(4) {
            0 => (x, y),
            1 => (9 - y, x),
            2 => (9 - x, 9 - y),
            3 => (y, 9 - x),
            _ => unreachable!(),
        };
        let (x, y) = (
            if self.flip_x { 9 - prerot_x } else { prerot_x },
            if self.flip_y { 9 - prerot_y } else { prerot_y },
        );
        self.tile.data[y][x]
    }

    fn then_transform(self, rotate: usize, flip_x: bool, flip_y: bool) -> Self {
        let mut result = self;
        if flip_x {
            result.rotate = 4 - result.rotate;
            result.flip_x = !result.flip_x;
        }
        if flip_y {
            result.rotate = 4 - result.rotate;
            result.flip_y = !result.flip_y;
        }
        result.rotate = (result.rotate + rotate) % 4;
        result
    }

    fn side(&self, side: usize) -> [bool; 10] {
        match side {
            0 => [
                self.cell(9, 0),
                self.cell(9, 1),
                self.cell(9, 2),
                self.cell(9, 3),
                self.cell(9, 4),
                self.cell(9, 5),
                self.cell(9, 6),
                self.cell(9, 7),
                self.cell(9, 8),
                self.cell(9, 9),
            ],
            1 => [
                self.cell(0, 0),
                self.cell(1, 0),
                self.cell(2, 0),
                self.cell(3, 0),
                self.cell(4, 0),
                self.cell(5, 0),
                self.cell(6, 0),
                self.cell(7, 0),
                self.cell(8, 0),
                self.cell(9, 0),
            ],
            2 => [
                self.cell(0, 0),
                self.cell(0, 1),
                self.cell(0, 2),
                self.cell(0, 3),
                self.cell(0, 4),
                self.cell(0, 5),
                self.cell(0, 6),
                self.cell(0, 7),
                self.cell(0, 8),
                self.cell(0, 9),
            ],
            3 => [
                self.cell(0, 9),
                self.cell(1, 9),
                self.cell(2, 9),
                self.cell(3, 9),
                self.cell(4, 9),
                self.cell(5, 9),
                self.cell(6, 9),
                self.cell(7, 9),
                self.cell(8, 9),
                self.cell(9, 9),
            ],
            _ => unreachable!(),
        }
    }
}

struct Orientations<'a> {
    tile: &'a Tile,
    next_rotate: usize,
    next_flip: u8,
}

impl<'a> Iterator for Orientations<'a> {
    type Item = OrientedTile<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_rotate < 4 {
            let item = OrientedTile {
                tile: self.tile,
                rotate: self.next_rotate,
                flip_x: self.next_flip & 1 == 1,
                flip_y: self.next_flip & 2 == 2,
            };
            if self.next_flip == 4 {
                self.next_rotate += 1;
                self.next_flip = 0;
            } else {
                self.next_flip += 1;
            }
            Some(item)
        } else {
            None
        }
    }
}

pub struct Day20 {}

impl<'a> Day<'a> for Day20 {
    type Input1 = Vec<Tile>;
    type Input2 = HashMap<(i32, i32), bool>;
    type Output1 = usize;
    type Output2 = usize;

    const INDEX: usize = 20;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        let lines: Vec<_> = raw_input.lines().collect();
        lines
            .split(|line| line.is_empty())
            .filter(|chunk| !chunk.is_empty())
            .map(|chunk| {
                let id_line = chunk[0];
                let data = &chunk[1..];
                Tile {
                    id: id_line[5..id_line.len() - 1].parse().unwrap(),
                    data: data
                        .iter()
                        .map(|line| {
                            line.chars()
                                .map(|c| c == '#')
                                .collect::<Vec<_>>()
                                .as_slice()
                                .try_into()
                                .unwrap()
                        })
                        .collect::<Vec<_>>()
                        .as_slice()
                        .try_into()
                        .unwrap(),
                }
            })
            .collect()
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let mut neighbours = HashMap::new();
        for tile in input.iter() {
            let tile_orient = tile.orientations().next().unwrap();
            let tile_sides = [
                tile_orient.side(0),
                tile_orient.side(1),
                tile_orient.side(2),
                tile_orient.side(3),
            ];
            for (i, &tile_side) in tile_sides.iter().enumerate() {
                let opposite_side = (2 + i) % 4;
                'search: for other in input.iter() {
                    if other.id == tile.id {
                        continue;
                    }
                    for other_orient in other.orientations() {
                        if other_orient.side(opposite_side) == tile_side {
                            neighbours
                                .entry(tile.id)
                                .or_insert([None, None, None, None])[i] = Some(other_orient);
                            break 'search;
                        }
                    }
                }
            }
        }
        let dx = [1, 0, -1, 0];
        let dy = [0, -1, 0, 1];
        let mut grid = HashMap::new();
        let mut border = Vec::new();
        let mut placed_ids = HashSet::new();
        grid.insert((0, 0), input[0].orientations().next().unwrap());
        placed_ids.insert(input[0].id);
        border.push((0, 0));
        while let Some((x, y)) = border.pop() {
            let placed = grid[&(x, y)];
            for (side, neighbour) in neighbours[&placed.tile.id].iter().enumerate() {
                if let Some(neighbour) = neighbour {
                    if !placed_ids.contains(&neighbour.tile.id) {
                        let side = if placed.flip_x { (6 - side) % 4 } else { side };
                        let side = if placed.flip_y { 4 - side } else { side };
                        let side = (side + placed.rotate) % 4;
                        let neighbour_pos = (x + dx[side], y + dy[side]);
                        grid.insert(
                            neighbour_pos,
                            neighbour.then_transform(placed.rotate, placed.flip_x, placed.flip_y),
                        );
                        placed_ids.insert(neighbour.tile.id);
                        border.push(neighbour_pos);
                    }
                }
            }
        }
        let ((x_min, x_max), (y_min, y_max)) = util::range_2d(grid.keys().cloned());
        let corner_product = iproduct!([x_min, x_max].iter(), [y_min, y_max].iter())
            .map(|xy| grid[&(*xy.0, *xy.1)].tile.id)
            .product();
        let mut new_grid = HashMap::new();
        for ((x, y), tile) in grid.into_iter() {
            for dy in 0..8 {
                for dx in 0..8 {
                    new_grid.insert(
                        (8 * x + dx as i32, 8 * y + dy as i32),
                        tile.cell(dx + 1, dy + 1),
                    );
                }
            }
        }
        (new_grid, corner_product)
    }

    fn solve_part2(input: Self::Input2) -> Self::Output2 {
        let ((x_min, x_max), (y_min, y_max)) = util::range_2d(input.keys().cloned());
        let mut monster_tiles = HashSet::new();
        let sea_monster = [
            (0, 0),
            (1, -1),
            (4, -1),
            (5, 0),
            (6, 0),
            (7, -1),
            (10, -1),
            (11, 0),
            (12, 0),
            (13, -1),
            (16, -1),
            (17, 0),
            (18, 0),
            (18, 1),
            (19, 0),
        ];
        let cos = [1, 0, -1, 0];
        let sin = [0, -1, 0, 1];
        let mut match_oriented = |x, y, rotate, flip| {
            let oriented_monster: Vec<_> = sea_monster
                .iter()
                .map(|pair| {
                    let (dx, dy) = pair;
                    let (dx, dy) = (*dx, if flip { -dy } else { *dy });
                    let (dx, dy) = (
                        dx * cos[rotate] - dy * sin[rotate],
                        dx * sin[rotate] + dy * cos[rotate],
                    );
                    (x + dx, y + dy)
                })
                .collect();
            if oriented_monster.iter().all(|pos| input[&pos]) {
                monster_tiles.extend(oriented_monster);
            }
        };
        #[allow(clippy::clippy::range_minus_one)]
        for y in y_min + 1..=y_max - 1 {
            for x in x_min..=x_max - 19 {
                match_oriented(x, y, 0, false);
                match_oriented(x, y, 0, true);
                match_oriented(x + 19, y, 2, false);
                match_oriented(x + 19, y, 2, true);
            }
        }
        for y in y_min + 19..=y_max {
            #[allow(clippy::clippy::range_minus_one)]
            for x in x_min + 1..=x_max - 1 {
                match_oriented(x, y, 1, false);
                match_oriented(x, y, 1, true);
                match_oriented(x, y - 19, 3, false);
                match_oriented(x, y - 19, 3, true);
            }
        }
        input
            .iter()
            .filter(|kv| *kv.1)
            .map(|kv| kv.0)
            .filter(|pos| !monster_tiles.contains(pos))
            .count()
    }
}
