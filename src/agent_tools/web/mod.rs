use crawl::Crawl;
use extract::Extract;
use map::Map;
use rig::tool::ToolDyn;
use search::Search;
use std::sync::Arc;
use tavily::TavilyClient;

pub mod crawl;
pub mod extract;
pub mod map;
pub mod search;
pub mod tavily;

pub trait WebTools {
    fn web_tools(&self) -> Vec<Box<dyn ToolDyn>>;
}

impl WebTools for Arc<TavilyClient> {
    fn web_tools(&self) -> Vec<Box<dyn ToolDyn>> {
        vec![
            Box::new(Search::new(self.clone())),
            Box::new(Extract::new(self.clone())),
            Box::new(Crawl::new(self.clone())),
            Box::new(Map::new(self.clone())),
        ]
    }
}
