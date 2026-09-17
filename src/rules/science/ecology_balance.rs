//! 生态平衡基础
//!
//! 食物链、生态系统与生物多样性的科学常识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: EcologyBalanceRules,
    name: "生态平衡基础",
    desc: "食物链、生态系统与生物多样性的科学常识",
    origin: "国际",
    tags: ["科学", "生态", "生物多样性", "环境"]
}

impl EcologyBalanceRules {
    /// 生态系统
    pub fn ecosystem(&self) -> Vec<&'static str> {
        vec![
            "生产者消费者分解者并存",
            "物质与能量在系统流动",
            "食物链环环相扣",
            "生态各组分相互依存",
        ]
    }

    /// 生物多样
    pub fn biodiversity(&self) -> Vec<&'static str> {
        vec![
            "物种多样性支撑稳定",
            "遗传多样性增强适应",
            "生境多样庇护物种",
            "多样性是宝贵财富",
        ]
    }

    /// 平衡受扰
    pub fn disturbance(&self) -> Vec<&'static str> {
        vec![
            "外来物种入侵威胁",
            "过度开发破坏栖息地",
            "污染伤及生态系统",
            "人类活动影响平衡",
        ]
    }

    /// 保护行动
    pub fn protect(&self) -> Vec<&'static str> {
        vec![
            "保护自然栖息地",
            "不随意放生外来种",
            "减少过度捕捞猎捕",
            "支持生态保护举措",
        ]
    }
}

impl Rule for EcologyBalanceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("ecology")
    }

    fn explain(&self) -> String {
        format!(
            "【生态平衡基础】\n{}",
            [
                format!(
                    "生态系统：\\n{}",
                    self.ecosystem()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生物多样：\\n{}",
                    self.biodiversity()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "平衡受扰：\\n{}",
                    self.disturbance()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "保护行动：\\n{}",
                    self.protect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
            ]
            .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_ecologybalancerules_basic() {
        let rules = EcologyBalanceRules::new();
        assert_eq!(rules.metadata().name, "生态平衡基础");
        assert!(!rules.ecosystem().is_empty());
        assert!(!rules.biodiversity().is_empty());
        assert!(!rules.disturbance().is_empty());
        assert!(!rules.protect().is_empty());
    }

    #[test]
    fn test_ecologybalancerules_validation() {
        let rules = EcologyBalanceRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("ecology"));
    }

    #[test]
    fn test_ecologybalancerules_explain() {
        let rules = EcologyBalanceRules::new();
        let e = rules.explain();
        assert!(e.contains("生态系统"));
        assert!(e.contains("生物多样"));
        assert!(e.contains("平衡受扰"));
    }
}
