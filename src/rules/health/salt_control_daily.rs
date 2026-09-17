//! 控盐少钠
//!
//! 每日食盐限量、隐形盐与低钠饮食

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SaltControlDailyRules,
    name: "控盐少钠",
    desc: "每日食盐限量、隐形盐与低钠饮食",
    origin: "营养学",
    tags: ["健康", "控盐", "钠", "血压"]
}

impl SaltControlDailyRules {
    /// 限量标准
    pub fn limit(&self) -> Vec<&'static str> {
        vec!["成人每日盐适量", "不过多摄盐", "少用味精酱油", "计算入量"]
    }

    /// 留意咸码
    pub fn hidden(&self) -> Vec<&'static str> {
        vec!["腌制品含钠高", "加工食品多盐", "酱料要少放", "看营养成分表"]
    }

    /// 减盐技巧
    pub fn reduce(&self) -> Vec<&'static str> {
        vec!["用香料提味", "起锅再加盐", "少吃咸菜下酒", "清淡烹调"]
    }

    /// 健康益处
    pub fn benefit(&self) -> Vec<&'static str> {
        vec!["减盐助控血压", "护心护肾", "少水肿", "养成清淡习惯"]
    }
}

impl Rule for SaltControlDailyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("salt_control")
    }

    fn explain(&self) -> String {
        format!(
            "【控盐少钠】\n{}",
            [
                format!(
                    "限量标准：\\n{}",
                    self.limit()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "留意咸码：\\n{}",
                    self.hidden()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "减盐技巧：\\n{}",
                    self.reduce()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "健康益处：\\n{}",
                    self.benefit()
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
    fn test_saltcontroldailyrules_basic() {
        let rules = SaltControlDailyRules::new();
        assert_eq!(rules.metadata().name, "控盐少钠");
        assert!(!rules.limit().is_empty());
        assert!(!rules.hidden().is_empty());
        assert!(!rules.reduce().is_empty());
        assert!(!rules.benefit().is_empty());
    }

    #[test]
    fn test_saltcontroldailyrules_validation() {
        let rules = SaltControlDailyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("salt_control"));
    }

    #[test]
    fn test_saltcontroldailyrules_explain() {
        let rules = SaltControlDailyRules::new();
        let e = rules.explain();
        assert!(e.contains("限量标准"));
        assert!(e.contains("留意咸码"));
        assert!(e.contains("减盐技巧"));
    }
}
