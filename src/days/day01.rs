pub static INPUT: &str = include_str!("../input/1.txt");
pub static TEST_INPUT: &str = include_str!("../input/1_test.txt");

pub fn a(input: &str) -> i32 {
    let mut dial = 50;
    let mut zero_count = 0;

    for line in input.lines() {
        let line = line.as_bytes();

        let amt = (str::from_utf8(&line[1..]).unwrap())
            .parse::<i32>()
            .unwrap();

        match line[0] {
            b'L' => dial -= amt,
            b'R' => dial += amt,
            _ => (),
        }

        while dial > 99 {
            dial -= 100;
        }

        while dial < 0 {
            dial += 100;
        }

        if dial == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 3);
    assert_eq!(a(INPUT), 0);
}

pub fn b(input: &str) -> i32 {
    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 0);
    assert_eq!(b(INPUT), 0);
}
