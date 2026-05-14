use actix_web::{
    http::{header, StatusCode},
    test, web, App,
};
use serde_json::{json, Value};

use actix_web_microservice_boilerplate::{
    app::{configure, json_config},
    common::request_id::RequestId,
    config::Settings,
    state::AppState,
};

fn state() -> web::Data<AppState> {
    web::Data::new(AppState::new(Settings::for_tests()))
}

#[actix_web::test]
async fn get_root_returns_service_metadata() {
    let app = test::init_service(
        App::new()
            .app_data(state())
            .app_data(json_config())
            .wrap(RequestId)
            .configure(configure),
    )
    .await;
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["ok"], true);
    assert_eq!(body["data"]["api"]["base_path"], "/api/v1");
}

#[actix_web::test]
async fn health_returns_healthy() {
    let app = test::init_service(
        App::new()
            .app_data(state())
            .app_data(json_config())
            .wrap(RequestId)
            .configure(configure),
    )
    .await;
    let req = test::TestRequest::get().uri("/api/v1/health").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["ok"], true);
    assert_eq!(body["data"]["status"], "healthy");
}

#[actix_web::test]
async fn readiness_returns_ready() {
    let app = test::init_service(
        App::new()
            .app_data(state())
            .app_data(json_config())
            .wrap(RequestId)
            .configure(configure),
    )
    .await;
    let req = test::TestRequest::get().uri("/api/v1/ready").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["ok"], true);
    assert_eq!(body["data"]["status"], "ready");
}

#[actix_web::test]
async fn create_item_returns_created_item() {
    let app = test::init_service(
        App::new()
            .app_data(state())
            .app_data(json_config())
            .wrap(RequestId)
            .configure(configure),
    )
    .await;
    let req = test::TestRequest::post()
        .uri("/api/v1/items")
        .set_json(json!({"name":"Keyboard","description":"Mechanical keyboard","price":99.99}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["ok"], true);
    assert_eq!(body["data"]["name"], "Keyboard");
    assert_eq!(body["data"]["price"], 99.99);
}

#[actix_web::test]
async fn list_items_returns_items() {
    let app = test::init_service(
        App::new()
            .app_data(state())
            .app_data(json_config())
            .wrap(RequestId)
            .configure(configure),
    )
    .await;
    let create_req = test::TestRequest::post()
        .uri("/api/v1/items")
        .set_json(json!({"name":"Coffee","description":"Cold brew","price":5.5}))
        .to_request();
    let create_resp = test::call_service(&app, create_req).await;
    assert_eq!(create_resp.status(), StatusCode::CREATED);

    let list_req = test::TestRequest::get().uri("/api/v1/items").to_request();
    let list_resp = test::call_service(&app, list_req).await;
    assert_eq!(list_resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(list_resp).await;
    assert_eq!(body["ok"], true);
    assert_eq!(body["data"]["count"], 1);
}

#[actix_web::test]
async fn get_item_returns_created_item() {
    let app = test::init_service(
        App::new()
            .app_data(state())
            .app_data(json_config())
            .wrap(RequestId)
            .configure(configure),
    )
    .await;
    let create_req = test::TestRequest::post()
        .uri("/api/v1/items")
        .set_json(json!({"name":"Mouse","description":"Wireless mouse","price":29.99}))
        .to_request();
    let create_resp = test::call_service(&app, create_req).await;
    let create_body: Value = test::read_body_json(create_resp).await;
    let id = create_body["data"]["id"].as_str().expect("id should exist");

    let get_req = test::TestRequest::get()
        .uri(&format!("/api/v1/items/{id}"))
        .to_request();
    let get_resp = test::call_service(&app, get_req).await;
    assert_eq!(get_resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(get_resp).await;
    assert_eq!(body["data"]["id"], id);
    assert_eq!(body["data"]["name"], "Mouse");
}

#[actix_web::test]
async fn invalid_json_returns_bad_request() {
    let app = test::init_service(
        App::new()
            .app_data(state())
            .app_data(json_config())
            .wrap(RequestId)
            .configure(configure),
    )
    .await;
    let req = test::TestRequest::post()
        .uri("/api/v1/items")
        .insert_header((header::CONTENT_TYPE, "application/json"))
        .set_payload(r#"{"name": "Broken""#)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["ok"], false);
    assert_eq!(body["error"]["code"], "BAD_REQUEST");
}

#[actix_web::test]
async fn validation_error_returns_bad_request() {
    let app = test::init_service(
        App::new()
            .app_data(state())
            .app_data(json_config())
            .wrap(RequestId)
            .configure(configure),
    )
    .await;
    let req = test::TestRequest::post()
        .uri("/api/v1/items")
        .set_json(json!({"name":"","price":-1}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn unknown_route_returns_not_found() {
    let app = test::init_service(
        App::new()
            .app_data(state())
            .app_data(json_config())
            .wrap(RequestId)
            .configure(configure),
    )
    .await;
    let req = test::TestRequest::get().uri("/api/v1/missing").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["error"]["code"], "NOT_FOUND");
}

#[actix_web::test]
async fn unsupported_method_returns_method_not_allowed() {
    let app = test::init_service(
        App::new()
            .app_data(state())
            .app_data(json_config())
            .wrap(RequestId)
            .configure(configure),
    )
    .await;
    let req = test::TestRequest::delete()
        .uri("/api/v1/health")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["error"]["code"], "METHOD_NOT_ALLOWED");
}

#[actix_web::test]
async fn request_id_is_propagated() {
    let app = test::init_service(
        App::new()
            .app_data(state())
            .app_data(json_config())
            .wrap(RequestId)
            .configure(configure),
    )
    .await;
    let req = test::TestRequest::get()
        .uri("/api/v1/health")
        .insert_header(("x-request-id", "test-request-id"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(
        resp.headers()
            .get("x-request-id")
            .and_then(|value| value.to_str().ok()),
        Some("test-request-id")
    );
}
