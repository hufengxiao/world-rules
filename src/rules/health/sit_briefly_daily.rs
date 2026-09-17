//! 久坐小憩活动
//!
//! 办公久坐的定时起身与微活动

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SitBreaksDailyRules,
    name: "久坐小憩活动",
    desc: "办公久坐的定时起身与微活动",
    origin: "医学",
    tags: ["健康", "久坐", "活动", "办公"]
}

impl SitBreaksDailyRules {
    /// 定时起身
    pub fn stand(&self) -> Vec<&'static str> {
        vec![
            "约四十分起身一次",
            "站起来走动几下",
            "提高桌面站立办公",
            "缓身不僵硬",
        ]
    }

    /// 微活动
    pub fn micro(&self) -> Vec<&'static str> {
        vec!["活动肩颈手腕", "转动腰胯松松", "原地踏步几圈", "伸个懒腰"]
    }

    /// 姿势调整
    pub fn posture(&self) -> Vec<&'static str> {
        vec!["坐直背靠腰", "双脚平放地面", "屏幕与眼平", "换姿缓解僵硬"]
    }

    /// 生活补动
    pub fn life(&self) -> Vec<&'static str> {
        vec!["上下多走楼梯", "午间散步走动", "周末运动补足", "动静结合"]
    }
}

impl Rule for SitBreaksDailyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("sit_break")
    }

    fn explain(&self) -> String {
        format!(
            "【久坐小憩活动】\n{}",
            [
                format!(
                    "定时起身：\\n{}",
                    self.stand()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "微活动：\\n{}",
                    self.micro()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "姿势调整：\\n{}",
                    self.posture()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活补动：\\n{}",
                    self.life()
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
    fn test_sitbreaksdailyrules_basic() {
        let rules = SitBreaksDailyRules::new();
        assert_eq!(rules.metadata().name, "久坐小憩活动");
        assert!(!rules.stand().is_empty());
        assert!(!rules.micro().is_empty());
        assert!(!rules.posture().is_empty());
        assert!(!rules.life().is_empty());
    }

    #[test]
    fn test_sitbreaksdailyrules_validation() {
        let rules = SitBreaksDailyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("sit_break"));
    }

    #[test]
    fn test_sitbreaksdailyrules_explain() {
        let rules = SitBreaksDailyRules::new();
        let e = rules.explain();
        assert!(e.contains("定时起身"));
        assert!(e.contains("微活动"));
        assert!(e.contains("姿势调整"));
    }
}
