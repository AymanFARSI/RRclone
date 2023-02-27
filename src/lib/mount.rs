#![allow(dead_code)]

pub mod mount {

    use log::LevelFilter;
    use mountpoints::mountpaths;
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use std::env;
    use std::fs;
    use std::process::Child;
    use std::process::Command;
    use std::process::Stdio;
    use std::sync::mpsc;
    use std::thread;

    use crate::lib::app::app_mod::App;
    use crate::lib::{config::config::DriveStruct, utils::utils::*};

    pub fn start_mounting(drive: &DriveStruct, app: &mut App) {
        log_info(format!("Start mounting {}", drive.name));
        let point: String = match env::consts::OS {
            "windows" => {
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
                difference.first().unwrap().to_string()
            }
            _ => {
                let mut path = String::from("/Users/evildave");
                let name = format!("/{}", drive.name.clone());
                path.push_str(&name);
                log_debug(path.clone());
                match fs::create_dir(&path) {
                    Ok(_) => {
                        log_info(format!("Already existing directory on {}", path));
                    }
                    Err(_) => {
                        // fs::create_dir(&path).expect("Could not create directory");
                        log_info(format!("Created empty directory for drive on {}", path));
                    }
                }
                path
            }
        };

        let (tx, rx) = mpsc::channel::<String>();
        let (tx1, rx1) = mpsc::channel::<Child>();
        tx.send(drive.name.clone()).unwrap();
        thread::spawn(move || {
            let child = Command::new("rclone")
                .args(&[
                    String::from("mount"),
                    format!("{}:", rx.recv().unwrap()),
                    format!("{}", point),
                    String::from("--vfs-cache-mode"),
                    String::from("full"),
                    String::from("--allow-other"),
                    String::from("--vfs-read-chunk-size"),
                    String::from("32M"),
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
        app.processes_mounted.push(rx1.recv().unwrap());
    }

    pub fn stop_mounting(drive: &DriveStruct, process: &mut Child) {
        log_warning(format!(
            "Stop mounting {}\nSave your stuff {}",
            drive.name,
            get_levelfilter_emoji(LevelFilter::Warn)
        ));
        match env::consts::OS {
            "windows" => process.kill().expect("command wasn't running"),
            "linux" => {
                match Command::new("fusermount")
                    .args(&[String::from("-uz"), format!("{}", drive.name)])
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                {
                    Ok(_) => log_debug("Unmounted successfully".to_owned()),
                    Err(e) => log_error(e.to_string()),
                }
            }
            "macos" => {
                match Command::new("diskutil")
                    .args(&[
                        String::from("unmount"),
                        format!("/Users/evildave/{}", drive.name),
                    ])
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                {
                    Ok(_) => log_debug("Unmounted successfully".to_owned()),
                    Err(e) => log_error(e.to_string()),
                }
            }
            _ => panic!("Platform not supported"),
        };
        log_debug(format!("Killed process ID: {}", process.id()));
    }
}
