//! 伴郎伴娘职责
//!
//! 伴郎伴娘协助新人、打理仪式的职责

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BridesmaidBestmanDutyRules,
    name: "伴郎伴娘职责",
    desc: "伴郎伴娘协助新人、打理仪式的职责",
    origin: "中国",
    tags: ["社交", "伴郎", "伴娘", "婚礼"]
}

impl BridesmaidBestmanDutyRules {
    /// 事前准备
    pub fn prepare(&self) -> Vec<&'static str> {
        vec!["帮忙筹备细节", "熟悉流程", "备好随身物", "配合彩排"]
    }

    /// 仪式协助
    pub fn assist(&self) -> Vec<&'static str> {
        vec!["递戒指引路", "整理裙摆", "稳住新人节奏", "即时补位"]
    }

    /// 应酬挡驾
    pub fn host(&self) -> Vec<&'static str> {
        vec!["替新人收红包", "招呼宾客", "挡酒适度", "维护氛围"]
    }

    /// 收尾善后
    pub fn wrap(&self) -> Vec<&'static str> {
        vec!["帮忙收拾物品", "清点礼金物", "护送交接", "圆满落幕"]
    }
}

impl Rule for BridesmaidBestmanDutyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("bridesmaid")
    }

    fn explain(&self) -> String {
        format!(
            "【伴郎伴娘职责】\n{}",
            [
                format!(
                    "事前准备：\\n{}",
                    self.prepare()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "仪式协助：\\n{}",
                    self.assist()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "应酬挡驾：\\n{}",
                    self.host()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "收尾善后：\\n{}",
                    self.wrap()
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
    fn test_bridesmaidbestmandutyrules_basic() {
        let rules = BridesmaidBestmanDutyRules::new();
        assert_eq!(rules.metadata().name, "伴郎伴娘职责");
        assert!(!rules.prepare().is_empty());
        assert!(!rules.assist().is_empty());
        assert!(!rules.host().is_empty());
        assert!(!rules.wrap().is_empty());
    }

    #[test]
    fn test_bridesmaidbestmandutyrules_validation() {
        let rules = BridesmaidBestmanDutyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("bridesmaid"));
    }

    #[test]
    fn test_bridesmaidbestmandutyrules_explain() {
        let rules = BridesmaidBestmanDutyRules::new();
        let e = rules.explain();
        assert!(e.contains("事前准备"));
        assert!(e.contains("仪式协助"));
        assert!(e.contains("应酬挡驾"));
    }
}
