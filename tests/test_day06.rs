use aoc_2021::day06 as day;

#[test]
fn test_part1() {
    let inputs = "3,4,3,1,2";
    let expected = 5934;
    let result = day::part1(inputs.to_string());
    assert_eq!(expected, result);
}

#[test]
fn test_part2() {
    let inputs = "3,4,3,1,2";
    let expected = 26984457539;
    let result = day::part2(inputs.to_string());
    assert_eq!(expected, result);
}