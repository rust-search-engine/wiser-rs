use super::PostingList;
use crate::db::DB;
use crate::primitive::DocumentId;
use std::error::Error;

/// 将文档添加到数据库，建立倒排索引
fn add_document(title: vec<u8>, body: Vec<u8>) -> Result<(), Error> {
    unimplemented!()
}

/// 管理小倒排索引
struct MiniPostings {
    buffer: Vec<u8>,
}

impl MiniPostings {
    /// document_id 文档编号
    /// text 输入的字符串
    /// n_gram_number 中N的取值
    /// postings 倒排列表的数组
    fn text_to_postings_lists(
        &mut self,
        document_id: DocumentId,
        text: Vec<u8>,
        n_gram_number: u8,
        postings: Vec<PostingList>,
    ) -> Result<(), Error>{
        unimplemented!()
    }

    fn update_postings(&mut self, db: DB) {
        unimplemented!()
    }
}
