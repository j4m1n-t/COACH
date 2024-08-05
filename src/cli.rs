use clap::{Command, Arg};

pub struct CliArgs {
    pub target: Option<String>,
    pub port: Option<u16>,
    pub target_file: Option<String>,
    pub config_file: String,
    pub protocol: String,
}

pub fn parse_args() -> CliArgs {
    let matches = Command::new("Coach")
        .arg(Arg::new("port").short('p').long("port"))
        .arg(Arg::new("target").short('t').long("target"))
        .arg(Arg::new("config").short('c').long("config").required(true))
        .arg(Arg::new("protocol").long("protocol").default_value("auto"))
        .get_matches();

         let target = matches.get_one::<String>("target").cloned();
        let port = matches.get_one::<String>("port").and_then(|p| p.parse().ok());
        let target_file = matches.get_one::<String>("target").cloned();
        let config_file = matches.get_one::<String>("config").unwrap().clone();
        let mut protocol = matches.get_one::<String>("protocol").unwrap().clone();

    // Infer protocol if set to "auto"
    if protocol == "auto" {
        protocol = match port {
            Some(22) => "ssh",
            Some(80) => "http",
            Some(443) => "https",
            Some(5985) | Some(5986) => "winrm",
            _ => "tcp",
        }.to_string();
    }

    CliArgs {
        target,
        port,
        target_file,
        config_file,
        protocol,
    }
}