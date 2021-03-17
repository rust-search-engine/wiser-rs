#![allow(dead_code)]

/// 管理的所有单词的倒排列表
pub struct PostingsLists {
    head: Vec<PostingList>,
}

struct PostingList {
    /// 文档编号
    document_id: u32,
    /// 单词位置信息数组
    words_positions: Vec<u32>,
}

/// 一个词元的倒排文件
pub struct InvertedDoc {
    /// 词元编号
    token_id: u32,
    /// 该单词包含的倒排列表
    postings_lists: PostingList,
    /// 文档数
    doc_count: u32,
    /// 所有文档中该词元出现次数之和
    positions_count: u32,
}

/// 词典
pub struct Dictionary {
    ///单词
    word: String,
    /// 词元编号
    token_id: u32,
}




