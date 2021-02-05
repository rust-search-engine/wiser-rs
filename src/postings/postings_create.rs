use super::PostingsList;
use crate::db::DB;
use crate::primitive::DocumentId;
use std::error::Error;

/// 将文档添加到数据库，建立倒排索引
fn add_document(title: Vec<u8>, body: Vec<u8>)  {
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
        postings: Vec<PostingsList>,
    ) {
        unimplemented!()
    }

    /// 将内存上（小倒排索引）的倒排列表与存储器上的倒排列表合并后存储
    fn update_postings(&mut self, db: DB) {
        unimplemented!()
    }
}

/// 作用是为文档中的一个词元创建倒排列表
/// 为传入的词元创建倒排列表
fn token_to_posting_list(
    document_id: u32,
    token: String,
    position: u32, /*inverted_index_hash*/
) {
    unimplemented!()
}

/// 合并两个倒排索引
/// base: 合并后其中的元素会被增多的倒排索引（合并目标）
/// to_be_added: 合并后就被释放的倒排索引（合并源）
fn merge_inverted_index(/*base: inverted_index_hash, to_be_added: inverted_index_hash*/
)  {
    unimplemented!()
}
