use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("day_1_1").unwrap();
    cmd.assert().success();
}