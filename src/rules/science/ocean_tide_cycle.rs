//! 潮汐涨落规律
//!
//! 潮汐现象的成因与规律，包括月球与太阳引力作用、大潮小潮、潮汐周期与对沿海生活的影响。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: OceanTideCycleRules,
    name: "潮汐涨落规律",
    desc: "潮汐现象的成因与规律，包括月球与太阳引力作用、大潮小潮、潮汐周期与对沿海生活的影响。",
    origin: "海洋学与天体引力理论",
    tags: ["科学", "海洋", "天文", "自然现象"]
}

impl OceanTideCycleRules {
    /// 成因原理
    pub fn cause(&self) -> Vec<&'static str> {
        vec![
            "月球引力牵引海水形成潮汐",
            "太阳引力叠加作用",
            "地球自转导致潮位周期变化",
            "引潮力与距离立方成反比",
        ]
    }

    /// 潮汐类型
    pub fn types(&self) -> Vec<&'static str> {
        vec![
            "日月连线时形成大潮",
            "日月成直角时形成小潮",
            "半日潮一日两次涨落",
            "全日潮一日一次涨落",
        ]
    }

    /// 周期规律
    pub fn cycle(&self) -> Vec<&'static str> {
        vec![
            "潮汐周期约十二小时二十五分",
            "每日潮时逐渐推迟约五十分钟",
            "朔望月内潮差规律变化",
            "地形影响实际潮时与潮差",
        ]
    }

    /// 实际影响
    pub fn impact(&self) -> Vec<&'static str> {
        vec![
            "渔船依潮汐进出港口",
            "赶海需掌握退潮时间",
            "潮间带生物随潮活动",
            "沿海工程需考虑潮位设计",
        ]
    }
}

impl Rule for OceanTideCycleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("tide")
    }

    fn explain(&self) -> String {
        format!(
            "【潮汐涨落规律】\n{}",
            [
                format!(
                    "成因原理：\\n{}",
                    self.cause()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "潮汐类型：\\n{}",
                    self.types()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "周期规律：\\n{}",
                    self.cycle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "实际影响：\\n{}",
                    self.impact()
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
    fn test_oceantidecyclerules_basic() {
        let rules = OceanTideCycleRules::new();
        assert_eq!(rules.metadata().name, "潮汐涨落规律");
        assert!(!rules.cause().is_empty());
        assert!(!rules.types().is_empty());
        assert!(!rules.cycle().is_empty());
        assert!(!rules.impact().is_empty());
    }

    #[test]
    fn test_oceantidecyclerules_validation() {
        let rules = OceanTideCycleRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("tide"));
    }

    #[test]
    fn test_oceantidecyclerules_explain() {
        let rules = OceanTideCycleRules::new();
        let e = rules.explain();
        assert!(e.contains("成因原理"));
        assert!(e.contains("潮汐类型"));
        assert!(e.contains("周期规律"));
    }
}
