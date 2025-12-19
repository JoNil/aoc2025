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
    assert_eq!(a(INPUT), 1118);
}

pub fn b(input: &str) -> i32 {
    let mut dial = 50;
    let mut zero_count = 0;

    for line in input.lines() {
        let line_b = line.as_bytes();

        let amt = (str::from_utf8(&line_b[1..]).unwrap())
            .parse::<i32>()
            .unwrap();

        let mut rotated = false;

        let started_as_zero = dial == 0;

        let full = amt / 100;
        let rest = amt % 100;
        zero_count += full;

        match line_b[0] {
            b'L' => dial -= rest,
            b'R' => dial += rest,
            _ => (),
        };

        if dial > 99 {
            if !started_as_zero {
                rotated = true;
                zero_count += 1;
            }

            dial -= 100;
        }

        if dial < 0 {
            if !started_as_zero {
                rotated = true;
                zero_count += 1;
            }

            dial += 100;
        }

        if dial == 0 && !rotated {
            zero_count += 1;
        }
    }

    zero_count
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 6);
    assert_eq!(b(INPUT), 6289);
}
