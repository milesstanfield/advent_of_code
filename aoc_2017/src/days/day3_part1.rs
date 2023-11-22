pub fn run(input: &String) {
    let input_num: usize = String::from(input).parse().expect("NaN");
    let root = root(input_num); // 3, 5, 7, 9, 11
    let center = (root - 1) / 2; // 1, 2, 3;
    let grid = grid(root, center);
    let steps = steps(&grid, input_num, center);

    println!("{}", steps);
}

fn steps(grid: &Vec<Vec<usize>>, target: usize, center: usize) -> usize {
    let (target_x, target_y) = position(&grid, &target);
    target_x.abs_diff(center) + target_y.abs_diff(center)
}

fn position(grid: &Vec<Vec<usize>>, target: &usize) -> (usize, usize) {
    for (y, rows) in grid.into_iter().enumerate() {
        for (x, data) in rows.into_iter().enumerate() {
            if data == target {
                return (x, y);
            }
        }
    }

    panic!(r#"target "{}" missing!?"#, target);
}

fn grid(root: usize, center: usize) -> Vec<Vec<usize>> {
    let squared = root * root; // 9, 25, 49, 81, 121
    let column: Vec<usize> = vec![0; root];

    let mut rows: Vec<Vec<usize>> = vec![column; root];
    let mut step: usize = 1;
    let (mut x, mut y) = (center, center);
    let mut dir = 3;
    rows[y][x] = 1;

    while step < squared {
        step += 1;
        (x, y, dir) = walk(&rows, (x, y), dir);
        rows[y][x] = step;
    }

    rows
}

fn root(input_num: usize) -> usize {
    let mut sqroot = 1;
    while sqroot * sqroot < input_num {
        sqroot += 2;
    }
    sqroot
}

fn walk(rows: &Vec<Vec<usize>>, pos: (usize, usize), dir: usize) -> (usize, usize, usize) {
    let (x, y) = pos;

    let next_dir = match dir {
        0 => 1, // up
        1 => 2, // left
        2 => 3, // down
        _ => 0, // right
    };

    if next_dir == 1 && y != 0 && rows[y - 1][x] == 0 {
        (x, y - 1, next_dir)
    } else if next_dir == 2 && x != 0 && rows[y][x - 1] == 0 {
        (x - 1, y, next_dir)
    } else if next_dir == 3 && rows.len() > y + 1 && rows[y + 1][x] == 0 {
        (x, y + 1, next_dir)
    } else if next_dir == 0 && rows[y].len() > x + 1 && rows[y][x + 1] == 0 {
        (x + 1, y, next_dir)
    } else {
        match dir {
            0 => (x + 1, y, dir),
            1 => (x, y - 1, dir),
            2 => (x - 1, y, dir),
            _ => (x, y + 1, dir),
        }
    }
}
