use aoc_2021::day13 as day;

#[test]
fn test_part1() {
    let inputs = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
    let expected = 17;
    let result = day::part1(inputs.to_string());
    assert_eq!(expected, result);
}

#[test]
fn test_part2() {
    let inputs = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
    let expected = 16;
    let result = day::part2(inputs.to_string());
    assert_eq!(expected, result);
}