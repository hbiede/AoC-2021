use aoc_2021::day15 as day;

#[test]
fn test_part1() {
    let inputs = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
    let expected = 40;
    let result = day::part1(inputs.to_string());
    assert_eq!(expected, result);
}

#[test]
fn test_part2() {
    let inputs = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
    let expected = 315;
    let result = day::part2(inputs.to_string());
    assert_eq!(expected, result);
}
