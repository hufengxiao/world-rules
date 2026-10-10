//! 产后抑郁应对
//!
//! 产后情绪低落与抑郁的识别与支持方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PostpartumDepressionHelpRules,
    name: "产后抑郁应对",
    desc: "产后情绪低落与抑郁的识别与支持方法",
    origin: "中国",
    tags: ["孕产", "心理", "女性"]
}

impl PostpartumDepressionHelpRules {
    /// 症状识别
    pub fn symptoms(&self) -> Vec<&'static str> {
        vec![
            "持续情绪低落易哭",
            "对婴儿缺乏兴趣",
            "失眠或过度嗜睡",
            "自责无助感强烈",
        ]
    }

    /// 家人支持
    pub fn family_support(&self) -> Vec<&'static str> {
        vec![
            "分担育儿与家务",
            "多倾听少指责",
            "关注产妇情绪变化",
            "给予理解与陪伴",
        ]
    }

    /// 自我调节
    pub fn self_care(&self) -> Vec<&'static str> {
        vec![
            "保证充足休息",
            "适度户外活动晒太阳",
            "与亲友倾诉交流",
            "接纳情绪不苛责自己",
        ]
    }

    /// 专业帮助
    pub fn professional(&self) -> Vec<&'static str> {
        vec![
            "症状持续两周应求助",
            "心理疏导有效",
            "必要时遵医嘱用药",
            "出现伤害念头立即就医",
        ]
    }
}

impl Rule for PostpartumDepressionHelpRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("postpartum_depression_help")
    }

    fn explain(&self) -> String {
        format!(
            "【产后抑郁应对】\n{}",
            [
                format!(
                    "症状识别：\\n{}",
                    self.symptoms()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "家人支持：\\n{}",
                    self.family_support()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "自我调节：\\n{}",
                    self.self_care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "专业帮助：\\n{}",
                    self.professional()
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
    fn test_postpartumdepressionhelprules_basic() {
        let rules = PostpartumDepressionHelpRules::new();
        assert_eq!(rules.metadata().name, "产后抑郁应对");
        assert!(!rules.symptoms().is_empty());
        assert!(!rules.family_support().is_empty());
        assert!(!rules.self_care().is_empty());
        assert!(!rules.professional().is_empty());
    }

    #[test]
    fn test_postpartumdepressionhelprules_validation() {
        let rules = PostpartumDepressionHelpRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::health("postpartum_depression_help")
        );
    }

    #[test]
    fn test_postpartumdepressionhelprules_explain() {
        let rules = PostpartumDepressionHelpRules::new();
        let e = rules.explain();
        assert!(e.contains("症状识别"));
        assert!(e.contains("家人支持"));
        assert!(e.contains("自我调节"));
    }
}
