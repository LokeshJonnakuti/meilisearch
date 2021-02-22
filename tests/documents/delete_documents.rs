use serde_json::json;

use crate::common::{Server, GetAllDocumentsOptions};

#[actix_rt::test]
async fn delete_one_document_unexisting_index() {
    let server = Server::new().await;
    let (_response, code) = server.index("test").delete_document(0).await;
    assert_eq!(code, 400);
}

#[actix_rt::test]
async fn delete_one_unexisting_document() {
    let server = Server::new().await;
    let index = server.index("test");
    index.create(None).await;
    let (_response, code) = index.delete_document(0).await;
    assert_eq!(code, 200);
    let update = index.wait_update_id(0).await;
    assert_eq!(update["status"], "processed");
}

#[actix_rt::test]
async fn delete_one_document() {
    let server = Server::new().await;
    let index = server.index("test");
    index.add_documents(json!([{ "id": 0, "content": "foobar" }]), None).await;
    index.wait_update_id(0).await;
    let (_response, code) = server.index("test").delete_document(0).await;
    assert_eq!(code, 200);

    let (_response, code) = index.get_document(0, None).await;
    assert_eq!(code, 400);
}

#[actix_rt::test]
async fn clear_all_documents_unexisting_index() {
    let server = Server::new().await;
    let (_response, code) = server.index("test").clear_all_documents().await;
    assert_eq!(code, 400);

}

#[actix_rt::test]
async fn clear_all_documents() {
    let server = Server::new().await;
    let index = server.index("test");
    index.add_documents(json!([{ "id": 1, "content": "foobar" }, { "id": 0, "content": "foobar" }]), None).await;
    index.wait_update_id(0).await;
    let (_response, code) = index.clear_all_documents().await;
    assert_eq!(code, 200);

    let _update = index.wait_update_id(0).await;
    let (response, code) = index.get_all_documents(GetAllDocumentsOptions::default()).await;
    assert_eq!(code, 200);
    assert!(response.as_array().unwrap().is_empty());
}

#[actix_rt::test]
async fn clear_all_documents_empty_index() {
    let server = Server::new().await;
    let index = server.index("test");
    index.create(None).await;

    let (_response, code) = index.clear_all_documents().await;
    assert_eq!(code, 200);

    let _update = index.wait_update_id(0).await;
    let (response, code) = index.get_all_documents(GetAllDocumentsOptions::default()).await;
    assert_eq!(code, 200);
    assert!(response.as_array().unwrap().is_empty());
}
