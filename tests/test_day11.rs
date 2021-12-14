use aoc_2021::day11 as day;

#[test]
fn test_part1() {
    let inputs = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
    let expected = 1656;
    let result = day::part1(inputs.to_string());
    assert_eq!(expected, result);
}

#[test]
fn test_part2() {
    let inputs = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
    let expected = 195;
    let result = day::part2(inputs.to_string());
    assert_eq!(expected, result);
}