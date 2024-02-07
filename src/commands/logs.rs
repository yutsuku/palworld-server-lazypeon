use serenity::all::{Context, CreateCommand};
use std::process::{Command, Stdio};

pub async fn run(_ctx: &Context, current_dir: &str) -> String {
    let output = Command::new("docker")
        .arg("compose")
        .arg("logs")
        .current_dir(current_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        let answer = String::from_utf8 (output.stdout).unwrap();

        if answer.trim().is_empty() {
            return "No response from the command (stdout)".to_string();
        }

        let answer_limited = {
            let split_pos = answer.char_indices().nth_back(1024).unwrap().0;
            &answer[split_pos..]
        };

        return answer_limited.to_string();
    } else {
        let answer = String::from_utf8 (output.stderr).unwrap();

        if answer.trim().is_empty() {
            return "No response from the command (stderr)".to_string();
        }

        return answer;
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("logs").description("Displays game server most recent logs. Contains sensitive data")
}