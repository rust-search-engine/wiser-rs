use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;
mod postings_create;
///! 倒排列表和倒排文件实现
use crate::primitive::DocumentId;
use rbatis::Error;
use std::cmp::PartialEq;

/// 倒排列表（以文档编号和位置信息为元素的链表结构）
#[derive(Debug, PartialEq)]
pub struct PostingsList {
    /// 每一个词元的对应的记录的文档中的信息,
    head: VecDeque<PostingsNode>,
}

impl PostingsList {
    fn new() -> PostingsList {
        Self {
            head: VecDeque::new(),
        }
    }
    fn len(&self) -> usize {
        self.head.len()
    }
    fn pop_front(&mut self) -> Option<PostingsNode> {
        self.head.pop_front()
    }
    fn get_front(&self) -> Option<&PostingsNode> {
        self.head.get(0)
    }
    fn push_back(&mut self, node: PostingsNode) {
        self.head.push_back(node)
    }
    fn is_empty(&self) -> bool {
        self.head.is_empty()
    }
}

impl FromIterator<PostingsNode> for PostingsList {
    fn from_iter<I: IntoIterator<Item=PostingsNode>>(iter: I) -> Self {
        let mut c = PostingsList::new();
        for i in iter {
            c.head.push_back(i);
        }
        c
    }
}

/// 合并倒排列表
/// 会将pb的倒排列表中的元素添加到pa倒排列表中
fn merge_postings(mut pa: PostingsList, mut pb: PostingsList) -> Option<PostingsList> {
    let mut ret = PostingsList::new();
    let mut index: usize = 0;
    let pa_len = pa.len();
    let pb_len = pb.len();

    if pa.is_empty() && !pb.is_empty() { // 当pa为空，pb不为空时
        println!("pa is empty, but pb is not empty");
        ret = pb;
    } else if !pa.is_empty() && pb.is_empty() { // 当pa不为空，pb为空时
        println!("pa is not empty, but ba is empty");
        ret = pa;
    } else if !pa.is_empty() && !pb.is_empty() { // 当两者都不为空时
        println!("pa and pb is not empty");
        loop {
            if index < pa_len || index < pb_len {
                let tmp_pa = pa.get_front().unwrap();
                let tmp_pb = pb.get_front().unwrap();
                println!("temp pa = {:?}, temp pb = {:?}", tmp_pa, tmp_pb);
                if tmp_pa.get_document_id() < tmp_pb.get_document_id(){
                    let tmp_pa = pa.pop_front().unwrap();
                    ret.push_back(tmp_pa);
                } else {
                    let tmp_pb = pb.pop_front().unwrap();
                    ret.push_back(tmp_pb);
                }
            } else {
                break;
            }
            index += 1;
        }
        // 将剩余的数据放入ret
        if !pa.is_empty() {
            println!("pa is {:?}", pa);
            for val in pa.head.into_iter() {
                ret.push_back(val);
            }
        } else if !pb.is_empty() {
            println!("pb is {:?}", pb);
            for val in pb.head.into_iter() {
                ret.push_back(val);
            }
        }
    }

    Some(ret)
}

/// 倒排列表的单个节点
#[derive(Debug, PartialEq)]
struct PostingsNode {
    /// 文档编号
    document_id: u32,

    /// 位置信息的数组
    positions: Vec<u32>,

    // 位置信息的条数，也就是在一个文档中的词频TF
    positions_count: u32,
}

impl PostingsNode {
    fn new() -> Self {
        Self {
            document_id: 0,
            positions: Vec::new(),
            positions_count: 0,
        }
    }
    fn get_document_id(&self) -> u32 {
        self.document_id
    }
    fn set_document_id(&mut self, val: u32)  {
        self.document_id = val;
    }

}

/// 倒排索引文件
struct InvertedIndex {
    // 将倒排索引的key与倒排列表相关联
    invert_map: HashMap<InvertedIndexKey, PostingsList>,
}

/// 倒排索引文件的键值信息
struct InvertedIndexKey {
    /// 词元编号（Token ID）
    /// 将词元及其编号关联起来的数据结构充当所谓的词典
    token_id: u32,

    /// 出现该词元的文档数
    docs_count: u32,

    /// 在所有文档中该词元的出现次数之和, 词频
    positions_count: u32,
}


#[test]
fn test_postings_list_merge_list() {

    let mut pa = PostingsList::new();
    let temp_pa = (0..10).map(|val|{
        let mut ret = PostingsNode::new();
        ret.set_document_id(val + 1);
        ret
    }).collect::<PostingsList>();
    println!("temp_pa = {:?}", temp_pa);
    let temp_pb = (10..20).map(|val|{
        let mut ret = PostingsNode::new();
        ret.set_document_id(val + 1);
        ret
    }).collect::<PostingsList>();
    println!("temp_pb = {:?}", temp_pb);

    let merge_result = (0..20).map(|val|{
        let mut ret = PostingsNode::new();
        ret.set_document_id(val + 1);
        ret
    }).collect::<PostingsList>();

    let result = merge_postings(temp_pa, temp_pb).unwrap();
    println!("result = {:?}", result);
    assert_eq!(merge_result, result);
}