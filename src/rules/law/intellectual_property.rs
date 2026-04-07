//! 知识产权法规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 知识产权类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IPType {
    Copyright,   // 著作权
    Patent,      // 专利
    Trademark,   // 商标
    TradeSecret, // 商业秘密
}

impl IPType {
    pub fn name(&self) -> &'static str {
        match self {
            IPType::Copyright => "著作权",
            IPType::Patent => "专利",
            IPType::Trademark => "商标",
            IPType::TradeSecret => "商业秘密",
        }
    }
}

/// 知识产权法规则
pub struct IPRules {
    metadata: RuleMetadata,
}

impl IPRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "知识产权法",
                "中国知识产权基本规则"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "知识产权".into()]),
        }
    }

    /// 著作权规则
    pub fn copyright_rules(&self) -> Vec<&'static str> {
        vec![
            "保护期限: 作者终身+50年",
            "自动取得: 作品完成即享有",
            "保护范围: 文学、艺术、科学作品",
            "包括人身权和财产权",
            "合理使用: 教学、科研等非商业使用",
        ]
    }

    /// 专利规则
    pub fn patent_rules(&self) -> Vec<&'static str> {
        vec![
            "发明专利: 保护20年",
            "实用新型专利: 保护10年",
            "外观设计专利: 保护15年",
            "先申请原则",
            "需要实质审查(发明专利)",
        ]
    }

    /// 商标规则
    pub fn trademark_rules(&self) -> Vec<&'static str> {
        vec![
            "注册原则: 先注册先保护",
            "保护期限: 10年，可续展",
            "分类注册: 45类商品和服务",
            "禁止抢注他人已使用商标",
        ]
    }

    /// 商业秘密规则
    pub fn trade_secret_rules(&self) -> Vec<&'static str> {
        vec![
            "不为公众所知悉",
            "具有商业价值",
            "采取保密措施",
            "无保护期限限制",
        ]
    }

    /// 侵权行为
    pub fn infringement_types(&self) -> Vec<&'static str> {
        vec![
            "未经许可复制发行",
            "假冒专利",
            "商标侵权",
            "侵犯商业秘密",
            "盗版",
        ]
    }

    /// 侵权救济
    pub fn remedies(&self) -> Vec<&'static str> {
        vec![
            "停止侵权",
            "消除影响",
            "赔偿损失",
            "行政罚款",
            "刑事处罚(严重情形)",
        ]
    }
}

impl Default for IPRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for IPRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("intellectual_property")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【知识产权法】\n\n\
            著作权规则:\n{}\n\n\
            专利规则:\n{}\n\n\
            商标规则:\n{}\n\n\
            商业秘密规则:\n{}\n\n\
            侵权救济:\n{}\n",
            self.copyright_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.patent_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.trademark_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.trade_secret_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.remedies().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_rules() {
        let rules = IPRules::new();
        assert!(!rules.copyright_rules().is_empty());
    }
}