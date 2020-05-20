use std::env;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use std::thread;
use clap::{App, AppSettings, Arg};

#[derive(Debug)]
struct Process {
    name:   String,
    path:   String,
    args:   Vec<String>,
    cwd:    Option<String>,
}

fn main() {
    let mut processes : Vec<Arc<Process>> = Vec::new();

    let matches = App::new("cmdr")
        .arg(Arg::new("cmd")
            .about("command to run")
            .takes_value(true)
            .value_name("COMMAND")
            .short('c')
            .long("cmd")
            .multiple(true)
            .required(true),
        )
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    let cmds: Vec<&str> = matches.values_of("cmd").unwrap().collect();
    let cwd = env::current_dir().unwrap();

    for cmd_line_str in cmds {
        let cmd_line: Vec<&str> = cmd_line_str.split(' ').collect();
        let args: Vec<&str> = cmd_line[1..].to_vec();

        processes.push(Arc::new(Process {
            name:   cmd_line[0].to_string(),
            path:   cmd_line[0].to_string(),
            args:   args.iter().map(ToString::to_string).collect(),
            cwd:    Some(cwd.display().to_string()),
        }));
    }

    let mut cmd_handles: Vec<std::thread::JoinHandle<()>> = vec!();

    for process in processes {
        cmd_handles.push(spawn_cmd_thread(process.clone()));
    }

    for cmd_handle in cmd_handles {
        cmd_handle.join().unwrap();
    }
}

fn spawn_cmd_thread(process: Arc<Process>) -> std::thread::JoinHandle<()>{
    return thread::Builder::new()
        .name(process.name.clone())
        .spawn(move || {
            let mut cmd_spawn = spawn_cmd(&*process);
            let status = cmd_spawn.wait().expect("Command did not run!");
            println!("{}: {}", process.name, status);
        }).expect("Failed to spawn command thread!");
}

fn spawn_cmd(process: &Process) -> Child {
    println!("Launching Process {}", process.name);
    let mut cmd = Command::new(&process.path);
    cmd.args(&process.args);

    if (process.cwd).is_some() {
        cmd.current_dir(process.cwd.as_ref().unwrap());
    }

    cmd.stdin(Stdio::null());
    cmd.stdout(Stdio::inherit());

    cmd.spawn().expect("Failed to spawn command!")
}
