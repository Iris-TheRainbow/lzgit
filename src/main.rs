use std::env;
use std::process::Command;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    for arg in args {
        commitmsg.push_str(&arg);
        commitmsg.push_str(" ");
    }
    println!("{}", commitmsg);
    let out1 = Command::new("git")
        .arg("stage")
        .arg(".")
        .output()
        .expect("unable to execute");
    let out1 = String::from_utf8(out1.stdout).unwrap();
    print!("{}", out1);

    let out2 = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commitmsg)
        .output()
        .expect("unable to execute");
    let out2 = String::from_utf8(out2.stdout).unwrap();
    print!("{}", out2);
    let out3 = Command::new("git")
        .arg("push")
        .output()
        .expect("unable to execute");
    let out3 = String::from_utf8(out3.stdout).unwrap();
    print!("{}", out3);
}
