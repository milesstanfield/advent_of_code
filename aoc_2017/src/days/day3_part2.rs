pub fn run(input: &String) {
    let input_num: usize = String::from(input).parse().expect("NaN");
    let root = root(input_num); // 3, 5, 7, 9, 11
    let center = (root - 1) / 2; // 1, 2, 3;
    grid(input_num, root, center);
}

fn grid(input_num: usize, root: usize, center: usize) -> Vec<Vec<usize>> {
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
        rows[y][x] = adjacent_sum(&rows, x, y);

        if input_num < rows[y][x] {
            println!("first value that is larger than input: {:?}", rows[y][x]);
            return rows;
        }
    }

    rows
}

fn adjacent_sum(rows: &Vec<Vec<usize>>, x: usize, y: usize) -> usize {
    let mut sum: usize = 0;

    if y != 0 {
        sum += rows[y - 1][x]; // top
    }

    if x != 0 {
        sum += rows[y][x - 1]; // left
    }

    if rows.len() > y + 1 {
        sum += rows[y + 1][x]; // bottom
    }

    if rows[y].len() > x + 1 {
        sum += rows[y][x + 1]; // right
    }

    if y != 0 && rows[y].len() > x + 1 {
        sum += rows[y - 1][x + 1]; // top right
    }

    if rows.len() > y + 1 && rows[y].len() > x + 1 {
        sum += rows[y + 1][x + 1]; // bottom right
    }

    if rows.len() > y + 1 && x != 0 {
        sum += rows[y + 1][x - 1]; // bottom left
    }

    if y != 0 && x != 0 {
        sum += rows[y - 1][x - 1]; // top left
    }

    sum
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
