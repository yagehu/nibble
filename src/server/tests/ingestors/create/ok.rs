use serde_json::json;

use test_app::TestApp;

#[tokio::test]
async fn main() {
    let app = TestApp::spawn().await;
    let name = "Github/main";
    let image_name = "nibble-ingestor-github";
    let image_tag = "latest";
    let ingestor = app
        .ingestors()
        .create_ok(json!({
            "name": name,
            "image_name": image_name,
            "image_tag": image_tag,
        }))
        .await;

    assert_eq!(ingestor.name, name);
    assert_eq!(ingestor.image_name, image_name);
    assert_eq!(ingestor.image_tag, image_tag);
}
