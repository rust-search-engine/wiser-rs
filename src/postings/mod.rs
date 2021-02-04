use std::collections::HashMap;

mod postings_create;
///! 倒排列表和倒排文件实现
use crate::primitive::DocumentId;

/// 倒排列表
pub struct PostingList {
    /// 每一个词元的对应的记录的文档中的信息,
    head: Vec<PostingsNode>,
}

/// 倒排列表的单个节点
struct PostingsNode {
    /// 文档编号
    document_id: DocumentId,

    /// 位置信息的数组
    positions: Vec<u32>,

    /// 位置信息的条数，也就是在一个文档中的词频TF
    positions_count: u32,
}

/// 倒排索引文件
struct InvertedIndex {
    // 将倒排索引的key与倒排列表相关联
    invert_map: HashMap<InvertedIndexKey, PostingList>,
}

/// 倒排索引文件的键值信息
struct InvertedIndexKey {
    /// 词元编号
    token_id: u32,
    /// 在所有文档中该词元的出现次数之和
    positions_count: u32,
}
