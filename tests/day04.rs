const INPUT: &str = "yzbqklnj";

#[test]
fn part1() {
    let mut ans = None;
    for i in 282749..1_000_000 {
        let digest = md5::compute(format!("{}{:06}", INPUT, i));
        if format!("{:?}", digest).starts_with("00000") {
            ans = Some(i);
            break;
        }
    }
    println!("Day 4, part 1: {}", ans.unwrap());
    assert_eq!(282749, ans.unwrap())
}

#[test]
fn part2() {
    let mut ans = None;
    for i in 9962624..10_000_000 {
        let digest = md5::compute(format!("{}{:06}", INPUT, i));
        if format!("{:?}", digest).starts_with("000000") {
            ans = Some(i);
            break;
        }
    }
    println!("Day 4, part 2: {}", ans.unwrap());
    assert_eq!(9962624, ans.unwrap());
}
