//! # 规则市场 Web 界面
//!
//! 提供Web界面渲染和API端点。

use super::search::{SearchEngine, SearchFilter, SearchResult};
use super::types::{MarketError, MarketStats, Marketplace, RulePackage};

/// Web服务器配置
#[derive(Debug, Clone)]
pub struct WebConfig {
    /// 服务器地址
    pub host: String,
    /// 服务器端口
    pub port: u16,
    /// 静态文件目录
    pub static_dir: String,
    /// 模板目录
    pub template_dir: String,
}

impl Default for WebConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            static_dir: "static".to_string(),
            template_dir: "templates".to_string(),
        }
    }
}

/// Web服务器
#[derive(Debug)]
pub struct WebServer {
    /// Web配置
    config: WebConfig,
    /// 市场实例
    market: Marketplace,
    /// 搜索引擎
    search_engine: SearchEngine,
}

impl WebServer {
    /// 创建新的Web服务器
    ///
    /// # Examples
    ///
    /// ```rust
    /// use world_rules::market::{Marketplace, MarketConfig, WebServer, WebConfig};
    ///
    /// let config = WebConfig::default();
    /// let market = Marketplace::new(MarketConfig::default());
    /// let server = WebServer::new(config, market);
    /// ```
    pub fn new(config: WebConfig, market: Marketplace) -> Self {
        Self {
            config,
            market,
            search_engine: SearchEngine::new(),
        }
    }

    /// 获取服务器地址
    pub fn address(&self) -> String {
        format!("{}:{}", self.config.host, self.config.port)
    }

    /// 处理首页请求
    pub fn handle_index(&self) -> Result<PageContent, MarketError> {
        let stats = self.market.stats();
        let featured = self.get_featured_rules()?;

        Ok(PageContent::Index { stats, featured })
    }

    /// 处理搜索请求
    ///
    /// # Arguments
    ///
    /// * `query` - 搜索关键词
    /// * `filter` - 搜索过滤器
    pub fn handle_search(
        &self,
        query: &str,
        filter: SearchFilter,
    ) -> Result<PageContent, MarketError> {
        let search_result = self.search_engine.search(&filter)?;

        Ok(PageContent::Search {
            query: query.to_string(),
            result: search_result,
        })
    }

    /// 处理规则详情请求
    ///
    /// # Arguments
    ///
    /// * `rule_id` - 规则ID
    pub fn handle_detail(&self, rule_id: &str) -> Result<PageContent, MarketError> {
        self.market
            .get_rule(rule_id)
            .map(|pkg| PageContent::Detail(pkg.clone()))
            .ok_or_else(|| MarketError::NotFound(rule_id.to_string()))
    }

    /// 处理分类浏览请求
    ///
    /// # Arguments
    ///
    /// * `category` - 分类名称
    pub fn handle_category(&self, category: &str) -> Result<PageContent, MarketError> {
        let filter = SearchFilter::new().category(category);
        let result = self.search_engine.search(&filter)?;

        Ok(PageContent::Category {
            category: category.to_string(),
            result,
        })
    }

    /// 处理上传请求
    ///
    /// # Arguments
    ///
    /// * `package` - 规则包
    pub fn handle_upload(&mut self, package: RulePackage) -> Result<String, MarketError> {
        self.search_engine.index(package.clone());
        self.market.upload(package)
    }

    /// 处理下载请求
    ///
    /// # Arguments
    ///
    /// * `rule_id` - 规则ID
    pub fn handle_download(&self, rule_id: &str) -> Result<&RulePackage, MarketError> {
        self.market
            .download(rule_id)
            .ok_or_else(|| MarketError::NotFound(rule_id.to_string()))
    }

    /// 获取推荐规则
    fn get_featured_rules(&self) -> Result<Vec<RulePackage>, MarketError> {
        let filter = SearchFilter::new().min_rating(4.0);
        let result = self.search_engine.search(&filter)?;

        // 只取前5个
        Ok(result.packages.into_iter().take(5).collect())
    }
}

/// 页面内容
#[derive(Debug, Clone)]
pub enum PageContent {
    /// 首页
    Index {
        stats: MarketStats,
        featured: Vec<RulePackage>,
    },
    /// 搜索结果
    Search { query: String, result: SearchResult },
    /// 规则详情
    Detail(RulePackage),
    /// 分类浏览
    Category {
        category: String,
        result: SearchResult,
    },
}

/// 页面渲染器
#[derive(Debug)]
pub struct PageRenderer {
    /// 模板目录
    #[allow(dead_code)]
    template_dir: String,
}

impl PageRenderer {
    /// 创建新的页面渲染器
    pub fn new(template_dir: impl Into<String>) -> Self {
        Self {
            template_dir: template_dir.into(),
        }
    }

    /// 渲染页面为HTML
    ///
    /// # Arguments
    ///
    /// * `content` - 页面内容
    pub fn render(&self, content: &PageContent) -> Result<String, MarketError> {
        match content {
            PageContent::Index { stats, featured } => self.render_index(stats, featured),
            PageContent::Search { query, result } => self.render_search(query, result),
            PageContent::Detail(package) => self.render_detail(package),
            PageContent::Category { category, result } => self.render_category(category, result),
        }
    }

    fn render_index(
        &self,
        stats: &MarketStats,
        featured: &[RulePackage],
    ) -> Result<String, MarketError> {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"zh-CN\">\n");
        html.push_str("<head>\n");
        html.push_str("  <meta charset=\"UTF-8\">\n");
        html.push_str("  <title>World Rules - 规则市场</title>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("  <h1>欢迎来到 World Rules 市场</h1>\n");
        html.push_str(&format!(
            "  <p>共有 {} 个规则，总下载 {} 次</p>\n",
            stats.total_rules, stats.total_downloads
        ));
        html.push_str("  <h2>推荐规则</h2>\n");
        html.push_str("  <ul>\n");

        for package in featured {
            html.push_str(&format!(
                "    <li><a href=\"/rules/{}\">{}</a> - 评分: {:.1}</li>\n",
                package.id, package.name, package.rating
            ));
        }

        html.push_str("  </ul>\n");
        html.push_str("  <form action=\"/search\" method=\"get\">\n");
        html.push_str("    <input type=\"text\" name=\"q\" placeholder=\"搜索规则...\">\n");
        html.push_str("    <button type=\"submit\">搜索</button>\n");
        html.push_str("  </form>\n");
        html.push_str("</body>\n");
        html.push_str("</html>\n");

        Ok(html)
    }

    fn render_search(&self, query: &str, result: &SearchResult) -> Result<String, MarketError> {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"zh-CN\">\n");
        html.push_str("<head>\n");
        html.push_str("  <meta charset=\"UTF-8\">\n");
        html.push_str("  <title>搜索结果 - World Rules</title>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str(&format!("  <h1>搜索: \"{}\"</h1>\n", query));
        html.push_str(&format!(
            "  <p>找到 {} 个结果（耗时 {} 毫秒）</p>\n",
            result.total, result.took_ms
        ));
        html.push_str("  <ul>\n");

        for package in &result.packages {
            html.push_str(&format!(
                "    <li><a href=\"/rules/{}\">{}</a> - {} - 评分: {:.1}</li>\n",
                package.id, package.name, package.category, package.rating
            ));
        }

        html.push_str("  </ul>\n");
        html.push_str("</body>\n");
        html.push_str("</html>\n");

        Ok(html)
    }

    fn render_detail(&self, package: &RulePackage) -> Result<String, MarketError> {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"zh-CN\">\n");
        html.push_str("<head>\n");
        html.push_str("  <meta charset=\"UTF-8\">\n");
        html.push_str(&format!(
            "  <title>{} - World Rules</title>\n",
            package.name
        ));
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str(&format!("  <h1>{}</h1>\n", package.name));
        html.push_str(&format!("  <p><strong>ID:</strong> {}</p>\n", package.id));
        html.push_str(&format!(
            "  <p><strong>分类:</strong> {}</p>\n",
            package.category
        ));
        html.push_str(&format!(
            "  <p><strong>作者:</strong> {}</p>\n",
            package.author
        ));
        html.push_str(&format!(
            "  <p><strong>版本:</strong> {}</p>\n",
            package.version
        ));
        html.push_str(&format!(
            "  <p><strong>评分:</strong> {:.1}/5.0</p>\n",
            package.rating
        ));
        html.push_str(&format!(
            "  <p><strong>下载次数:</strong> {}</p>\n",
            package.downloads
        ));
        html.push_str(&format!(
            "  <p><strong>描述:</strong> {}</p>\n",
            package.description
        ));
        html.push_str("  <p><strong>标签:</strong> ");
        html.push_str(&package.tags.join(", "));
        html.push_str("</p>\n");
        html.push_str("  <a href=\"/download/");
        html.push_str(&package.id);
        html.push_str("\">下载规则</a>\n");
        html.push_str("</body>\n");
        html.push_str("</html>\n");

        Ok(html)
    }

    fn render_category(
        &self,
        category: &str,
        result: &SearchResult,
    ) -> Result<String, MarketError> {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"zh-CN\">\n");
        html.push_str("<head>\n");
        html.push_str("  <meta charset=\"UTF-8\">\n");
        html.push_str(&format!("  <title>{} - World Rules</title>\n", category));
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str(&format!("  <h1>分类: {}</h1>\n", category));
        html.push_str(&format!("  <p>共 {} 个规则</p>\n", result.total));
        html.push_str("  <ul>\n");

        for package in &result.packages {
            html.push_str(&format!(
                "    <li><a href=\"/rules/{}\">{}</a> - 评分: {:.1}</li>\n",
                package.id, package.name, package.rating
            ));
        }

        html.push_str("  </ul>\n");
        html.push_str("</body>\n");
        html.push_str("</html>\n");

        Ok(html)
    }
}

/// API响应
#[derive(Debug, Clone, serde::Serialize)]
pub struct ApiResponse<T> {
    /// 是否成功
    pub success: bool,
    /// 数据
    pub data: Option<T>,
    /// 错误信息
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    /// 创建成功响应
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    /// 创建错误响应
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::market::MarketConfig;

    #[test]
    fn test_web_config_default() {
        let config = WebConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);
    }

    #[test]
    fn test_web_server_creation() {
        let config = WebConfig::default();
        let market = Marketplace::new(MarketConfig::default());
        let server = WebServer::new(config, market);

        assert_eq!(server.address(), "127.0.0.1:8080");
    }

    #[test]
    fn test_web_server_index() {
        let config = WebConfig::default();
        let market = Marketplace::new(MarketConfig::default());
        let server = WebServer::new(config, market);

        let content = server.handle_index().unwrap();
        match content {
            PageContent::Index { stats, .. } => {
                assert_eq!(stats.total_rules, 0);
            }
            _ => panic!("Expected Index page"),
        }
    }

    #[test]
    fn test_web_server_search() {
        let config = WebConfig::default();
        let market = Marketplace::new(MarketConfig::default());
        let server = WebServer::new(config, market);

        let filter = SearchFilter::new();
        let content = server.handle_search("麻将", filter).unwrap();
        match content {
            PageContent::Search { query, .. } => {
                assert_eq!(query, "麻将");
            }
            _ => panic!("Expected Search page"),
        }
    }

    #[test]
    fn test_web_server_detail_not_found() {
        let config = WebConfig::default();
        let market = Marketplace::new(MarketConfig::default());
        let server = WebServer::new(config, market);

        let result = server.handle_detail("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_page_renderer_index() {
        let renderer = PageRenderer::new("templates");
        let stats = MarketStats {
            total_rules: 10,
            total_downloads: 100,
            categories: vec!["games".to_string()],
        };
        let content = PageContent::Index {
            stats,
            featured: vec![],
        };

        let html = renderer.render(&content).unwrap();
        assert!(html.contains("World Rules"));
        assert!(html.contains("10 个规则"));
    }

    #[test]
    fn test_page_renderer_search() {
        let renderer = PageRenderer::new("templates");
        let result = SearchResult {
            packages: vec![],
            total: 0,
            took_ms: 10,
        };
        let content = PageContent::Search {
            query: "麻将".to_string(),
            result,
        };

        let html = renderer.render(&content).unwrap();
        assert!(html.contains("搜索: \"麻将\""));
    }

    #[test]
    fn test_page_renderer_detail() {
        let renderer = PageRenderer::new("templates");
        let package = RulePackage::new(
            "test-1".to_string(),
            "四川麻将".to_string(),
            "games".to_string(),
        );
        let content = PageContent::Detail(package);

        let html = renderer.render(&content).unwrap();
        assert!(html.contains("四川麻将"));
        assert!(html.contains("games"));
    }

    #[test]
    fn test_api_response_success() {
        let response = ApiResponse::success(42);
        assert!(response.success);
        assert_eq!(response.data, Some(42));
        assert!(response.error.is_none());
    }

    #[test]
    fn test_api_response_error() {
        let response: ApiResponse<()> = ApiResponse::error("测试错误");
        assert!(!response.success);
        assert!(response.data.is_none());
        assert_eq!(response.error, Some("测试错误".to_string()));
    }

    #[test]
    fn test_web_server_upload() {
        let config = WebConfig::default();
        let market = Marketplace::new(MarketConfig::default());
        let mut server = WebServer::new(config, market);

        let package = RulePackage::new(
            "test-1".to_string(),
            "四川麻将".to_string(),
            "games".to_string(),
        );

        let id = server.handle_upload(package).unwrap();
        assert_eq!(id, "test-1");

        // 验证可以搜索到
        let content = server.handle_detail("test-1").unwrap();
        match content {
            PageContent::Detail(pkg) => {
                assert_eq!(pkg.name, "四川麻将");
            }
            _ => panic!("Expected Detail page"),
        }
    }
}
