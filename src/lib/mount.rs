#![allow(dead_code)]

pub mod mount {

    use log::LevelFilter;
    use mountpoints::mountpaths;
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use std::process::Child;
    use std::process::Command;
    use std::process::Stdio;
    use std::sync::mpsc;
    use std::thread;

    use crate::lib::app::app_mod::App;
    use crate::lib::{config::config::DriveStruct, utils::utils::*};

    pub fn start_mounting(drive: &DriveStruct, app: &mut App) {
        log_info(format!("Start mounting {}", drive.name));
        let mounted_points: Vec<char> = mountpaths()
            .unwrap()
            .iter()
            .map(|f| {
                let point = f
                    .to_str()
                    .unwrap()
                    .clone()
                    .split(":")
                    .collect::<Vec<_>>()
                    .first()
                    .unwrap()
                    .clone();
                point.to_owned().chars().next().unwrap()
            })
            .collect();
        let alphabet: Vec<char> = ('A'..='Z').into_iter().collect::<Vec<char>>();
        let mut difference: Vec<char> = alphabet
            .clone()
            .into_iter()
            .filter(|&item| !mounted_points.contains(&item))
            .collect::<Vec<char>>();
        difference.shuffle(&mut thread_rng());
        let point = difference.first().unwrap().to_string();
        let (tx, rx) = mpsc::channel::<String>();
        let (tx1, rx1) = mpsc::channel::<Child>();
        tx.send(drive.name.clone()).unwrap();
        thread::spawn(move || {
            let child = Command::new("rclone")
                .args(&[
                    String::from("mount"),
                    format!("{}:", rx.recv().unwrap()),
                    format!("{}:", point),
                    String::from("--vfs-cache-mode"),
                    String::from("full"),
                ])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn();
            match child {
                Ok(child) => {
                    // for line in BufReader::new(child.stdout.as_mut().unwrap()).lines() {
                    //     let line = (&line.unwrap()[20..]).to_string();
                    //     log_info(format!("{}", line));
                    // }
                    // for line in BufReader::new(child.stderr.as_mut().unwrap()).lines() {
                    //     let line = (&line.unwrap()[20..]).to_string();
                    //     log_error(format!("{}", line));
                    // }
                    tx1.send(child).unwrap();
                }
                Err(e) => log_error(e.to_string()),
            }
            // match rx.try_recv() {
            //     Ok(_) | Err(TryRecvError::Disconnected) => {
            //         println!("Terminating.");
            //         return;
            //     }
            //     Err(TryRecvError::Empty) => {
            //         log_info("msg".to_owned());
            //     }
            // }
        });
        app.process_mount.push(rx1.recv().unwrap());
    }

    pub fn stop_mounting(drive: &DriveStruct, process: &mut Child) {
        log_warning(format!(
            "Stop mounting {}\nSave your stuff {}",
            drive.name,
            get_levelfilter_emoji(LevelFilter::Warn)
        ));
        process.kill().expect("command wasn't running");
    }
}
