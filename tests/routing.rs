use axum_test::TestServer;
use warung_service_account_api::app::router;

#[tokio::test]
async fn root_works() {
    let app = router();

    let server = TestServer::new(app).unwrap();

    let response = server.get("/").await;

    response.assert_status_ok();
    response.assert_text("Lorem ipsum dolor sit amet");
}
