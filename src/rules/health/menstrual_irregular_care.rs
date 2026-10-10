//! 月经不调调理
//!
//! 月经周期紊乱的观察、调理与就医提示

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MenstrualIrregularCareRules,
    name: "月经不调调理",
    desc: "月经周期紊乱的观察、调理与就医提示",
    origin: "中国",
    tags: ["女性", "内分泌", "调理"]
}

impl MenstrualIrregularCareRules {
    /// 常见表现
    pub fn symptoms(&self) -> Vec<&'static str> {
        vec![
            "周期提前或推迟",
            "经量过多或过少",
            "经期延长或缩短",
            "痛经加重",
        ]
    }

    /// 生活调理
    pub fn lifestyle(&self) -> Vec<&'static str> {
        vec![
            "规律作息少熬夜",
            "适度运动控体重",
            "经期注意保暖",
            "减少精神压力",
        ]
    }

    /// 饮食调节
    pub fn diet(&self) -> Vec<&'static str> {
        vec![
            "均衡营养不节食",
            "适量补充铁质",
            "经期少食生冷",
            "控制高糖高脂",
        ]
    }

    /// 就医提示
    pub fn seeking_help(&self) -> Vec<&'static str> {
        vec![
            "周期长期紊乱应就诊",
            "经量异常须排查",
            "闭经应查明原因",
            "记录周期便于诊断",
        ]
    }
}

impl Rule for MenstrualIrregularCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("menstrual_irregular_care")
    }

    fn explain(&self) -> String {
        format!(
            "【月经不调调理】\n{}",
            [
                format!(
                    "常见表现：\\n{}",
                    self.symptoms()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活调理：\\n{}",
                    self.lifestyle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "饮食调节：\\n{}",
                    self.diet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医提示：\\n{}",
                    self.seeking_help()
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
    fn test_menstrualirregularcarerules_basic() {
        let rules = MenstrualIrregularCareRules::new();
        assert_eq!(rules.metadata().name, "月经不调调理");
        assert!(!rules.symptoms().is_empty());
        assert!(!rules.lifestyle().is_empty());
        assert!(!rules.diet().is_empty());
        assert!(!rules.seeking_help().is_empty());
    }

    #[test]
    fn test_menstrualirregularcarerules_validation() {
        let rules = MenstrualIrregularCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::health("menstrual_irregular_care")
        );
    }

    #[test]
    fn test_menstrualirregularcarerules_explain() {
        let rules = MenstrualIrregularCareRules::new();
        let e = rules.explain();
        assert!(e.contains("常见表现"));
        assert!(e.contains("生活调理"));
        assert!(e.contains("饮食调节"));
    }
}
