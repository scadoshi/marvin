use std::sync::Arc;

use crawl::Crawl;
use extract::Extract;
use map::Map;
use rig::tool::ToolDyn;
use search::Search;
use tavily::TavilyClient;

pub(crate) mod crawl;
pub(crate) mod extract;
pub(crate) mod map;
pub(crate) mod search;
pub(crate) mod tavily;

pub(crate) fn tools(client: &Arc<TavilyClient>) -> Vec<Box<dyn ToolDyn>> {
    vec![
        Box::new(Search::new(Arc::clone(client))),
        Box::new(Extract::new(Arc::clone(client))),
        Box::new(Crawl::new(Arc::clone(client))),
        Box::new(Map::new(Arc::clone(client))),
    ]
}
