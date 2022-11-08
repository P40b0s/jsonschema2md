use std::process::Command;


pub fn run_container()
{
    Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process");
}
