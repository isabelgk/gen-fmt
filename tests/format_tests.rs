#[test]
fn format_example() {
    let input = include_str!("input/example.genexpr");
    let expected = include_str!("expected/example.genexpr");
    let result = gen_fmt::format_str(input).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn format_for_loop() {
    let input = include_str!("input/for_loop.genexpr");
    let expected = include_str!("expected/for_loop.genexpr");
    let result = gen_fmt::format_str(input).unwrap();
    assert_eq!(result, expected);
}
