use crate::primitive::DocumentId;
use rbatis::rbatis::Rbatis;
use rbatis::Error;
use std::sync::Arc;

pub type DB = Arc<Rbatis>;

/// 将文中存储到数据库中
fn db_add_document(title: Vec<u8>, body: Vec<u8>) -> Result<(), Error> {
    unimplemented!()
}

/// 获取文档的Id
fn db_get_document_id(title: Vec<u8>) -> Option<DocumentId> {
    unimplemented!()
}
