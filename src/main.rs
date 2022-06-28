use std::io::stdin;
use std::process::Command;
use std::io::stdout;
use std::io::Write;


fn repl() {
    // use the `>` character as the prompt
    // need to explicitly flush this to ensure it prints before `read_line`
    print!("> ");
    stdout().flush();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    // read_line leaves a trailing newline, which rtim removes
    let command = input.trim();

    let mut child = Command::new(command)
        .spawn()
        .unwrap();

    // don't accept another command until this one completes
    child.wait();
}

fn main() {
    loop {
        repl();
    }
}
