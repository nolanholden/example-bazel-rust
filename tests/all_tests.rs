extern crate mylib;

#[test]
fn test_mylib() {
    assert_eq!("mylib world", mylib::world());
}

macro_rules! in_temp_dir {
    ($block:block) => {
        let tmpdir = tempfile::tempdir().unwrap();
        std::env::set_current_dir(&tmpdir).unwrap();
        $block;
    };
}

use std::process::Command;

fn run_shell(bash_content: &str) -> (String, String) {
    let o = Command::new("/bin/bash")
        .arg("-c")
        .arg(bash_content)
        .output()
        .unwrap();
    let rc = o.status;
    let stdout: String = String::from_utf8_lossy(&o.stdout).to_string();
    let stderr: String = String::from_utf8_lossy(&o.stderr).to_string();
    if !rc.success() {
        println!("status: {}", rc);
        println!("stdout: {}", stdout);
        println!("stderr: {}", stderr);
        panic!("command failed");
    }
    return (stdout, stderr);
}

#[test]
fn test_run_shell_success() {
    run_shell("echo ok");
    in_temp_dir!({
        assert_eq!(run_shell("ls").0, "");
        run_shell("touch abc");
        assert_eq!(run_shell("ls").0, "abc\n");
    });
}

// Uncomment to test failure:
//
// #[test]
// fn test_run_shell_fail() {
//     run_shell("echo2 not-ok");
// }

#[test]
fn test_run_runfiles_rust_binary() {
    // Actually, runfiles not needed, our binary is already in the path.
    // Still, I'll leave the code here for reference.
    //
    // let runfiles = Runfiles::create().unwrap();
    // let my_rust_binary_path = runfiles.rlocation("example-bazel-rust/bazel-bin/my_rust_binary");
    // let my_rust_binary_path_str: String = my_rust_binary_path.into_os_string().into_string().unwrap();

    let cmd: String = format!("{} arg1 arg2", "./my_rust_binary");
    let stdout = run_shell(&cmd).0;
    assert_eq!(stdout, "Hello mylib world\n");
}
