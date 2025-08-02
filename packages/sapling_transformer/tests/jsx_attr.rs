mod test_utils;

use crate::test_utils::run_test;

#[test]
pub fn index_tsx() {
    let test_file = "tests/specs/jsx_attr/index.tsx";
    run_test(test_file);
}
