use std::ops::Range;

pub fn run(input: &String) {
    let mut offsets = offsets(&input);
    let mut _offset: i32 = 0;
    let mut _abs_offset: i32 = 0;
    let mut pos: i32 = 0;
    let mut step: u32 = 0;
    let size = offsets.len() as i32;
    let pos_range: Range<i32> = 0..size;

    while pos_range.contains(&pos) {
        _offset = offsets[pos as usize];
        _abs_offset = _offset.abs();

        if _offset >= 3 {
            offsets[pos as usize] -= 1; // jump back
        } else {
            offsets[pos as usize] += 1; // jump forward
        }

        if _offset.is_positive() {
            pos += _abs_offset;
        } else if _offset != 0 {
            pos -= _abs_offset;
        }
        step += 1;
    }

    println!("({:?}) steps", step);
}

fn offsets(input: &str) -> Vec<i32> {
    let mut offsets: Vec<i32> = vec![];
    for offset_str in input.lines() {
        offsets.push(String::from(offset_str).parse().expect("NaN"));
    }
    offsets
}
