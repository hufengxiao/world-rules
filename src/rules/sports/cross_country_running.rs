//! 越野跑步基础
//!
//! 山地越野跑的体能、补给与安全基础

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CrossCountryRunningRules,
    name: "越野跑步基础",
    desc: "山地越野跑的体能、补给与安全基础",
    origin: "国际",
    tags: ["体育", "越野", "跑步", "户外"]
}

impl CrossCountryRunningRules {
    /// 赛前准备
    pub fn prepare(&self) -> Vec<&'static str> {
        vec![
            "了解路线地形",
            "准备适宜鞋服",
            "携带足量饮水补给",
            "告知行程做好热身",
        ]
    }

    /// 途中节奏
    pub fn pace(&self) -> Vec<&'static str> {
        vec![
            "合理分配体力",
            "上下坡调节步伐",
            "注意落脚安全",
            "按自身配速不怕慢",
        ]
    }

    /// 补给安全
    pub fn supply(&self) -> Vec<&'static str> {
        vec![
            "及时补水补充能量",
            "留意中暑失温信号",
            "不独自野练",
            "紧急情况求助",
        ]
    }

    /// 环保礼仪
    pub fn etiquette(&self) -> Vec<&'static str> {
        vec![
            "不随意丢弃垃圾",
            "礼让其他赛友",
            "注意脚下生态",
            "赛后良好恢复",
        ]
    }
}

impl Rule for CrossCountryRunningRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("cross_country")
    }

    fn explain(&self) -> String {
        format!(
            "【越野跑步基础】\n{}",
            [
                format!(
                    "赛前准备：\\n{}",
                    self.prepare()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "途中节奏：\\n{}",
                    self.pace()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "补给安全：\\n{}",
                    self.supply()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "环保礼仪：\\n{}",
                    self.etiquette()
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
    fn test_crosscountryrunningrules_basic() {
        let rules = CrossCountryRunningRules::new();
        assert_eq!(rules.metadata().name, "越野跑步基础");
        assert!(!rules.prepare().is_empty());
        assert!(!rules.pace().is_empty());
        assert!(!rules.supply().is_empty());
        assert!(!rules.etiquette().is_empty());
    }

    #[test]
    fn test_crosscountryrunningrules_validation() {
        let rules = CrossCountryRunningRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("cross_country"));
    }

    #[test]
    fn test_crosscountryrunningrules_explain() {
        let rules = CrossCountryRunningRules::new();
        let e = rules.explain();
        assert!(e.contains("赛前准备"));
        assert!(e.contains("途中节奏"));
        assert!(e.contains("补给安全"));
    }
}
