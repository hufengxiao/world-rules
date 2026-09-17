//! 日常补水
//!
//! 每日饮水量、时机与补水习惯

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DailyHydrationRules,
    name: "日常补水",
    desc: "每日饮水量、时机与补水习惯",
    origin: "医学",
    tags: ["健康", "补水", "喝水", "饮水"]
}

impl DailyHydrationRules {
    /// 饮水量
    pub fn amount(&self) -> Vec<&'static str> {
        vec![
            "成年人约每日适量水",
            "按活动天气调整",
            "少量多次补",
            "口渴及时喝",
        ]
    }

    /// 喝水时机
    pub fn timing(&self) -> Vec<&'static str> {
        vec!["晨起先喝温水", "餐前后适当", "运动后补充", "不等到口渴才喝"]
    }

    /// 正确饮水
    pub fn manner(&self) -> Vec<&'static str> {
        vec!["温水为宜", "小口慢饮", "不用高糖饮料替代", "淡茶果饮亦可"]
    }

    /// 注意人群
    pub fn special(&self) -> Vec<&'static str> {
        vec![
            "心肾功能异常遵医嘱",
            "儿童勤提醒",
            "老人提醒饮水",
            "高温户外补盐补水",
        ]
    }
}

impl Rule for DailyHydrationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("hydration")
    }

    fn explain(&self) -> String {
        format!(
            "【日常补水】\n{}",
            [
                format!(
                    "饮水量：\\n{}",
                    self.amount()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "喝水时机：\\n{}",
                    self.timing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "正确饮水：\\n{}",
                    self.manner()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "注意人群：\\n{}",
                    self.special()
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
    fn test_dailyhydrationrules_basic() {
        let rules = DailyHydrationRules::new();
        assert_eq!(rules.metadata().name, "日常补水");
        assert!(!rules.amount().is_empty());
        assert!(!rules.timing().is_empty());
        assert!(!rules.manner().is_empty());
        assert!(!rules.special().is_empty());
    }

    #[test]
    fn test_dailyhydrationrules_validation() {
        let rules = DailyHydrationRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("hydration"));
    }

    #[test]
    fn test_dailyhydrationrules_explain() {
        let rules = DailyHydrationRules::new();
        let e = rules.explain();
        assert!(e.contains("饮水量"));
        assert!(e.contains("喝水时机"));
        assert!(e.contains("正确饮水"));
    }
}
