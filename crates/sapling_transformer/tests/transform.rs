mod test_utils;

use sapling_transformer::transform;

#[test]
pub fn counter_tsx() {
    let transformed = transform("const a = <div/>".into());
    insta::assert_snapshot!(format!("{:?}", transformed));
}
