use std::error::Error;
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

use retry::{OperationResult, retry};
use retry::delay::Fixed;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "retry", about, version)]
struct CliArgs {
    /// maximum number of script executions before giving up
    #[clap(long)]
    count: Option<u64>,

    /// maximum duration in seconds before giving up
    #[clap(long)]
    duration: Option<u64>,

    /// send system notification on exit
    #[clap(long, takes_value=false)]
    notify: bool,

    /// delay between script executions in seconds
    #[clap(long)]
    delay: u64,

    /// path to the script to retry
    #[clap()]
    script: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: CliArgs = CliArgs::parse();
    let retry_count = args.count.unwrap_or(u64::MAX);
    let retry_duration = args.duration.map(|u| { Duration::from_secs(u) }).unwrap_or(Duration::MAX);
    let notify = args.notify;
    let delay_in_seconds = args.delay;
    let script_path = args.script.into_os_string().into_string().unwrap();

    let started = Instant::now();

    let duration_iterator = Fixed::from_millis(delay_in_seconds * 1000)
        .take(usize::try_from(retry_count)?)
        .take_while(|_| {
            return started.elapsed() <= retry_duration;
        });
    let retry_result = retry(duration_iterator, || {
        let result = Command::new(&script_path)
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
        send_notification(&script_path, retry_result.is_ok())?;
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
