use aoc_2021::day09 as day;

#[test]
fn test_part1() {
    let inputs = "2199943210
3987894921
9856789892
8767896789
9899965678";
    let expected = 15;
    let result = day::part1(inputs.to_string());
    assert_eq!(expected, result);
}

#[test]
fn test_part2() {
    let inputs = "2199943210
3987894921
9856789892
8767896789
9899965678";
    let expected = 1134;
    let result = day::part2(inputs.to_string());
    assert_eq!(expected, result);
}