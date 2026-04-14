//! 广告法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 广告法规则
pub struct AdvertisingLawRules {
    metadata: RuleMetadata,
}

impl AdvertisingLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "广告法规则",
                "中国广告法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "广告".into()]),
        }
    }

    /// 广告基本原则
    pub fn basic_principles(&self) -> Vec<&'static str> {
        vec![
            "真实原则: 广告真实",
            "合法原则: 广告合法",
            "诚信原则: 广告诚信",
            "公平竞争原则",
            "保护消费者原则",
            "社会效益原则",
            "未成年人保护原则",
            "特殊人群保护原则",
        ]
    }

    /// 广告内容规范
    pub fn content_rules(&self) -> Vec<&'static str> {
        vec![
            "禁止虚假广告",
            "禁止误导广告",
            "禁止绝对化用语",
            "禁止贬低他人广告",
            "禁止违法内容广告",
            "禁止不良价值观广告",
            "广告内容真实性要求",
            "广告内容合法性要求",
        ]
    }

    /// 特殊商品广告
    pub fn special_product_ads(&self) -> Vec<&'static str> {
        vec![
            "药品广告审查",
            "医疗器械广告审查",
            "保健食品广告审查",
            "农药广告审查",
            "兽药广告审查",
            "食品广告审查",
            "化妆品广告审查",
            "酒类广告审查",
        ]
    }

    /// 禁止广告类型
    pub fn prohibited_ads(&self) -> Vec<&'static str> {
        vec![
            "烟草广告禁止",
            "处方药广告禁止",
            "母乳代用品广告禁止",
            "非法集资广告禁止",
            "淫秽色情广告禁止",
            "暴力恐怖广告禁止",
            "迷信邪教广告禁止",
            "歧视性广告禁止",
        ]
    }

    /// 广告代言人规则
    pub fn spokesperson_rules(&self) -> Vec<&'static str> {
        vec![
            "代言人真实性义务",
            "代言人使用产品要求",
            "代言人禁止代言情形",
            "代言人连带责任",
            "代言人虚假代言处罚",
            "代言人推荐证明要求",
            "代言人信息披露要求",
            "代言人广告审查",
        ]
    }

    /// 广告发布规范
    pub fn publication_rules(&self) -> Vec<&'static str> {
        vec![
            "广告发布审查",
            "广告发布登记",
            "广告发布渠道规范",
            "广告发布时段限制",
            "广告发布频率限制",
            "广告发布位置限制",
            "广告发布内容审核",
            "广告发布档案保存",
        ]
    }

    /// 广告违法行为
    pub fn advertising_violations(&self) -> Vec<&'static str> {
        vec![
            "虚假广告违法行为",
            "违法内容广告行为",
            "未经审查广告发布",
            "代言违法广告行为",
            "广告误导行为",
            "广告欺骗行为",
            "广告不正当竞争",
            "广告侵权行为",
        ]
    }

    /// 广告监管处罚
    pub fn regulatory_penalties(&self) -> Vec<&'static str> {
        vec![
            "广告违法罚款",
            "广告违法停业",
            "广告违法吊销许可",
            "广告违法责令整改",
            "广告违法没收所得",
            "广告违法刑事责任",
            "广告违法民事赔偿",
            "广告违法信用惩戒",
        ]
    }
}

impl Default for AdvertisingLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AdvertisingLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("advertising")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【广告法规则】\n\n基本原则:\n{}\n\n内容规范:\n{}\n\n禁止广告:\n{}\n",
            self.basic_principles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.content_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.prohibited_ads().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advertising_law_rules() {
        let rules = AdvertisingLawRules::new();
        assert!(!rules.basic_principles().is_empty());
        assert!(!rules.content_rules().is_empty());
    }
}