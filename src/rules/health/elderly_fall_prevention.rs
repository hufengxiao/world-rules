//! 老人防跌倒
//!
//! 老人环境防跌、步态与骨骼保护措施

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ElderlyFallPreventionRules,
    name: "老人防跌倒",
    desc: "老人环境防跌、步态与骨骼保护措施",
    origin: "医学",
    tags: ["健康", "防跌倒", "老人", "安全"]
}

impl ElderlyFallPreventionRules {
    /// 环境安全
    pub fn home(&self) -> Vec<&'static str> {
        vec![
            "保持地面干燥防滑",
            "清除通道障碍",
            "床边夜灯充足",
            "扶手浴卫增设",
        ]
    }

    /// 步态增强
    pub fn walk(&self) -> Vec<&'static str> {
        vec!["适度运动强肌力", "扶稳慢走少急", "穿防滑鞋", "起身放缓站立"]
    }

    /// 视力与用药
    pub fn vision(&self) -> Vec<&'static str> {
        vec![
            "视力障碍配镜",
            "防血压血糖波动",
            "遵医嘱用药防眩晕",
            "眩晕时暂别独行",
        ]
    }

    /// 跌倒应对
    pub fn response(&self) -> Vec<&'static str> {
        vec!["跌倒后不勉强起身", "呼救求助", "受伤立即就医", "排查骨疏松"]
    }
}

impl Rule for ElderlyFallPreventionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("fall_prevention")
    }

    fn explain(&self) -> String {
        format!(
            "【老人防跌倒】\n{}",
            [
                format!(
                    "环境安全：\\n{}",
                    self.home()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "步态增强：\\n{}",
                    self.walk()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "视力与用药：\\n{}",
                    self.vision()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "跌倒应对：\\n{}",
                    self.response()
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
    fn test_elderlyfallpreventionrules_basic() {
        let rules = ElderlyFallPreventionRules::new();
        assert_eq!(rules.metadata().name, "老人防跌倒");
        assert!(!rules.home().is_empty());
        assert!(!rules.walk().is_empty());
        assert!(!rules.vision().is_empty());
        assert!(!rules.response().is_empty());
    }

    #[test]
    fn test_elderlyfallpreventionrules_validation() {
        let rules = ElderlyFallPreventionRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("fall_prevention"));
    }

    #[test]
    fn test_elderlyfallpreventionrules_explain() {
        let rules = ElderlyFallPreventionRules::new();
        let e = rules.explain();
        assert!(e.contains("环境安全"));
        assert!(e.contains("步态增强"));
        assert!(e.contains("视力与用药"));
    }
}
