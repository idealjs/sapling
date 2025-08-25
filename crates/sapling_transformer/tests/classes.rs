mod test_utils;

use crate::test_utils::run_test;

#[test]
pub fn counter_tsx() {
    let test_file = "tests/specs/classes/counter.tsx";
    run_test(test_file);
}
