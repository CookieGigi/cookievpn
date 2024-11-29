use assert_cmd::Command;

#[test]
fn cli_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cookievpn")?;

    cmd.assert().success();
    Ok(())
}
