macro_rules! fmt_test {
    ($name:ident) => {
        #[test]
        fn $name() {
            let input = include_str!(concat!("input/", stringify!($name), ".genexpr"));
            let expected = include_str!(concat!("expected/", stringify!($name), ".genexpr"));
            let result = gen_fmt::format_str(input, false, false).unwrap();
            assert_eq!(result, expected);
        }
    };
}

fmt_test!(example);
fmt_test!(for_loop);
fmt_test!(while_loop);
fmt_test!(ternary);
fmt_test!(break_continue);
fmt_test!(require_directive);
fmt_test!(multiple_assignment);
fmt_test!(unary_ops);
fmt_test!(array_index);
fmt_test!(block_comment);
fmt_test!(logical_operators);
fmt_test!(polymorphic_ops);
fmt_test!(all_declarations);
fmt_test!(default_params);
fmt_test!(inline_if);
fmt_test!(number_literals);
