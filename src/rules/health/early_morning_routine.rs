//! 早起晨间习惯
//!
//! 晨间起床、光亮与规律作息的调整方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: EarlyMorningRoutineRules,
    name: "早起晨间习惯",
    desc: "晨间起床、光亮与规律作息的调整方法",
    origin: "医学",
    tags: ["健康", "早起", "晨间", "习惯"]
}

impl EarlyMorningRoutineRules {
    /// 按时起床
    pub fn wake(&self) -> Vec<&'static str> {
        vec![
            "定时起床固定时间",
            "睡醒即起不赖床",
            "渐进调整早起",
            "不过度用闹钟赖",
        ]
    }

    /// 晨光唤醒
    pub fn light(&self) -> Vec<&'static str> {
        vec![
            "起床拉开窗帘见光",
            "自然光调节节律",
            "不长时间躺刷",
            "活动舒展身体",
        ]
    }

    /// 启动晨间
    pub fn morning(&self) -> Vec<&'static str> {
        vec!["简单洗漱饮水", "可做轻度拉伸", "规划今日要事", "轻松用早餐"]
    }

    /// 调整节律
    pub fn rhythm(&self) -> Vec<&'static str> {
        vec![
            "保持睡眠充足",
            "睡前不兴奋",
            "规律作息利早起",
            "循序渐进最稳",
        ]
    }
}

impl Rule for EarlyMorningRoutineRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("morning_routine")
    }

    fn explain(&self) -> String {
        format!(
            "【早起晨间习惯】\n{}",
            [
                format!(
                    "按时起床：\\n{}",
                    self.wake()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "晨光唤醒：\\n{}",
                    self.light()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "启动晨间：\\n{}",
                    self.morning()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "调整节律：\\n{}",
                    self.rhythm()
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
    fn test_earlymorningroutinerules_basic() {
        let rules = EarlyMorningRoutineRules::new();
        assert_eq!(rules.metadata().name, "早起晨间习惯");
        assert!(!rules.wake().is_empty());
        assert!(!rules.light().is_empty());
        assert!(!rules.morning().is_empty());
        assert!(!rules.rhythm().is_empty());
    }

    #[test]
    fn test_earlymorningroutinerules_validation() {
        let rules = EarlyMorningRoutineRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("morning_routine"));
    }

    #[test]
    fn test_earlymorningroutinerules_explain() {
        let rules = EarlyMorningRoutineRules::new();
        let e = rules.explain();
        assert!(e.contains("按时起床"));
        assert!(e.contains("晨光唤醒"));
        assert!(e.contains("启动晨间"));
    }
}
