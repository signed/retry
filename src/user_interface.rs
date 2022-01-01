use std::error::Error;
use std::process::Command;
use indicatif::{FormattedDuration, ProgressBar, ProgressStyle};
use std::time::Duration;
use std::ops::Add;
use crate::Arguments;

pub trait UserInterface {
    fn display(&self);
    fn starting_retry(&self, retry_count: u64);
    fn retry_done(&self);
    fn close(&self);
    fn send_notification(&self, script_name: &str, script_success: bool) -> Result<(), Box<dyn Error>>;
}

pub fn new_indicative(arguments: &Arguments) -> Indicative {
    Indicative { arguments: &arguments, spinner: ProgressBar::new_spinner() }
}

pub struct Indicative<'a> {
    pub arguments: &'a Arguments,
    pub spinner: ProgressBar,
}

impl UserInterface for Indicative<'_> {
    fn display(&self) {
        match self.arguments.count {
            Some(count) => self.spinner.set_length(count),
            _ => ()
        }
        self.spinner.set_position(0);
        self.spinner.enable_steady_tick(120);
        let position_details = format!("{pos}/{len}",
                                       pos = "{pos}",
                                       len = match self.arguments.count {
                                           Some(_) => "{len}",
                                           None => "?"
                                       }
        );
        let spinner_part = "{spinner:.blue}";
        let message = "{wide_msg}";
        let prefix = "{prefix}";
        let elapsed = "{elapsed_precise}";

        let template = format!("{position} {spinner} {message} {prefix}[{elapsed}]",
                               position = position_details,
                               spinner = spinner_part,
                               message = message,
                               prefix = prefix,
                               elapsed = elapsed
        );

        self.spinner.set_style(
            ProgressStyle::default_spinner()
                // For more spinners check out the cli-spinners project:
                // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
                .tick_strings(&[
                    "▹▹▹▹▹",
                    "▸▹▹▹▹",
                    "▹▸▹▹▹",
                    "▹▹▸▹▹",
                    "▹▹▹▸▹",
                    "▹▹▹▹▸",
                    "▪▪▪▪▪",
                ])
                .template(&template),
        );

        self.spinner.set_message("Calculating...");
        self.spinner.set_message("Next...");


        if self.arguments.notify {
            self.spinner.set_prefix("📩");
        }
    }

    fn starting_retry(&self, retry_count: u64) {
        self.spinner.set_position(retry_count);
        let script_path = self.arguments.script.clone().into_os_string().into_string().unwrap();
        self.spinner.set_message(format!("Executing {}", script_path));
    }

    fn retry_done(&self) {
        let next = self.spinner.elapsed().add(Duration::from_secs(self.arguments.delay_in_seconds));
        self.spinner.set_message(format!("next execution {}", FormattedDuration(next)));
    }

    fn close(&self) {
        self.spinner.abandon_with_message("done...");
    }

    fn send_notification(&self, script_name: &str, script_success: bool) -> Result<(), Box<dyn Error>> {
        let emoji = match script_success {
            true => "✅",
            false => "❌"
        };
        let title = format!("{emoji} retry {script_name}", emoji = emoji, script_name = script_name);
        let display_script = format!("display notification \"{message}\" with title \"{title}\" subtitle \"{subtitle}\"", title = "", subtitle = "", message = title);
        Command::new("osascript")
            .arg("-e")
            .arg(display_script)
            .output()?;
        return Ok(());
    }
}
