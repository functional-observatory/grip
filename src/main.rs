use std::env;
use std::process::exit;
use pervasives::file::relative;
use pervasives::proc::call_command;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.get(1).is_none() {
        println!("no arg passed, need at least one, PROJECT_NAME");
        exit(1)
    }

    if args.get(1).unwrap() == "--help" {
        println!("USAGE: grip PROJECT_NAME");
        exit(0)
    }

    let project_name = args.get(1).unwrap();

    call_command(&*format!("git clone git@github.com:rajatsharma/{}.git", project_name))
        .expect("unable to clone repo");

    let relative_dir = relative(project_name).unwrap();

    env::set_current_dir(relative_dir.clone()).expect("unable to change dir");

    if relative_dir.join("pnpm-lock.yaml").exists() {
        call_command("pnpm i").expect("unable to clone repo");
    }

    if relative_dir.join("stack.yaml").exists() {
        call_command("stack build").expect("unable to run stack build");
    }

    if relative_dir.join("go.mod").exists() {
        call_command("go mod tidy").expect("unable to clone repo");
    }

    if relative_dir.join("yarn.lock").exists() {
        call_command("yarn").expect("unable to clone repo");
    }

    println!("Gripped rajatsharma/{} 🎉", project_name)
}
