// MIT License
//
// Copyright (c) 2023 Sergio Arroutbi
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

const KOJI_RETRIEVER_BINARY: &str = "koji-retriever";
const LS_CMD: &str = "/usr/bin/ls";
const RM_CMD: &str = "/usr/bin/rm";

#[test]
fn url_existing_file_does_not_exist_in_test_mode_test() -> Result<(), Box<dyn std::error::Error>> {
    // Clean previous downloads (if any)
    let mut rm_cmd = Command::cargo_bin(RM_CMD)?;
    rm_cmd
        .arg("-v")
        .arg("-f")
        .arg("/tmp/pykickstart-3.47-1.fc38.src.rpm");
    let mut cmd = Command::cargo_bin(KOJI_RETRIEVER_BINARY)?;
    cmd.arg("-u")
        .arg("https://koji.fedoraproject.org/koji/buildinfo?buildID=2171737")
        .arg("-d")
        .arg("/tmp")
        .arg("-t");
    cmd.assert().success();
    let mut cmd_ls = Command::cargo_bin(LS_CMD)?;
    cmd_ls.arg("/tmp/pykickstart-3.47-1.fc38.src.rpm");
    cmd_ls
        .assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
    Ok(())
}

#[test]
fn url_existing_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(KOJI_RETRIEVER_BINARY)?;
    cmd.arg("-u")
        .arg("https://koji.fedoraproject.org/koji/buildinfo?buildID=2249970")
        .arg("-d")
        .arg("/tmp");
    cmd.assert().success().stdout(predicate::str::contains(
        "/tmp/pykickstart-3.48-3.fc39.src.rpm",
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
        .arg("https://koji.fedoraproject.org/koji/buildinfo?buildID=2249970")
        .arg("-v")
        .arg("-d")
        .arg("/tmp");
    cmd.assert().success().stdout(predicate::str::contains(
        "/tmp/pykickstart-3.48-3.fc39.src.rpm",
    ));
    let standard_output = cmd
        .assert()
        .success()
        .try_stdout(predicate::str::is_empty());
    assert!(standard_output.is_err());
    Ok(())
}

#[test]
fn url_existing_test_mode_verbose_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(KOJI_RETRIEVER_BINARY)?;
    cmd.arg("-u")
        .arg("https://koji.fedoraproject.org/koji/buildinfo?buildID=2249970")
        .arg("-v")
        .arg("-d")
        .arg("/tmp")
        .arg("-t");
    cmd.assert().success().stdout(predicate::str::contains(
        "/tmp/pykickstart-3.48-3.fc39.src.rpm",
    ));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Test mode"));
    let standard_output = cmd
        .assert()
        .success()
        .try_stdout(predicate::str::is_empty());
    assert!(standard_output.is_err());
    Ok(())
}

#[test]
fn url_existing_slash_end_directory_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(KOJI_RETRIEVER_BINARY)?;
    cmd.arg("-u")
        .arg("https://koji.fedoraproject.org/koji/buildinfo?buildID=2249970")
        .arg("-v")
        .arg("-d")
        .arg("/tmp");
    cmd.assert().success().stdout(predicate::str::contains(
        "/tmp/pykickstart-3.48-3.fc39.src.rpm",
    ));
    Ok(())
}

#[test]
fn url_existing_files_exist_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(KOJI_RETRIEVER_BINARY)?;
    cmd.arg("-u")
        .arg("https://koji.fedoraproject.org/koji/buildinfo?buildID=2249970")
        .arg("-d")
        .arg("/tmp")
        .arg("-r");
    cmd.assert().success().stdout(predicate::str::contains(
        "/tmp/pykickstart-3.48-3.fc39.src.rpm",
    ));
    let mut cmd_ls = Command::cargo_bin(LS_CMD)?;
    cmd_ls.arg("/tmp/pykickstart-3.48-3.fc39.src.rpm");
    cmd.assert().success().stdout(predicate::str::contains(
        "/tmp/pykickstart-3.48-3.fc39.src.rpm",
    ));
    cmd_ls = Command::cargo_bin(LS_CMD)?;
    cmd_ls.arg("/tmp/pykickstart-3.48-3.fc39.noarch.rpm");
    cmd.assert().success().stdout(predicate::str::contains(
        "/tmp/pykickstart-3.48-3.fc39.noarch.rpm",
    ));
    cmd_ls = Command::cargo_bin(LS_CMD)?;
    cmd_ls.arg("/tmp/python3-kickstart-3.48-3.fc39.noarch.rpm");
    cmd.assert().success().stdout(predicate::str::contains(
        "/tmp/python3-kickstart-3.48-3.fc39.noarch.rpm",
    ));
    Ok(())
}
