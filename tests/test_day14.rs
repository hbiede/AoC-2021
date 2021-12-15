use aoc_2021::day14 as day;

#[test]
fn test_part1() {
    let inputs = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
    let expected = 1588;
    let result = day::part1(inputs.to_string());
    assert_eq!(expected, result);
}

#[test]
fn test_part2() {
    let inputs = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
    let expected = 2188189693529;
    let result = day::part2(inputs.to_string());
    assert_eq!(expected, result);
}
