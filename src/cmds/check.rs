use std::{cell::RefCell, collections::HashMap, fs::File, io::BufReader, rc::Rc};

use comfy_table::Table;
use futures::future::select_all;
use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    common::{PodSpecGroup, PodVer},
    error::{PodError, Result},
    opts::pod_helper::CheckOpts,
    pod_http_client::PodHTTPClient,
    utils::parse_podfile_lock_podspec,
};

/// 输出podfile的更新信息
fn output_podfile_update_info(
    opt: &CheckOpts,
    pg_map: &HashMap<String, Rc<RefCell<PodSpecGroup>>>,
) {
    let (not_vers, has_vers): (Vec<_>, Vec<_>) =
        pg_map.values().map(|pg| pg.borrow()).partition(|pg| {
            if let Some(vers) = pg.1.get_vers() {
                vers.is_empty()
            } else {
                true
            }
        });
    let (has_new_vers, has_vers): (Vec<_>, Vec<_>) = has_vers
        .iter()
        .partition(|pg| &pg.0.ver < pg.1.get_vers().as_ref().unwrap().first().unwrap());

    let mut tb = Table::new();
    tb.set_header(vec!["name", "ver", "new ver"]);
    not_vers.iter().for_each(|pg| {
        let s = &pg.0.ver.to_string() as &str;
        let v: Vec<&str> = vec![&pg.0.name, s, "not found or request failed"];
        tb.add_row(v);
    });
    if opt.show_no_update {
        has_vers.iter().for_each(|pg| {
            let s = &pg.0.ver.to_string() as &str;
            let v: Vec<&str> = vec![&pg.0.name, s, "-"];
            tb.add_row(v);
        });
    }
    has_new_vers.iter().for_each(|pg| {
        let s = &pg.0.ver.to_string() as &str;
        let ns = &pg
            .1
            .get_vers()
            .as_ref()
            .unwrap()
            .first()
            .unwrap()
            .to_string() as &str;
        let v: Vec<&str> = vec![&pg.0.name, s, ns];
        tb.add_row(v);
    });

    println!("{}", tb);
}

/// 从podfile.lock获取podspec检查更新
pub async fn check_file(opt: CheckOpts) -> Result<()> {
    let f = File::open(&opt.file_path)?;
    let pg_map = parse_podfile_lock_podspec(BufReader::new(f))?;
    if pg_map.is_empty() {
        return Err(PodError::Other(
            "parse file podfile.lock can not find any podspec",
        ));
    }

    // name: podspec group
    let pg_map: HashMap<_, _> = pg_map
        .into_iter()
        .map(|pg| (pg.0, Rc::new(RefCell::new(pg.1))))
        .collect();

    // name_hex: pg count
    let mut hex_map: HashMap<String, u32> = HashMap::new();
    for pg in pg_map.values() {
        let pg = pg.borrow();
        match hex_map.get_mut(&pg.1.hex) {
            Some(c) => *c += 1,
            None => {
                hex_map.insert(pg.1.hex.clone(), 1);
            }
        }
    }

    // make async request
    let pod_client = PodHTTPClient::new(
        opt.request_concurrent,
        opt.connect_timeout,
        opt.request_timeout,
    );
    let mut fs: Vec<_> = hex_map
        .keys()
        .map(|hex| &hex as &str)
        .map(|hex| pod_client.fetch_idx_pods_(hex))
        .map(Box::pin)
        .collect();

    log::debug!("start request podspec idx");
    let pb = create_pb(fs.len() as u64);
    while !fs.is_empty() {
        let ((hex, res), _idx, remaining) = select_all(fs).await;
        pb.inc(1);

        match res {
            Ok(val) => {
                let mut counter = hex_map.get(hex).unwrap().clone();
                // NAME/VERS...
                for line in val.split("\n") {
                    if line.is_empty() {
                        continue; // skip empty line
                    }

                    let mut ds = line.split("/");
                    if let Some(name) = ds.next() {
                        // is contains pod name
                        match pg_map.get(name) {
                            Some(pg) => {
                                pg.borrow_mut()
                                    .1
                                    .set_vers(Some(ds.map(PodVer::new).collect()));

                                counter -= 1;
                                if counter == 0 {
                                    break;
                                }
                            }
                            None => {}
                        }
                    } else {
                        log::warn!("parse podspec idx line failed: {}", line);
                    }
                }
            }
            Err(err) => {
                log::warn!("fetch podspec idx failed {}: {:?}", hex, err);
            }
        }
        fs = remaining;
    }
    log::debug!("end request podspec idx");
    // pb.finish_print("request done.");
    pb.finish_with_message("request done.");
    drop(pb);

    // output
    output_podfile_update_info(&opt, &pg_map);

    Ok(())
}

/// 创建进度条显示
fn create_pb(count: u64) -> ProgressBar {
    let pb = ProgressBar::new(count);
    let style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {spinner} {bar:50} {pos}/{len} {percent}%")
        .progress_chars("▌▌░");
    pb.set_style(style);

    pb
}

// 查找podspec idx
// async fn fetch_podspec_idx(name: String) -> Result<()> {
//     let ph = PodHTTPClient::default();
//     let pod = PodSpec::new(name).expect("md5 name failed");

//     let content = ph.fetch_idx_pods(&pod.hex).await?;
//     let match_str = format!("{}/", pod.name);
//     if let Some(line) = content
//         .split("\n")
//         .find(|&line| line.starts_with(&match_str))
//     {
//         let mut ds = line.split("/");
//         let _ = ds.next(); // drop name
//         log::info!("Found: {:#?}", pod);
//     } else {
//         log::info!("Not Found");
//     }

//     Ok(())
// }
