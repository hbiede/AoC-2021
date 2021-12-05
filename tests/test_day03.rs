use aoc_2021::day03 as day;

#[test]
fn test_part1() {
    let inputs = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    let expected = 198;
    let result = day::part1(inputs.to_string());
    assert_eq!(expected, result);
}

#[test]
fn test_part2() {
    let inputs = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    let expected = 230;
    let result = day::part2(inputs.to_string());
    assert_eq!(expected, result);
}