//! 可持续生活方式
//!
//! 资源节约、循环再利用的可持续生活常识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SustainableLivingScienceRules,
    name: "可持续生活方式",
    desc: "资源节约、循环再利用的可持续生活常识",
    origin: "国际",
    tags: ["科学", "可持续", "环保", "循环"]
}

impl SustainableLivingScienceRules {
    /// 减少消耗
    pub fn reduce(&self) -> Vec<&'static str> {
        vec![
            "购物按需不冲动",
            "减少一次性用品",
            "节约用水用电",
            "修旧利废减少浪费",
        ]
    }

    /// 循环再用
    pub fn reuse(&self) -> Vec<&'static str> {
        vec![
            "物尽其用再利用",
            "闲置物品转赠他人",
            "分门别类回收",
            "选择可循环包装",
        ]
    }

    /// 理性消费
    pub fn consume(&self) -> Vec<&'static str> {
        vec![
            "选择耐用产品",
            "关注健康环保材质",
            "不盲目跟风消费",
            "重视使用价值",
        ]
    }

    /// 共担责任
    pub fn responsibility(&self) -> Vec<&'static str> {
        vec![
            "关心环境与社区",
            "宣传低碳意识",
            "参与环保活动",
            "长远眼光看待自然",
        ]
    }
}

impl Rule for SustainableLivingScienceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("sustainable")
    }

    fn explain(&self) -> String {
        format!(
            "【可持续生活方式】\n{}",
            [
                format!(
                    "减少消耗：\\n{}",
                    self.reduce()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "循环再用：\\n{}",
                    self.reuse()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "理性消费：\\n{}",
                    self.consume()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "共担责任：\\n{}",
                    self.responsibility()
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
    fn test_sustainablelivingsciencerules_basic() {
        let rules = SustainableLivingScienceRules::new();
        assert_eq!(rules.metadata().name, "可持续生活方式");
        assert!(!rules.reduce().is_empty());
        assert!(!rules.reuse().is_empty());
        assert!(!rules.consume().is_empty());
        assert!(!rules.responsibility().is_empty());
    }

    #[test]
    fn test_sustainablelivingsciencerules_validation() {
        let rules = SustainableLivingScienceRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("sustainable"));
    }

    #[test]
    fn test_sustainablelivingsciencerules_explain() {
        let rules = SustainableLivingScienceRules::new();
        let e = rules.explain();
        assert!(e.contains("减少消耗"));
        assert!(e.contains("循环再用"));
        assert!(e.contains("理性消费"));
    }
}
