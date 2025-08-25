mod test_utils;
use crate::test_utils::run_test;

#[test]
pub fn counter_tsx() {
    let test_file = "tests/specs/counter.tsx";
    run_test(test_file);
}

#[test]
pub fn fragments() {
    let test_file = "tests/specs/fragments.tsx";
    run_test(test_file);
}

#[test]
pub fn jsx_attr() {
    let test_file = "tests/specs/jsx_attr.tsx";
    run_test(test_file);
}

#[test]
pub fn jsx_expr() {
    let test_file = "tests/specs/jsx_expr.tsx";
    run_test(test_file);
}

#[test]
pub fn jsx_self_close() {
    let test_file = "tests/specs/jsx_self_close.tsx";
    run_test(test_file);
}

#[test]
pub fn simple_element() {
    let test_file = "tests/specs/simple_element.tsx";
    run_test(test_file);
}

#[test]
pub fn todo_list() {
    let test_file = "tests/specs/todo_list.tsx";
    run_test(test_file);
}
