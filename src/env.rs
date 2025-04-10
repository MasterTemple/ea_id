use std::{collections::HashMap, fs::OpenOptions, io::BufWriter, path::Path};

use std::io::Write;

use dotenvy::{dotenv, from_path_iter};

use crate::origin::OriginApi;

pub fn load_env() {
    // try reading but if it is not there, its ok
    _ = dotenv();
}

pub fn update_env(api: &OriginApi) {
    let path = Path::new(".env");
    let mut vars: HashMap<String, String> = HashMap::new();

    if path.exists() {
        for item in from_path_iter(path).unwrap().flatten() {
            vars.insert(item.0, item.1);
        }
    }

    vars.insert("REMID".to_string(), api.remid.clone());
    vars.insert("SID".to_string(), api.sid.clone());
    vars.insert("ACCESS_TOKEN".to_string(), api.auth_token.clone());

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .unwrap();
    let mut writer = BufWriter::new(file);
    for (k, v) in vars {
        writeln!(writer, "{}={}", k, v).unwrap();
    }
}
