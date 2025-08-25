use camino::Utf8Path;
use sapling_transformer::{
    transform, write_transformation_snapshot::write_transformation_snapshot,
};
use std::fs::read_to_string;

pub fn run_test(input: &'static str) -> Option<()> {
    let input_file = Utf8Path::new(input);
    let file_name = input_file.file_name().unwrap();

    let mut snapshot = String::new();

    let input_code = read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {input_file:?}: {err:?}"));

    let formatted = transform(input_code.clone())?;

    write_transformation_snapshot(
        &mut snapshot,
        input_code.clone().as_str(),
        formatted.as_str(),
        input_file.extension()?,
    );

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => {
            let path = input_file.parent().unwrap();
            let path_str = path.as_str();
            if let Some(idx) = path_str.find("specs/") {
                Utf8Path::new(&path_str[idx..])
            } else {
                path
            }
        },
    },
    {
        insta::assert_snapshot!(file_name, snapshot, file_name);
    });

    None
}
