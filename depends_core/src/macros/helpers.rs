use std::{
    io::Write,
    process::{Command, Stdio},
};

pub fn format_source(source: &str) -> String {
    let mut rustfmt = Command::new("rustfmt")
        .args(["+nightly", "--edition", "2021"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to run rustfmt");

    {
        let stdin = rustfmt
            .stdin
            .as_mut()
            .expect("stdin was not created for `rustfmt` child process");
        stdin
            .write_all(source.as_bytes())
            .expect("failed to write to stdin");
    }

    let output = rustfmt.wait_with_output().unwrap();
    if !output.status.success() {
        panic!(
            "`rustfmt` exited with code {}:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr),
        );
    }

    String::from_utf8(output.stdout).unwrap()
}
