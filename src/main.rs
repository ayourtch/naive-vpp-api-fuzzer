use clap::Clap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::time::{Duration, SystemTime};
use vpp_api_transport::*;

/// This is a simple user of vpp-api-transport at low level,
/// it attempts to send various junk at it
/// at varying levels of sophistication
#[derive(Debug, Clone, Clap, Serialize, Deserialize)]
#[clap(version = env!("GIT_VERSION"), author = "Andrew Yourtchenko <ayourtch@gmail.com>")]
struct Opts {
    /// Unix socket to connect to VPP on
    #[clap(short, long)]
    socket_path: Option<String>,

    /// Override options from this yaml/json file
    #[clap(short, long)]
    options_override: Option<String>,

    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
}

use vpp_api_transport::*;

fn test_transport(t: &mut vpp_api_transport::VppApiTransport) {
    println!("Connect result: {}", t.connect("api-test", None, 32));
    println!("ping 1");
    let mut buf = [0; 256];
    for i in 0..42 {
        println!("Random iteration #{}", i);
        for j in 0..buf.len() {
            buf[1] = i;
        }
        t.write(&buf);
    }
    std::thread::sleep(std::time::Duration::from_secs(1));
    t.dump();
}

fn main() {
    let opts: Opts = Opts::parse();

    // allow to load the options, so far there is no good built-in way
    let opts = if let Some(fname) = &opts.options_override {
        if let Ok(data) = std::fs::read_to_string(&fname) {
            let res = serde_json::from_str(&data);
            if res.is_ok() {
                res.unwrap()
            } else {
                serde_yaml::from_str(&data).unwrap()
            }
        } else {
            opts
        }
    } else {
        opts
    };

    if opts.verbose > 4 {
        let data = serde_json::to_string_pretty(&opts).unwrap();
        println!("{}", data);
        println!("===========");
        let data = serde_yaml::to_string(&opts).unwrap();
        println!("{}", data);
    }

    let mut vppt: Box<dyn VppApiTransport> = if let Some(afunix_path) = &opts.socket_path {
        Box::new(afunix::Transport::new(&afunix_path))
    } else {
        Box::new(shmem::Transport::new())
    };
    test_transport(&mut *vppt);
    std::thread::sleep(std::time::Duration::from_secs(60));
    vppt.disconnect();
}
