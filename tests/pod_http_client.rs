use pod_helper::{pod_http_client::PodHTTPClient, utils};

#[test]
fn test_str_join() {
    assert_eq!(utils::str_join("123", ","), "1,2,3")
}

#[tokio::test]
async fn test_fetch_idx_pods() {
    let name = "AppNetworkManager";
    let hex = utils::name_hash(name).unwrap();
    let content = PodHTTPClient::default().fetch_idx_pods(&hex).await.unwrap();

    assert!(content.contains(name));
}
