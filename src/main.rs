use std::{env, process, path::Path};
use std::collections::HashMap;

use rushbox::{
    cmd_add_shell,
    cmd_addgroup,
    cmd_echo,
    cmd_false,
    cmd_true,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let prog_name = Path::new(&args[1])
                    .file_name().unwrap()
                    .to_str().unwrap();

    let cmds = Command::new();
    let code = match cmds.get(prog_name) {
        Some(execute) => {
            execute(&args[2..]).unwrap_or(-1)
        },
        None => {
            eprintln!("Not found the command: {}", prog_name);
            -1
        },
    };

    process::exit(code);
}

type CommandFn = fn(&[String]) -> Result<i32, Box<dyn std::error::Error>>;

struct Command;
impl Command {
    fn new() -> HashMap<&'static str, CommandFn> {
        let pairs = [
            ("add-shell", cmd_add_shell::main as CommandFn),
            ("addgroup", cmd_addgroup::main),
            ("echo", cmd_echo::main),
            ("false", cmd_false::main),
            ("true", cmd_true::main),
        ];

        HashMap::from_iter(pairs.into_iter())
    }
}

