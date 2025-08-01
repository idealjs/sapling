mod test_utils;

use crate::test_utils::run_test;

#[test]
pub fn index_tsx() {
    let test_file = "tests/specs/simple_fragments/index.tsx";
    run_test(test_file);
}
