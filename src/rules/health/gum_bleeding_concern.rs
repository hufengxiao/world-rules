//! 牙龈出血处理
//!
//! 刷牙出血、牙龈炎的识别与正确应对

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: GumBleedingConcernRules,
    name: "牙龈出血处理",
    desc: "刷牙出血、牙龈炎的识别与正确应对",
    origin: "牙科",
    tags: ["健康", "牙龈", "出血", "口腔"]
}

impl GumBleedingConcernRules {
    /// 常见原因
    pub fn cause(&self) -> Vec<&'static str> {
        vec!["刷力过大易出血", "牙龈炎常见", "牙结石刺激", "激素变化影响"]
    }

    /// 正确做法
    pub fn handle(&self) -> Vec<&'static str> {
        vec!["不因出血少刷", "轻柔刷净", "止血用冷敷", "坚持清洁"]
    }

    /// 就医判断
    pub fn visit(&self) -> Vec<&'static str> {
        vec!["持续出血就医", "洗牙除结石", "牙周治疗", "不拖延"]
    }

    /// 预防
    pub fn prevent(&self) -> Vec<&'static str> {
        vec!["正确刷牙", "定期洁牙", "均衡营养", "保牙龈健康"]
    }
}

impl Rule for GumBleedingConcernRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("gum_bleeding")
    }

    fn explain(&self) -> String {
        format!(
            "【牙龈出血处理】\n{}",
            [
                format!(
                    "常见原因：\\n{}",
                    self.cause()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "正确做法：\\n{}",
                    self.handle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医判断：\\n{}",
                    self.visit()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "预防：\\n{}",
                    self.prevent()
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
    fn test_gumbleedingconcernrules_basic() {
        let rules = GumBleedingConcernRules::new();
        assert_eq!(rules.metadata().name, "牙龈出血处理");
        assert!(!rules.cause().is_empty());
        assert!(!rules.handle().is_empty());
        assert!(!rules.visit().is_empty());
        assert!(!rules.prevent().is_empty());
    }

    #[test]
    fn test_gumbleedingconcernrules_validation() {
        let rules = GumBleedingConcernRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("gum_bleeding"));
    }

    #[test]
    fn test_gumbleedingconcernrules_explain() {
        let rules = GumBleedingConcernRules::new();
        let e = rules.explain();
        assert!(e.contains("常见原因"));
        assert!(e.contains("正确做法"));
        assert!(e.contains("就医判断"));
    }
}
