use aoc_2021::day07 as day;

#[test]
fn test_part1() {
    let inputs = "16,1,2,0,4,2,7,1,2,14";
    let expected = 37;
    let result = day::part1(inputs.to_string());
    assert_eq!(expected, result);
}

#[test]
fn test_part2() {
    let inputs = "16,1,2,0,4,2,7,1,2,14";
    let expected = 168;
    let result = day::part2(inputs.to_string());
    assert_eq!(expected, result);
}