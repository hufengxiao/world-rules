//! 视屏用眼距离
//!
//! 看屏幕的距离、高度与护眼调节

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ScreenEyeDistanceRuleRules,
    name: "视屏用眼距离",
    desc: "看屏幕的距离、高度与护眼调节",
    origin: "医学",
    tags: ["健康", "屏幕", "用眼", "距离"]
}

impl ScreenEyeDistanceRuleRules {
    /// 合理距离
    pub fn distance(&self) -> Vec<&'static str> {
        vec!["屏幕距眼五十厘米", "约一臂长", "手机稍远", "过近伤眼"]
    }

    /// 高度角度
    pub fn angle(&self) -> Vec<&'static str> {
        vec!["屏幕略低于眼平", "俯视约十度", "不仰视", "低头勿久"]
    }

    /// 短暂调节
    pub fn rest(&self) -> Vec<&'static str> {
        vec!["注意频繁眨眼", "20-20-20规则遵", "勤远眺", "防视疲劳"]
    }

    /// 环境光亮
    pub fn light(&self) -> Vec<&'static str> {
        vec!["光线柔和免反光", "不过暗过亮", "屏幕调适", "舒适用眼"]
    }
}

impl Rule for ScreenEyeDistanceRuleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("screen_eye")
    }

    fn explain(&self) -> String {
        format!(
            "【视屏用眼距离】\n{}",
            [
                format!(
                    "合理距离：\\n{}",
                    self.distance()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "高度角度：\\n{}",
                    self.angle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "短暂调节：\\n{}",
                    self.rest()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "环境光亮：\\n{}",
                    self.light()
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
    fn test_screeneyedistancerulerules_basic() {
        let rules = ScreenEyeDistanceRuleRules::new();
        assert_eq!(rules.metadata().name, "视屏用眼距离");
        assert!(!rules.distance().is_empty());
        assert!(!rules.angle().is_empty());
        assert!(!rules.rest().is_empty());
        assert!(!rules.light().is_empty());
    }

    #[test]
    fn test_screeneyedistancerulerules_validation() {
        let rules = ScreenEyeDistanceRuleRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("screen_eye"));
    }

    #[test]
    fn test_screeneyedistancerulerules_explain() {
        let rules = ScreenEyeDistanceRuleRules::new();
        let e = rules.explain();
        assert!(e.contains("合理距离"));
        assert!(e.contains("高度角度"));
        assert!(e.contains("短暂调节"));
    }
}
