use aleph_syntax_tree::syntax::AlephTree;
use em_filter::{async_trait, AgentConfig, EmFilterError, Filter, FilterRunner};
use serde_json::{json, Value};

struct BetaReductionAgent;

#[async_trait]
impl Filter for BetaReductionAgent {
    async fn handle(&mut self, body: &str) -> Result<Value, EmFilterError> {
        tracing::info!(len = body.len(), "applying beta reduction");

        let embryo: Value = serde_json::from_str(body)?;
        let tree: AlephTree = serde_json::from_value(embryo["properties"]["tree"].clone())?;
        let source_lang = embryo["properties"]["source_language"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();

        let transformed = betareduction::transform(tree);
        let tree_out = serde_json::to_value(transformed)?;

        tracing::info!(source_lang = %source_lang, "beta reduction complete");

        Ok(json!([{
            "type": "aleph_tree",
            "properties": {
                "source_language": source_lang,
                "tree": tree_out
            }
        }]))
    }

    fn capabilities(&self) -> Vec<String> {
        vec!["transform".into(), "betareduction".into()]
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    FilterRunner::new("em_betareduction", BetaReductionAgent, AgentConfig::default())
        .run()
        .await
        .unwrap();
}
