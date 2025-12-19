pub static INPUT: &str = include_str!("../input/2.txt");
pub static TEST_INPUT: &str = include_str!("../input/2_test.txt");

pub fn a(input: &str) -> i64 {
    let mut sum = 0;

    for range in input.trim().split(',') {
        let (a, b) = range.split_once('-').unwrap();
        let a = a.parse::<i64>().unwrap();
        let b = b.parse::<i64>().unwrap();

        for num in a..=b {
            let nums = format!("{num}");
            let num_len = nums.len();
            let (a, b) = nums.split_at(num_len / 2);
            if a == b {
                sum += num
            }
        }
    }

    sum
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 1227775554);
    assert_eq!(a(INPUT), 15873079081);
}

pub fn b(input: &str) -> i32 {
    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 0);
    assert_eq!(b(INPUT), 0);
}
