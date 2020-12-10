use std::io::BufReader;

use pod_helper::utils::{name_hash, parse_podfile_lock_podspec, str_join};

#[test]
fn test_name_hash() {
    assert_eq!("222", name_hash("AppNetworkManager").unwrap());
}

#[test]
fn test_str_join() {
    assert_eq!("1_2_3", str_join("123", "_"));
}

#[test]
fn test_parse_podfile_lock_podspec() {
    let podfile_lock = r#"
PODS:
  - Alamofire (5.3.0)
  - Kingfisher (5.15.6):
  - Kingfisher/Core (= 5.15.6)
  - Kingfisher/Core (5.15.6)
  - "UITableView+FDTemplateLayoutCell (1.6)":
  - "UITableView+FDTemplateLayoutCell/Core" (=1.6)

DEPENDENCIES:
  - Alamofire
  - Kingfisher
  - "UITableView+FDTemplateLayoutCell"

SPEC REPOS:
  trunk:
    - Alamofire
    - Kingfisher
    - "UITableView+FDTemplateLayoutCell"

SPEC CHECKSUMS:
  Alamofire: 2c792affbdc2f18016e08fdbcacd60aebe1ba593
  Kingfisher: b3554e7bf6106115b44e8795300bad580ef2fdc7
  "UITableView+FDTemplateLayoutCell": 5c949b4a5059c404b442926c0e80f81d10a2d66f

PODFILE CHECKSUM: a22a05f6bb3b016d9eaa60819db5e0998dbbc7cb

COCOAPODS: 1.10.0
    "#;

    let pg_map = parse_podfile_lock_podspec(BufReader::new(podfile_lock.as_bytes())).unwrap();
    assert_eq!(
        vec!([5, 3, 0], [5, 15, 6], [1, 6, 0]),
        [
            "Alamofire",
            "Kingfisher",
            "UITableView+FDTemplateLayoutCell"
        ]
        .iter()
        .map(|&name| pg_map.get(name).unwrap().0.ver.nums)
        .collect::<Vec<_>>(),
    );
}
