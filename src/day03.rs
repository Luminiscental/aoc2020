use crate::day::Day;

pub struct Grid {
    rows: Vec<Vec<bool>>,
}

impl Grid {
    fn width(&self) -> usize {
        self.rows[0].len()
    }

    fn height(&self) -> usize {
        self.rows.len()
    }

    fn tree_at(&self, x: usize, y: usize) -> bool {
        self.rows[y][x % self.width()]
    }

    fn count_trees(&self, x_slope: usize, y_slope: usize) -> usize {
        let mut x = 0;
        let mut y = 0;
        let mut trees = 0;
        while y < self.height() {
            if self.tree_at(x, y) {
                trees += 1;
            }
            x += x_slope;
            y += y_slope;
        }
        trees
    }
}

pub struct Day03 {}

impl<'a> Day<'a> for Day03 {
    type Input1 = Grid;
    type Input2 = Grid;
    type Output1 = usize;
    type Output2 = usize;

    const INDEX: usize = 3;

    fn parse(raw_input: &'a str) -> Self::Input1 {
        Grid {
            rows: raw_input
                .lines()
                .filter(|line| !line.is_empty())
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect(),
        }
    }

    fn solve_part1(input: Self::Input1) -> (Self::Input2, Self::Output1) {
        let count = input.count_trees(3, 1);
        (input, count)
    }

    fn solve_part2(input: Self::Input2) -> Self::Output2 {
        input.count_trees(1, 1)
            * input.count_trees(3, 1)
            * input.count_trees(5, 1)
            * input.count_trees(7, 1)
            * input.count_trees(1, 2)
    }
}
