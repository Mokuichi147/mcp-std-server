use chrono::Local;
use fasteval::Evaler;
use rmcp::{
    Error as McpError,
    ServerHandler,
    model::{
        CallToolResult,
        Content,
        ServerCapabilities,
        ServerInfo
    },
    tool,
};

#[derive(Clone)]
pub struct Functions {}

#[tool(tool_box)]
impl Functions {
    pub fn new() -> Self {
        Functions {}
    }

    #[tool(description = "現在の時刻を取得します。")]
    pub async fn get_current_time() -> Result<CallToolResult, McpError> {
        let now = Local::now();
        Ok(CallToolResult::success(vec![Content::text(now.to_string())]))
    }

    #[tool(description = "複雑な計算式を正確に計算します。")]
    pub async fn calculator(
        #[tool(param)]
        #[schemars(description = "計算式を入力してください。次のような式を計算できます。\"1+sum(2,3)*abs(4-5)/6^7\"")]
        formula: String
    ) -> Result<CallToolResult, McpError> {
        let parser = fasteval::Parser::new();
        let mut slab = fasteval::Slab::new();
        let val = parser.parse(&formula, &mut slab.ps);
        if let Err(e) = val {
            return Ok(CallToolResult::error(
                vec![Content::text(format!("計算式の解析に失敗しました: {}", e))],
            ));
        }

        let val = val.unwrap()
            .from(&slab.ps)
            .eval(&slab, &mut fasteval::EmptyNamespace);

        if let Err(e) = val {
            return Ok(CallToolResult::error(
                vec![Content::text(format!("計算式の評価に失敗しました: {}", e))],
            ));
        }
        let val = val.unwrap();

        Ok(CallToolResult::success(vec![Content::text(val.to_string())]))
    }
}

#[tool(tool_box)]
impl ServerHandler for Functions {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("このサーバーはLLMに欠かせない標準的な機能を提供します。現在時刻の取得や複雑な計算式を正確に計算するといったことが行えます。".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}