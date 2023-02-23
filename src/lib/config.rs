pub mod config {
    use std::env;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use serde_json::Value;

    #[derive(Debug, Clone)]
    pub struct ConfigStruct {
        pub path: String,
        pub drives: Vec<DriveStruct>,
    }

    #[derive(Debug, Clone)]
    pub struct DriveStruct {
        pub name: String,
        pub drive_type: String,
        pub scope: String,
        pub token: TokenStruct,
    }

    #[derive(Debug, Clone)]
    pub struct TokenStruct {
        pub access_token: String,
        pub token_type: String,
        pub refresh_token: String,
        pub expiry: String,
    }

    pub fn read_config() -> ConfigStruct {
        let rclone_path;
        match env::consts::OS {
            "windows" => {
                let mut path = std::env::var("APPDATA").expect("Can not get AppData folder");
                path.push_str("/rclone/rclone.conf");
                rclone_path = path;
            }
            _ => {
                let mut path = std::env::var("HOME").expect("Can not get HOME directory");
                path.push_str("/rclone/rclone.conf");
                rclone_path = path;
            }
        }

        let file = File::open(&rclone_path).unwrap();
        let buffered = BufReader::new(file);

        let mut drive_name = String::new();
        let mut drive_type = String::new();
        let mut drive_scope = String::new();
        let mut drive_token: Option<TokenStruct> = None;

        let mut drives: Vec<DriveStruct> = Vec::new();

        for line in buffered.lines() {
            let line = line.unwrap();
            if line == "" {
                drives.push(DriveStruct {
                    name: drive_name.clone(),
                    drive_type: drive_type.clone(),
                    scope: drive_scope.clone(),
                    token: match drive_token {
                        Some(ref val) => val.clone(),
                        None => panic!("didnt get token for driver"),
                    },
                });
            } else if line.starts_with('[') {
                drive_name = line.get(1..line.len() - 1).unwrap().to_owned();
            } else {
                match line.get(..2) {
                    Some("ty") => {
                        drive_type = line.split('=').last().unwrap().replace(" ", "").to_owned()
                    }
                    Some("sc") => {
                        drive_scope = line.split('=').last().unwrap().replace(" ", "").to_owned()
                    }
                    Some("to") => {
                        let input = line.split('=').last().unwrap();
                        let json: Value =
                            serde_json::from_str(&input).expect("couldnt parse token json");
                        drive_token = Some(TokenStruct {
                            access_token: json["access_token"].to_string().replace("\"", ""),
                            token_type: json["token_type"].to_string().replace("\"", ""),
                            refresh_token: json["refresh_token"].to_string().replace("\"", ""),
                            expiry: json["expiry"].to_string().replace("\"", ""),
                        });
                    }
                    _ => continue,
                }
            }
        }
        return ConfigStruct {
            path: rclone_path,
            drives,
        };
    }

    // pub fn write() {}
}
