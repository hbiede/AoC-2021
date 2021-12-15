use aoc_2021::day12 as day;

#[test]
fn test_part1() {
    let inputs = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    let expected = 10;
    let result = day::part1(inputs.to_string());
    assert_eq!(expected, result);

    let inputs2 = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
    let expected2 = 19;
    let result2 = day::part1(inputs2.to_string());
    assert_eq!(expected2, result2);

    let inputs3 = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
    let expected3 = 226;
    let result3 = day::part1(inputs3.to_string());
    assert_eq!(expected3, result3);
}

#[test]
fn test_part2() {
    let inputs = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    let expected = 36;
    let result = day::part2(inputs.to_string());
    assert_eq!(expected, result);

    let inputs2 = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
    let expected2 = 103;
    let result2 = day::part2(inputs2.to_string());
    assert_eq!(expected2, result2);

    let inputs3 = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
    let expected3 = 3509;
    let result3 = day::part2(inputs3.to_string());
    assert_eq!(expected3, result3);
}