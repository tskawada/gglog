use clap::{App, Arg};
use std::process::Command;

fn main() {
    let latest: Arg = Arg::new("latest")
        .long("latest")
        .help("display a log file which updated latest");

    let list: Arg = Arg::new("list")
        .long("list")
        .help("display a list of log files");

    let app: App = App::new("gglog")
        .author("tskawada")
        .version("v0.0.1")
        .about("gglog is a tool for viewing AWS Greengrass log")
        .arg(latest)
        .arg(list);
    
    match app.try_get_matches() {
        Ok(matches) => {
            if matches.is_present("latest") {
                let mut _process = Command::new("bash")
                    .arg("-c")
                    .current_dir("/greengrass/v2/logs")
                    .arg("tail -f $(ls -rt *.log | tail -n 1)")
                    .spawn()
                    .expect("failed to run");
                _process.wait().expect("failed to wait");
            }
            else if matches.is_present("list") {
                let mut _process = Command::new("bash")
                    .arg("-c")
                    .current_dir("/greengrass/v2/logs")
                    .arg("ls -alh *.log")
                    .spawn()
                    .expect("failed to run");
                _process.wait().expect("failed to wait");
            }
        },
        Err(e) => eprintln!("{}", e),
    }
}