use aoc_2021::day02 as day;

#[test]
fn test_part1() {
    let inputs = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
    let expected = 150;
    let result = day::part1(inputs.to_string());
    assert_eq!(expected, result);
}

#[test]
fn test_part2() {
    let inputs = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
    let expected = 900;
    let result = day::part2(inputs.to_string());
    assert_eq!(expected, result);
}