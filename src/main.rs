use std::io::stdin;
use std::process::Command;
use std::io::stdout;
use std::io::Write;
use std::path::Path;
use std::env;
use std::process;
use std::process::{Child, Stdio};

fn repl() {
    // use the `>` character as the prompt
    // need to explicitly flush this to ensure it prints before `read_line`
    print!("> ");
    stdout().flush();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    // must be peekable so we know when we are on the last command
    let mut commands = input.trim().split(" | ").peekable();
    let mut previous_command = None;

    while let Some(command) = commands.next() {
        let mut parts = command.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                // default to `/` as new directory if one was not provided
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
                previous_command = None;
            },
            "exit" => std::process::exit(0),
            command => {
                let stdin = previous_command
                    .map_or(Stdio::inherit(),
                    |output: Child| Stdio::from(output.stdout.unwrap())
                    );
                let stdout = if commands.peek().is_some() {
                    // there is anotehr command piped behind this one
                    // prepare to send output to the next command
                    Stdio::piped()
                } else {
                    // there are no more commands piped behind this one
                    // send output to shell stdout
                    Stdio::inherit()
                };


                let output = Command::new(command)
                    .args(args)
                    .stdin(stdin)
                    .stdout(stdout)
                    .spawn();

                match output {
                    Ok(output) => { previous_command = Some(output); },
                    Err(e) => {
                        previous_command = None;
                        eprintln!("{}", e);
                    },
                };
            }
        }
    }
    if let Some(mut final_command) = previous_command {
        // block until final command has finished
        final_command.wait();
    }
}


fn main() {
    loop {
        repl();
    }
}
