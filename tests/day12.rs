use serde_json::Value;

const INPUT: &str = include_str!("res/12.txt");

#[test]
fn part1() {
    let ans: i64 = INPUT
        .split(|c: char| !c.is_alphanumeric() && c != '-')
        .filter_map(|token| match token.parse::<i64>() {
            Ok(n) => Some(n),
            Err(_) => None,
        })
        .sum();
    println!("Day 12, part 1: {}", ans);
    assert_eq!(119433, ans);
}

fn walking_sum(value: &Value) -> i64 {
    match value {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(arr) => arr.iter().map(walking_sum).sum(),
        Value::Object(obj) => {
            let contains_red = obj
                .values()
                .filter_map(|value| match value {
                    Value::String(color) => Some(color),
                    _ => None,
                })
                .any(|color| color == "red");
            if contains_red {
                0
            } else {
                obj.values().map(walking_sum).sum()
            }
        }
        _ => 0,
    }
}

#[test]
fn part2() {
    let root: Value = serde_json::from_str(INPUT).unwrap();
    let ans = walking_sum(&root);
    println!("Day 12, part 2: {}", ans);
    assert_eq!(68466, ans);
}
