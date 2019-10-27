use std::io::{stdin, stdout, Write};
use std::process::{Child, Command, Stdio};

struct CommandView {
    list_commands: String,
    prev_out: Option<Child>,
    command: String,
    args: Vec<String>
}

enum PreviousOutput{
    PrevCommand(String),
    NoPrevCommand
}


enum Arguments{
    OneArgument(String),
    MultipleArguments(Vec<String>),
    NoArguments
}

enum ListCommands{
    List(String),
    Empty
}

fn split_command(input: String) -> (String, Vec<String>, String){
    let split: Vec<&str> = input.splitn(2," | ").collect();

    let mut parts = split[0].trim().split_whitespace();

    let command = parts.next().unwrap().to_string();
    let args: Vec<String> = parts.map(|s| s.to_string()).collect();

    match split.get(1) {
        Some(list_commands) => (command, args, list_commands.to_string()),
        None => (command, args, String::new())
    }
}

fn execute_command(command_view: CommandView) -> Option<Child>{

    let stdin = command_view.prev_out
        .map_or(Stdio::inherit(),
        |output: Child| Stdio::from(output.stdout.unwrap()));

    let output = Command::new(command_view.command)
        .args(command_view.args)
        .stdin(stdin)
        .stdout(match command_view.list_commands.as_ref() {
            "" => Stdio::inherit(),
            _ => Stdio::piped()
        })
        .spawn();

    match output {
        Ok(output) => { Some(output) },
        Err(e) => { println!("{}", e); None}
    }
}

fn ribosome (command_view: CommandView) {

    let list_commands = command_view.list_commands.clone();

    let output = match command_view.command.as_ref() {
        "" => command_view.prev_out,
        _ => execute_command(command_view)
    };

    match list_commands.as_ref() {
        "" => {
            if let Some(mut final_command) = output {
                final_command.wait().unwrap();
            }
            //output.unwrap().wait().unwrap();
            },
        x => {
            let(command, args, new_list_commands) = split_command(x.to_string());

            ribosome(CommandView {
                list_commands: new_list_commands,
                prev_out: output,
                command: command,
                args: args
            })
        }
    };
}

fn main() {
    loop {
        print!("krabben$ ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let command_view = CommandView {
            list_commands: input.to_string(),
            prev_out: Option::None,
            command: String::new(),
            args: [].to_vec()
        };

        ribosome(command_view);
    }
}

//TESTS OF RIBOSOME


#[test]
fn test_execute_commands(){

    let command_view = CommandView {
        list_commands: "ls -l | more".to_string(),
        prev_out: Option::None,
        command: String::new(),
        args: [].to_vec()
    };


    ribosome(command_view);

    assert!(true);
    println!("MORI");
}
//TESTS OF EXECUTE COMMAND FUNCTION


#[test]
fn test_execute_command(){

    let command_view = CommandView {
        list_commands: String::new(),
        prev_out: Option::None,
        command: "ls".to_string(),
        args: [].to_vec()
    };


    let output = execute_command(command_view);

    assert!(match output {Some(_) => true, None => false});
}

#[test]
fn test_execute_bad_command(){

    let command_view = CommandView {
        list_commands: String::new(),
        prev_out: Option::None,
        command: "sa".to_string(),
        args: [].to_vec()
    };


    let output = execute_command(command_view);

    assert!(match output {Some(_) => false, None => true});
}

#[test]
fn test_execute_empty_command(){

    let command_view = CommandView {
        list_commands: String::new(),
        prev_out: Option::None,
        command: String::new(),
        args: [].to_vec()
    };


    let output = execute_command(command_view);

    assert!(match output {Some(_) => false, None => true});
}

//TESTS OF SPLIT COMMAND FUNCTION

#[test]
fn test_split_command(){

    let (command, args, list_commands) = split_command("ls".to_string());

    assert_eq!(command, "ls");
    assert!(args.is_empty());
    assert!(list_commands.is_empty());
}

#[test]
fn test_split_command_single_argument(){
    let (command, args, list_commands) = split_command("ls -a".to_string());
    assert_eq!(command, "ls");
    assert_eq!(args, ["-a"]);
    assert!(list_commands.is_empty());
}

#[test]
fn test_split_command_multiple_argument(){
    let (command, args, list_commands) = split_command("ls -a -B".to_string());
    assert_eq!(command, "ls");
    assert_eq!(args, ["-a", "-B"]);
    assert!(list_commands.is_empty());
}

#[test]
fn test_split_command_pipe(){
    let (command, args, list_commands) = split_command("ls -a -B | ls -a".to_string());
    assert_eq!(command, "ls");
    assert_eq!(args, ["-a", "-B"]);
    assert_eq!(list_commands, "ls -a");
}

#[test]
fn test_split_command_many_pipe(){
    let (command, args, list_commands) = split_command("ls -a -B | ls -a | ls -B".to_string());
    assert_eq!(command, "ls");
    assert_eq!(args, ["-a", "-B"]);
    assert_eq!(list_commands, "ls -a | ls -B");
}
