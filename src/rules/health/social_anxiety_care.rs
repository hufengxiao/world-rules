//! 社交焦虑应对
//!
//! 社交场合紧张不安的自我调节与改善方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SocialAnxietyCareRules,
    name: "社交焦虑应对",
    desc: "社交场合紧张不安的自我调节与改善方法",
    origin: "中国",
    tags: ["心理", "焦虑", "社交"]
}

impl SocialAnxietyCareRules {
    /// 认识症状
    pub fn symptoms(&self) -> Vec<&'static str> {
        vec![
            "社交前过度担忧",
            "脸红出汗心跳加速",
            "害怕被评判注视",
            "回避社交场合",
        ]
    }

    /// 认知调整
    pub fn cognitive(&self) -> Vec<&'static str> {
        vec![
            "识别不合理想法",
            "他人未必如此关注你",
            "接纳紧张不抗拒",
            "降低完美期待",
        ]
    }

    /// 行为练习
    pub fn behavior(&self) -> Vec<&'static str> {
        vec![
            "循序渐进暴露练习",
            "从小场合开始",
            "提前准备话题",
            "练习眼神与微笑",
        ]
    }

    /// 求助建议
    pub fn seeking_help(&self) -> Vec<&'static str> {
        vec![
            "严重影响生活应求助",
            "心理咨询有效",
            "必要时遵医嘱用药",
            "加入支持群体",
        ]
    }
}

impl Rule for SocialAnxietyCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("social_anxiety_care")
    }

    fn explain(&self) -> String {
        format!(
            "【社交焦虑应对】\n{}",
            [
                format!(
                    "认识症状：\\n{}",
                    self.symptoms()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "认知调整：\\n{}",
                    self.cognitive()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "行为练习：\\n{}",
                    self.behavior()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "求助建议：\\n{}",
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
    fn test_socialanxietycarerules_basic() {
        let rules = SocialAnxietyCareRules::new();
        assert_eq!(rules.metadata().name, "社交焦虑应对");
        assert!(!rules.symptoms().is_empty());
        assert!(!rules.cognitive().is_empty());
        assert!(!rules.behavior().is_empty());
        assert!(!rules.seeking_help().is_empty());
    }

    #[test]
    fn test_socialanxietycarerules_validation() {
        let rules = SocialAnxietyCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::health("social_anxiety_care")
        );
    }

    #[test]
    fn test_socialanxietycarerules_explain() {
        let rules = SocialAnxietyCareRules::new();
        let e = rules.explain();
        assert!(e.contains("认识症状"));
        assert!(e.contains("认知调整"));
        assert!(e.contains("行为练习"));
    }
}
