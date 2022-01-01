use std::error::Error;
use std::process::Command;
use std::time::{Duration, Instant};

use retry::{OperationResult, retry_with_index};
use retry::delay::Fixed;

use arguments::{arguments, Arguments};
use retry_extension::FixedExt;
use user_interface::UserInterface;

mod arguments;
mod retry_extension;
mod user_interface;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = arguments();
    let retry_count = arguments.count.unwrap_or(u64::MAX);
    let retry_duration = arguments.duration_in_seconds.map(|u| { Duration::from_secs(u) }).unwrap_or(Duration::MAX);
    let script_path = arguments.script.clone().into_os_string().into_string().unwrap();

    let ui = user_interface::new_indicative(&arguments);
    ui.display();

    let started = Instant::now();

    let duration_iterator = Fixed::from_seconds(arguments.delay_in_seconds)
        .take(usize::try_from(retry_count - 1)?)
        .take_while(|_| {
            return started.elapsed() <= retry_duration;
        });
    let retry_result = retry_with_index(duration_iterator, |index| {
        ui.starting_retry(index);
        let result = Command::new(&script_path)
            .output();
        let script_output = match result {
            Err(_) => return OperationResult::Err("unable to execute script"),
            Ok(output) => output,
        };
        ui.retry_done();
        match script_output.status.code() {
            Some(code) if { code == 0 } => OperationResult::Ok(code),
            Some(_) => OperationResult::Retry("different code"),
            None => OperationResult::Err("broken"),
        }
    });

    if arguments.notify {
        ui.send_notification(&script_path, retry_result.is_ok())?;
    }

    ui.close();
    Ok(())
}
