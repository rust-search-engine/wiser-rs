#[macro_use]
extern crate rbatis;

use std::sync::Arc;
use rbatis::rbatis::Rbatis;
use rbatis::crud::CRUD;
use rbatis::Error;
use rbatis::core::db::DBPoolOptions;


#[crud_enable]
#[derive(Debug, Clone)]
pub struct Wiser {
    // 主键id
    pub id: Option<String>,
    // 文章标题
    pub title: Option<String>,
    // 文章内容
    pub body: Option<String>,
}

impl Default for Wiser {
    fn  default() -> Self {
        Self {
            id: None,
            title: None,
            body: None,
        }
    }
}


pub const MYSQL_URL : &str = "mysql://root:123456@182.61.33.66:3306/search_engine";


pub async fn init_rbatis() -> Arc<Rbatis> {
    let _ = fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);

    let mut opt = DBPoolOptions::new();
    opt.max_connections = 20;

    let rb = Rbatis::new();

    rb.link_opt(MYSQL_URL, &opt)
        .await.unwrap();

    Arc::new(rb)
}

impl Wiser {
    pub fn new(title: String, body: String) -> Self {
        Self {
            id: None,
            title: Some(title),
            body: Some(body),
        }
    }

    pub async fn add_document(self, db: Arc<Rbatis>)  {

        let w = db.new_wrapper().eq("title", self.title.clone().unwrap().clone());

        let add_title: Result<Vec<Wiser>, Error>  = db.fetch_list_by_wrapper("", &w).await;

        match add_title {
            Err(_error) => {
                log::info!("search document by title error");
            },
            Ok(res) => {
                if res.is_empty() {
                    let result = db.save("", &self).await;
                    match result {
                        Ok(_success) => {
                            log::info!("document add successful!");
                        },
                        Err(_error) => {
                            log::info!("document add failed");
                        }
                    }
                }else {
                    log::info!(" --> title already exists (title: {}, body: {})", self.title.unwrap().clone(), self.body.unwrap().clone());
                }
            }
        }
    }

    /// get document id
    pub async fn get_document_id(&self, db: Arc<Rbatis>) -> u32 {
        let w = db.new_wrapper().eq("title", self.title.clone().unwrap().clone());
        let result : Result<Option<Wiser>, Error> = db.fetch_by_wrapper("", &w).await;
        println!("is_some: {:?}", result);

        return if result.is_err() {
            println!("{}", result.err().unwrap().to_string());
            // if result is error return u32::MAX
            u32::MAX
        } else {
            let ret = result.unwrap().unwrap().id.unwrap().parse::<u32>().unwrap();
            ret
        }
    }
}


#[tokio::test]
async fn test_add_document() {
    let wiser1 = Wiser::new("davirain".to_string(), "from Zero to Hero".to_string());
    let db = init_rbatis().await;
    wiser1.add_document(db);
}

#[tokio::test]
async fn test_init_rbatis() {
    let _  = init_rbatis().await;
}