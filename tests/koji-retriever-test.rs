use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

const KOJI_RETRIEVER_BINARY: &str = "koji-retriever";

#[test]
fn url_existing_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(KOJI_RETRIEVER_BINARY)?;
    cmd.arg("-u")
        .arg("https://koji.fedoraproject.org/koji/buildinfo?buildID=2166955")
        .arg("-d")
        .arg("/tmp");
    cmd.assert().success().stdout(predicate::str::contains(
        "/tmp/pykickstart-3.45-1.fc39.src.rpm",
    ));
    Ok(())
}

#[test]
fn url_does_not_exist_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(KOJI_RETRIEVER_BINARY)?;
    cmd.arg("-u").arg("https://unexisting-url.almost.sure");
    cmd.assert().failure();
    Ok(())
}

#[test]
fn url_existing_verbose_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(KOJI_RETRIEVER_BINARY)?;
    cmd.arg("-u")
        .arg("https://koji.fedoraproject.org/koji/buildinfo?buildID=2166955")
        .arg("-v")
        .arg("-d")
        .arg("/tmp");
    cmd.assert().success().stdout(predicate::str::contains(
        "/tmp/pykickstart-3.45-1.fc39.src.rpm",
    ));
    Ok(())
}
