use std::sync::Arc;

use crawl::Crawl;
use extract::Extract;
use map::Map;
use rig::tool::ToolDyn;
use search::Search;
use tavily::TavilyClient;

pub mod crawl;
pub mod extract;
pub mod map;
pub mod search;
pub mod tavily;

pub fn tools(client: &Arc<TavilyClient>) -> Vec<Box<dyn ToolDyn>> {
    vec![
        Box::new(Search::new(Arc::clone(client))),
        Box::new(Extract::new(Arc::clone(client))),
        Box::new(Crawl::new(Arc::clone(client))),
        Box::new(Map::new(Arc::clone(client))),
    ]
}
