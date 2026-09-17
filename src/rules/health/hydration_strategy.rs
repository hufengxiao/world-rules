//! 合理补水策略
//!
//! 日常与运动时合理补水的原则与电解质平衡

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HydrationStrategyRules,
    name: "合理补水策略",
    desc: "日常与运动时合理补水的原则与电解质平衡",
    origin: "国际",
    tags: ["健康", "补水", "水分", "电解质", "运动"]
}

impl HydrationStrategyRules {
    /// 日常补水
    pub fn daily(&self) -> Vec<&'static str> {
        vec![
            "按身体状况适量饮水",
            "不能等口渴才饮",
            "分散在一天中少量多次",
            "观察尿液颜色判断水分",
        ]
    }

    /// 运动补水
    pub fn exercise(&self) -> Vec<&'static str> {
        vec![
            "运动前后补充水分",
            "长时间运动适度补电解质",
            "不要一次性大量猛灌",
            "结合环境温度判断",
        ]
    }

    /// 特殊情况
    pub fn special(&self) -> Vec<&'static str> {
        vec![
            "炎热运动注意防脱水",
            "呕吐腹泻及时补液",
            "饮水量受疾病影响者遵医嘱",
            "老人儿童注意主动补水",
        ]
    }

    /// 平衡原则
    pub fn balance(&self) -> Vec<&'static str> {
        vec![
            "过度饮水同样不适",
            "注意电解质均衡",
            "以白开水为主",
            "减少高糖饮品摄取",
        ]
    }
}

impl Rule for HydrationStrategyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("hydration")
    }

    fn explain(&self) -> String {
        format!(
            "【合理补水策略】\n{}",
            [
                format!(
                    "日常补水：\\n{}",
                    self.daily()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "运动补水：\\n{}",
                    self.exercise()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "特殊情况：\\n{}",
                    self.special()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "平衡原则：\\n{}",
                    self.balance()
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
    fn test_hydrationstrategyrules_basic() {
        let rules = HydrationStrategyRules::new();
        assert_eq!(rules.metadata().name, "合理补水策略");
        assert!(!rules.daily().is_empty());
        assert!(!rules.exercise().is_empty());
        assert!(!rules.special().is_empty());
        assert!(!rules.balance().is_empty());
    }

    #[test]
    fn test_hydrationstrategyrules_validation() {
        let rules = HydrationStrategyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("hydration"));
    }

    #[test]
    fn test_hydrationstrategyrules_explain() {
        let rules = HydrationStrategyRules::new();
        let e = rules.explain();
        assert!(e.contains("日常补水"));
        assert!(e.contains("运动补水"));
        assert!(e.contains("特殊情况"));
    }
}
