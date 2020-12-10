use std::{collections::HashMap, io::prelude::*, io::BufReader};

use md5::{Digest, Md5};

use crate::{
    common::{FilePodSpec, PodSpec, PodSpecGroup},
    error::{PodError, Result},
};

/// 计算名字对应的pod index索引
/// - MD5(name)[0..3]
pub fn name_hash(name: &str) -> Result<String> {
    // let ds: Vec<u8> = name.bytes().collect();
    let res = Md5::digest(name.as_bytes());
    // let l = res.len();
    if res.len() != 16 {
        return Err(PodError::MD5);
    }
    let hex = format!("{:02x}{:02x}", res[0], res[1]);

    return Ok(hex[0..3].to_string());
}

/// str join sep
pub fn str_join(string: &str, sep: &str) -> String {
    string
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}

/// 解析podfile.lock的podspec
pub fn parse_podfile_lock_podspec<T: Read>(
    buf: BufReader<T>,
) -> Result<HashMap<String, PodSpecGroup>> {
    let lines = buf
        .lines()
        .skip_while(|line| line.as_ref().unwrap() != "PODS:");

    // parse pod spec
    log::info!("parse file start");
    let mut fps_map: HashMap<String, PodSpecGroup> = HashMap::new();
    for line in lines {
        let line = line?;
        if line.is_empty() {
            log::debug!("line is empty, end parse");
            break; // pods end
        } else if !line.starts_with("  - ") {
            log::debug!("line is not a root spec, skip: {}", line);
            continue; // dep pod spec, skip
        }
        // - `  - NAME (VER):`
        // - `  - NAME (VER)`
        // - `  - "NAME (VER)"`
        let line_tm: &[_] = &[' ', '-', '"', ':'];
        let ls = line.trim_matches(line_tm).split(" ").collect::<Vec<&str>>();
        if ls.len() != 2 {
            log::warn!("parse line failed: {}", line);
            continue;
        }

        // NAME/SPEC
        let name = ls[0];
        let (name, sub_spec) = if let Some(idx) = name.find("/") {
            (&name[0..idx], Some(name[idx + 1..].to_string()))
        } else {
            (name, None)
        };

        // 检查是否有相同的pod spec
        if let Some(pg) = fps_map.get_mut(name) {
            log::debug!("podspec did add {}", name);
            // 是否为pod sub spec
            if let Some(sub_spec) = sub_spec {
                match pg.0.sub_spec.as_mut() {
                    Some(ss) => ss.push(sub_spec),
                    None => pg.0.sub_spec = Some(vec![sub_spec]),
                }
            }
            continue;
        }

        let ver_tm: &[_] = &['(', ')'];
        let ver = ls[1].trim_matches(ver_tm);

        log::debug!("podspec add {} {}", name, ver);
        let fps = FilePodSpec::new(name.to_string(), ver.to_string(), None);
        let ps = PodSpec::new(name.to_string())?;
        fps_map.insert(name.to_string(), PodSpecGroup(fps, ps));
    }

    log::info!("parse file end {}", fps_map.len());
    Ok(fps_map)
}
