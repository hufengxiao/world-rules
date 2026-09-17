//! 地震震级烈度
//!
//! 地震震级、烈度与避险知识的理解

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: EarthquakeMagnitudeScaleRules,
    name: "地震震级烈度",
    desc: "地震震级、烈度与避险知识的理解",
    origin: "国际",
    tags: ["科学", "地震", "震级", "安全"]
}

impl EarthquakeMagnitudeScaleRules {
    /// 震级概念
    pub fn magnitude(&self) -> Vec<&'static str> {
        vec![
            "震级衡量地震能量",
            "每差一级差几十倍",
            "数值越高越强",
            "如实记录",
        ]
    }

    /// 烈度区别
    pub fn intensity(&self) -> Vec<&'static str> {
        vec![
            "烈度衡量破坏程度",
            "同一地震各地不同",
            "离震中远近不同",
            "有感差异",
        ]
    }

    /// 减少误传
    pub fn avoid(&self) -> Vec<&'static str> {
        vec!["认准官方消息", "不传夸张谣言", "了解逃生避险", "科学认识"]
    }

    /// 应急避险
    pub fn prepare(&self) -> Vec<&'static str> {
        vec![
            "震时找结实地",
            "室内蔽身桌下",
            "远离玻璃重物",
            "震后有序撤离",
        ]
    }
}

impl Rule for EarthquakeMagnitudeScaleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("earthquake")
    }

    fn explain(&self) -> String {
        format!(
            "【地震震级烈度】\n{}",
            [
                format!(
                    "震级概念：\\n{}",
                    self.magnitude()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "烈度区别：\\n{}",
                    self.intensity()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "减少误传：\\n{}",
                    self.avoid()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "应急避险：\\n{}",
                    self.prepare()
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
    fn test_earthquakemagnitudescalerules_basic() {
        let rules = EarthquakeMagnitudeScaleRules::new();
        assert_eq!(rules.metadata().name, "地震震级烈度");
        assert!(!rules.magnitude().is_empty());
        assert!(!rules.intensity().is_empty());
        assert!(!rules.avoid().is_empty());
        assert!(!rules.prepare().is_empty());
    }

    #[test]
    fn test_earthquakemagnitudescalerules_validation() {
        let rules = EarthquakeMagnitudeScaleRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("earthquake"));
    }

    #[test]
    fn test_earthquakemagnitudescalerules_explain() {
        let rules = EarthquakeMagnitudeScaleRules::new();
        let e = rules.explain();
        assert!(e.contains("震级概念"));
        assert!(e.contains("烈度区别"));
        assert!(e.contains("减少误传"));
    }
}
