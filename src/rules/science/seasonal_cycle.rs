//! 四季更替知识
//!
//! 地球公转与四季成因、节气知识的科学常识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SeasonalCycleRules,
    name: "四季更替知识",
    desc: "地球公转与四季成因、节气知识的科学常识",
    origin: "国际",
    tags: ["科学", "四季", "节气", "地球"]
}

impl SeasonalCycleRules {
    /// 四季成因
    pub fn cause(&self) -> Vec<&'static str> {
        vec![
            "地球绕日公转",
            "地轴倾斜造成季节",
            "阳光直射点北移南移",
            "四季交替循环",
        ]
    }

    /// 昼夜变化
    pub fn day_night(&self) -> Vec<&'static str> {
        vec![
            "昼夜由自转决定",
            "夏至白昼最长",
            "冬至白昼最短",
            "春秋分昼夜等长",
        ]
    }

    /// 节气民俗
    pub fn solar_term(&self) -> Vec<&'static str> {
        vec![
            "二十四节气划分时令",
            "反映气候变化",
            "指导农事生活",
            "传统智慧科学印证",
        ]
    }

    /// 季节适应
    pub fn adapt(&self) -> Vec<&'static str> {
        vec![
            "随季节调整作息",
            "季节变化注意健康",
            "合理应对温差",
            "尊重自然规律",
        ]
    }
}

impl Rule for SeasonalCycleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("seasonal")
    }

    fn explain(&self) -> String {
        format!(
            "【四季更替知识】\n{}",
            [
                format!(
                    "四季成因：\\n{}",
                    self.cause()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "昼夜变化：\\n{}",
                    self.day_night()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "节气民俗：\\n{}",
                    self.solar_term()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "季节适应：\\n{}",
                    self.adapt()
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
    fn test_seasonalcyclerules_basic() {
        let rules = SeasonalCycleRules::new();
        assert_eq!(rules.metadata().name, "四季更替知识");
        assert!(!rules.cause().is_empty());
        assert!(!rules.day_night().is_empty());
        assert!(!rules.solar_term().is_empty());
        assert!(!rules.adapt().is_empty());
    }

    #[test]
    fn test_seasonalcyclerules_validation() {
        let rules = SeasonalCycleRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("seasonal"));
    }

    #[test]
    fn test_seasonalcyclerules_explain() {
        let rules = SeasonalCycleRules::new();
        let e = rules.explain();
        assert!(e.contains("四季成因"));
        assert!(e.contains("昼夜变化"));
        assert!(e.contains("节气民俗"));
    }
}
