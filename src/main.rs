use std::error::Error;
use std::process::Command;
use std::time::{Duration, Instant};

use retry::{OperationResult, retry};
use retry::delay::Fixed;

fn main() -> Result<(), Box<dyn Error>> {
    let script_name = "./scripts/fail.sh";
    let delay_in_seconds = 1;

    let notify = Some(true).unwrap_or(false);
    let retry_count = Some(10).unwrap_or(usize::MAX);
    let retry_duration = Some(Duration::from_secs(1) * 5).unwrap_or(Duration::MAX);
    let started = Instant::now();

    let duration_iterator = Fixed::from_millis(delay_in_seconds * 1000)
        .take(retry_count)
        .take_while(|_| {
            return started.elapsed() <= retry_duration;
        });
    let retry_result = retry(duration_iterator, || {
        let result = Command::new(script_name)
            .output();
        let output = match result {
            Err(_) => return OperationResult::Err("unable to execute script"),
            Ok(output) => output,
        };

        match output.status.code() {
            Some(code) if { code == 0 } => OperationResult::Ok(code),
            Some(_) => OperationResult::Retry("different code"),
            None => OperationResult::Err("broken"),
        }
    });

    if notify {
        send_notification(script_name, retry_result.is_ok())?;
    }
    Ok(())
}

fn send_notification(script_name: &str, retry_ok: bool) -> Result<(), Box<dyn Error>> {
    let emoji = match retry_ok {
        true => "✅",
        false => "❌"
    };
    let title = format!("{emoji} retry {script_name}", emoji = emoji, script_name = script_name);
    let display_script = format!("display notification \"{message}\" with title \"{title}\" subtitle \"{subtitle}\"", title = "", subtitle = "", message = title);
    Command::new("osascript")
        .arg("-e")
        .arg(display_script)
        .output()?;
    return Ok(())
}
