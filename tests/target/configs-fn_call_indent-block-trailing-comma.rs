// rustfmt-error_on_line_overflow: false
// rustfmt-fn_call_indent: Block

// rustfmt should not add trailing comma when rewriting macro. See #1528.
fn a() {
    panic!(
        "this is a long string that goes past the maximum line length causing rustfmt to insert a comma here:"
    );
    foo(
        a,
        oooptoptoptoptptooptoptoptoptptooptoptoptoptptoptoptoptoptpt(),
    );
}
