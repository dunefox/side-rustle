use indicatif::ProgressBar;
use clap::Parser;
use std::fs;
use json;
use std::process::{Command};
use std::time::Duration;
use tokio;

#[derive(Parser)]
struct Cli {
    /// Path to the commands to run
    path: String,
    /// Time delta between calls
    delta_t: u64
}

async fn cli_call(params: &Vec<String>) -> String {
    let result = Command::new("./cli.py")
        .args(params)
        .output()
        .expect("failed to execute process");

    return String::from_utf8(result.stdout).unwrap()
}

struct CliCall {
    call: Vec<String>,
    result: String
}

impl CliCall {
    async fn resolve(&mut self) {
        self.result = cli_call(&self.call).await.parse().unwrap();
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

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let lines = fs::read_to_string(args.path)
        .expect("Something went wrong reading the file");
    let commands = lines.split("\n");
    let items: Vec<_> = commands.map(|line| CliCall {
        call: line.split(" ").map(|s| s.to_string()).collect(),
        result: "".to_string()
    }).collect();
    let tasks: Vec<_> = items
        .into_iter()
        .map(|mut item| {
            tokio::spawn(async {
                item.resolve().await;
                item
            })
        })
        .collect();
    let mut results = vec![];
    let pb = ProgressBar::new(tasks.len() as u64);

    for task in tasks {
        results.push(task.await.unwrap());
        tokio::time::sleep(Duration::from_millis(args.delta_t)).await;
        pb.inc(1);
    }

    pb.finish_with_message("done");

    for item in results.iter() {
        item.print_result();
    }

    println!("{}", statistics(&results))
}
