use aoc_2021::day17 as day;

#[test]
fn test_part1() {
    let mut inputs = "target area: x=20..30, y=-10..-5";
    let mut expected = 45;
    let mut result = day::part1(inputs.to_string());
    assert_eq!(expected, result);

    inputs = "target area: x=102..157, y=-146..-90";
    expected = 10585;
    result = day::part1(inputs.to_string());
    assert_eq!(expected, result);
}

#[test]
fn test_part2() {
    let mut inputs = "target area: x=20..30, y=-10..-5";
    let mut expected = 112;
    let mut result = day::part2(inputs.to_string());
    assert_eq!(expected, result);

    inputs = "target area: x=102..157, y=-146..-90";
    expected = 5247;
    result = day::part2(inputs.to_string());
    assert_eq!(expected, result);
}
