use aoc_2021::day01;

#[test]
fn test_part1() {
    let inputs = "199
200
208
210
200
207
240
269
260
263";
    let expected = 7;
    let result = day01::part1(inputs.to_string());
    assert_eq!(expected, result);
}

#[test]
fn test_part2() {
    let inputs = "199
200
208
210
200
207
240
269
260
263";
    let expected = 5;
    let result = day01::part2(inputs.to_string());
    assert_eq!(expected, result);
}