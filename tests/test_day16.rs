use aoc_2021::day16 as day;

#[test]
fn test_part1() {
    let mut inputs = "EE00D40C823060";
    let mut expected = 14;
    let mut result = day::part1(inputs.to_string());
    assert_eq!(expected, result);

    inputs = "38006F45291200";
    expected = 9;
    result = day::part1(inputs.to_string());
    assert_eq!(expected, result);

    inputs = "8A004A801A8002F478";
    expected = 16;
    result = day::part1(inputs.to_string());
    assert_eq!(expected, result);

    inputs = "620080001611562C8802118E34";
    expected = 12;
    result = day::part1(inputs.to_string());
    assert_eq!(expected, result);

    inputs = "C0015000016115A2E0802F182340";
    expected = 23;
    result = day::part1(inputs.to_string());
    assert_eq!(expected, result);

    inputs = "A0016C880162017C3686B18A3D4780";
    expected = 31;
    result = day::part1(inputs.to_string());
    assert_eq!(expected, result);
}

#[test]
fn test_part2() {
    let mut inputs = "C200B40A82";
    let mut expected = 3;
    let mut result = day::part2(inputs.to_string());
    assert_eq!(expected, result);

    inputs = "04005AC33890";
    expected = 54;
    result = day::part2(inputs.to_string());
    assert_eq!(expected, result);

    inputs = "880086C3E88112";
    expected = 7;
    result = day::part2(inputs.to_string());
    assert_eq!(expected, result);

    inputs = "CE00C43D881120";
    expected = 9;
    result = day::part2(inputs.to_string());
    assert_eq!(expected, result);

    inputs = "D8005AC2A8F0";
    expected = 1;
    result = day::part2(inputs.to_string());
    assert_eq!(expected, result);

    inputs = "F600BC2D8F";
    expected = 0;
    result = day::part2(inputs.to_string());
    assert_eq!(expected, result);

    inputs = "9C005AC2F8F0";
    expected = 0;
    result = day::part2(inputs.to_string());
    assert_eq!(expected, result);

    inputs = "9C0141080250320F1802104A08";
    expected = 1;
    result = day::part2(inputs.to_string());
    assert_eq!(expected, result);
}
