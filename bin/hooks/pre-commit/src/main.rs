fn main() {
    cmd("cargo", &["check"], &[]);
    cmd("cargo", &["fmt", "--check", "--all"], &[]);
    cmd(
        "cargo",
        &["clippy", "--workspace", "--", "-Dclippy::all"],
        &[],
    );
}

fn cmd(cmd: &str, args: &[&str], env: &[(&str, &str)]) {
    println!("=== Running command: {cmd} {args:?}");
    let mut builder = std::process::Command::new(cmd);
    builder.args(args);
    builder.envs(env.iter().copied());
    let mut child = builder.spawn().unwrap();
    if !child.wait().unwrap().success() {
        panic!("Failed to run command: {} {:?}", cmd, builder);
    }
}
