const EXAMPLE_DATA: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

pub fn day1(data: &str) {
    let data = EXAMPLE_DATA;

    let result = data
        .lines()
        .map(|v| {
            let (direction, amount) = v.split_at(1);
            let amount: i64 = amount.parse().unwrap();
            return if direction == "L" { -amount } else { amount };
        })
        .fold((0, 0, 50), |(mut counts, mut total_counts, mut current), change| {
            let capped_change = change % 100;
            let start = current;

            current = (current + capped_change).rem_euclid(100);
            if current == 0 {
                counts += 1;
            }

            total_counts += change.abs() / 100;
            if start + capped_change >= 100 || (start != 0 && start + capped_change <= 0) {
                total_counts += 1;
            }

            return (counts, total_counts, current);
        });

    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1);
}
