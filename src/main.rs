use indicatif::ProgressBar;
use clap::Parser;
use std::{fs, thread};
use json;
use std::process::{Command};
use std::time::Duration;

#[derive(Parser)]
struct Cli {
    /// Path to the cli programm
    cli_path: String,
    /// Path to the commands to run
    command_path: String,
    /// Time delta between calls
    delta_t: u64
}

fn cli_call(cmd: &String, params: &Vec<String>) -> String {
    let result = Command::new(cmd)
        .args(params)
        .output()
        .expect("failed to execute process");

    return String::from_utf8(result.stdout).unwrap()
}

struct CliCall {
    cmd: String,
    call: Vec<String>,
    result: String
}

impl CliCall {
    fn resolve(&mut self) {
        self.result = cli_call(&self.cmd, &self.call).parse().unwrap();
    }

    fn print_result(&self) {
        println!("result={}", self.result);
    }
}

fn statistics(call_results: &Vec<CliCall>) -> String {
    let mut successful = 0;
    let mut failed = 0;

    for call_result in call_results {
        let dict = json::parse(&call_result.result).unwrap();

        if dict["success"] == true {
            successful += 1
        } else {
            failed += 1
        }
    }

    return format!(
        "Made {} calls\n  successful: {}, failed {} ({:.1}%)",
        call_results.len(),
        successful,
        failed,
        (successful as f32 / (successful + failed) as f32) * 100.0
    )
}

fn log_results(results: &Vec<CliCall>) {
    let content = results.iter().map(|el| {
        el.result.to_string()
    }).collect::<Vec<String>>().join("");

    fs::write(
        "results.txt",
        content
    ).expect("Unable to write file");
}

fn main() {
    let args = Cli::parse();
    let lines = fs::read_to_string(args.command_path)
        .expect("Something went wrong reading the file");
    let commands = lines.split("\n");
    let items: Vec<_> = commands.map(|line| CliCall {
        cmd: args.cli_path.clone(),
        call: line.split(" ").map(|s| s.to_string()).collect(),
        result: "".to_string()
    }).collect();
    let mut tasks = vec![];
    let pb = ProgressBar::new(items.len() as u64);

    for mut item in items {
        let fut = thread::spawn(
            || {
                item.resolve();
                item
            }
        );
        tasks.push(fut);
        thread::sleep(Duration::from_millis(args.delta_t));
        pb.inc(1);
    }

    let results = tasks.into_iter()
        .map(|task| {
            let call_result = task.join().unwrap();
            call_result.print_result();
            call_result
        }).collect();

    pb.finish_with_message("All tasks started, awaiting completion...");
    log_results(&results);
    println!("{}", statistics(&results))
}
