//! 紧急报警求助
//!
//! 报警求助的时机、讲清要点与配合处置

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: EmergencyCall110Rules,
    name: "紧急报警求助",
    desc: "报警求助的时机、讲清要点与配合处置",
    origin: "中国",
    tags: ["法律", "报警", "求助"]
}

impl EmergencyCall110Rules {
    /// 何时报警
    pub fn when(&self) -> Vec<&'static str> {
        vec![
            "遇到违法犯罪可报警",
            "紧急危险即时求助",
            "目击违法行为可报告",
            "不虚报警情",
        ]
    }

    /// 讲清要点
    pub fn details(&self) -> Vec<&'static str> {
        vec![
            "说清时间地点",
            "简述经过与人身情况",
            "留下联系电话",
            "不夸大不隐瞒",
        ]
    }

    /// 现场配合
    pub fn cooperate(&self) -> Vec<&'static str> {
        vec!["听从接警指引", "在安全处等待", "保护好现场", "如实配合调查"]
    }

    /// 正确使用
    pub fn proper(&self) -> Vec<&'static str> {
        vec![
            "报警电话不随意乱拨",
            "不报假警耗费资源",
            "有疑问可咨询求助",
            "文明配合处置",
        ]
    }
}

impl Rule for EmergencyCall110Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("emergency_call")
    }

    fn explain(&self) -> String {
        format!(
            "【紧急报警求助】\n{}",
            [
                format!(
                    "何时报警：\\n{}",
                    self.when()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "讲清要点：\\n{}",
                    self.details()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "现场配合：\\n{}",
                    self.cooperate()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "正确使用：\\n{}",
                    self.proper()
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
    fn test_emergencycall110rules_basic() {
        let rules = EmergencyCall110Rules::new();
        assert_eq!(rules.metadata().name, "紧急报警求助");
        assert!(!rules.when().is_empty());
        assert!(!rules.details().is_empty());
        assert!(!rules.cooperate().is_empty());
        assert!(!rules.proper().is_empty());
    }

    #[test]
    fn test_emergencycall110rules_validation() {
        let rules = EmergencyCall110Rules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("emergency_call"));
    }

    #[test]
    fn test_emergencycall110rules_explain() {
        let rules = EmergencyCall110Rules::new();
        let e = rules.explain();
        assert!(e.contains("何时报警"));
        assert!(e.contains("讲清要点"));
        assert!(e.contains("现场配合"));
    }
}
