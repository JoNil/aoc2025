pub static INPUT: &str = include_str!("../input/4.txt");
pub static TEST_INPUT: &str = include_str!("../input/4_test.txt");

pub fn a(input: &str) -> i32 {
    let map = input.as_bytes();
    let line_len = input.lines().next().unwrap().len();
    let stride = line_len + 1;

    let mut sum_of_ok = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, item) in line.bytes().enumerate() {
            if item == b'@' {
                let mut sum_of_occupied = 0;
                for (dx, dy) in &[
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (-1, 0),
                    (1, 0),
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                ] {
                    let other_x = x as i32 + *dx;
                    let other_y = y as i32 + *dy;

                    if other_x >= 0
                        && other_x < line_len as i32
                        && other_y >= 0
                        && let Some(item) = map.get((other_x + other_y * stride as i32) as usize)
                        && *item == b'@'
                    {
                        sum_of_occupied += 1;
                    }
                }

                if sum_of_occupied < 4 {
                    sum_of_ok += 1;
                }
            }
        }
    }

    sum_of_ok
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 13);
    assert_eq!(a(INPUT), 1478);
}

pub fn b(input: &str) -> i32 {
    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 0);
    assert_eq!(b(INPUT), 0);
}
