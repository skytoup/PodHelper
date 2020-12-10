use pod_helper::common::PodVer;

#[test]
fn test_pod_ver_0() {
    assert_eq!([0, 0, 0], PodVer::new("").nums)
}

#[test]
fn test_pod_ver_1() {
    assert_eq!([1, 0, 0], PodVer::new("1").nums)
}

#[test]
fn test_pod_ver_2() {
    assert_eq!([1, 6, 0], PodVer::new("1.6").nums)
}

#[test]
fn test_pod_ver_3() {
    assert_eq!([1, 0, 0], PodVer::new("1.0.0").nums)
}

#[test]
fn test_pod_ver_xxx() {
    assert_eq!([1, 0, 0], PodVer::new("1.0.beta_3").nums)
}
