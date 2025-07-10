use std::fmt::Write;

pub fn write_transformation_snapshot(
    snapshot: &mut String,
    input_code: &str,
    transformed_code: &str,
    extension: &str,
) {
    writeln!(snapshot, "# Input").unwrap();
    writeln!(snapshot, "```{extension}").unwrap();
    writeln!(snapshot, "{input_code}").unwrap();
    writeln!(snapshot, "```").unwrap();
    writeln!(snapshot).unwrap();
    writeln!(snapshot, "# Transformations").unwrap();
    writeln!(snapshot, "```{extension}").unwrap();
    writeln!(snapshot, "{transformed_code}").unwrap();
    writeln!(snapshot, "```").unwrap();
    writeln!(snapshot).unwrap();
}
