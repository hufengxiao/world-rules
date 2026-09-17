//! 情绪低落与抑郁求助
//!
//! 识别抑郁信号、自我关怀与及时求助的支持规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DepressionSupportRules,
    name: "情绪低落与抑郁求助",
    desc: "识别抑郁信号、自我关怀与及时求助的支持规则",
    origin: "心理学",
    tags: ["健康", "抑郁", "情绪", "求助", "支持"]
}

impl DepressionSupportRules {
    /// 识别信号
    pub fn signs(&self) -> Vec<&'static str> {
        vec![
            "关注持续情绪低落",
            "留意兴趣减退与疲乏",
            "注意睡眠食欲变化",
            "留意自我贬低念头",
        ]
    }

    /// 自我关怀
    pub fn self_care(&self) -> Vec<&'static str> {
        vec![
            "不苛责自己情绪",
            "保持基本作息与进食",
            "做较小的可完成之事",
            "避免自我孤立",
        ]
    }

    /// 寻求支持
    pub fn seek(&self) -> Vec<&'static str> {
        vec![
            "向信任的人诉说",
            "主动寻求专业咨询",
            "不因羞耻而拖延就医",
            "家人朋友给予理解和陪伴",
        ]
    }

    /// 紧急求助
    pub fn crisis(&self) -> Vec<&'static str> {
        vec![
            "有伤害自己念头立即求助",
            "拨打心理求助热线",
            "告知身边人陪伴",
            "寻求危机干预或就医",
        ]
    }
}

impl Rule for DepressionSupportRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("depression_support")
    }

    fn explain(&self) -> String {
        format!(
            "【情绪低落与抑郁求助】\n{}",
            [
                format!(
                    "识别信号：\\n{}",
                    self.signs()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "自我关怀：\\n{}",
                    self.self_care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "寻求支持：\\n{}",
                    self.seek()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "紧急求助：\\n{}",
                    self.crisis()
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
    fn test_depressionsupportrules_basic() {
        let rules = DepressionSupportRules::new();
        assert_eq!(rules.metadata().name, "情绪低落与抑郁求助");
        assert!(!rules.signs().is_empty());
        assert!(!rules.self_care().is_empty());
        assert!(!rules.seek().is_empty());
        assert!(!rules.crisis().is_empty());
    }

    #[test]
    fn test_depressionsupportrules_validation() {
        let rules = DepressionSupportRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("depression_support"));
    }

    #[test]
    fn test_depressionsupportrules_explain() {
        let rules = DepressionSupportRules::new();
        let e = rules.explain();
        assert!(e.contains("识别信号"));
        assert!(e.contains("自我关怀"));
        assert!(e.contains("寻求支持"));
    }
}
