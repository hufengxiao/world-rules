//! 打鼾的认识与缓解
//!
//! 打鼾原因、对睡眠的影响与缓解方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SnoringSleepBasicRules,
    name: "打鼾的认识与缓解",
    desc: "打鼾原因、对睡眠的影响与缓解方法",
    origin: "中国",
    tags: ["健康", "打鼾", "睡眠", "呼吸"]
}

impl SnoringSleepBasicRules {
    /// 成因认知
    pub fn cause(&self) -> Vec<&'static str> {
        vec!["咽喉气道变窄", "舌根后坠", "鼻塞压迫", "体型影响"]
    }

    /// 姿势调整
    pub fn position(&self) -> Vec<&'static str> {
        vec!["侧卧减轻后坠", "抬高枕部", "避免仰睡", "调整睡姿帮助"]
    }

    /// 生活习惯
    pub fn habit(&self) -> Vec<&'static str> {
        vec!["戒酒睡前少饮酒", "戒烟", "控制体重", "睡前不饱食"]
    }

    /// 就医指征
    pub fn seek_help(&self) -> Vec<&'static str> {
        vec![
            "呼吸暂停憋醒",
            "白天嗜睡乏力",
            "鼾声响雷惊醒他人",
            "及时就医检查",
        ]
    }
}

impl Rule for SnoringSleepBasicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("snoring_sleep")
    }

    fn explain(&self) -> String {
        format!(
            "【打鼾的认识与缓解】\n{}",
            [
                format!(
                    "成因认知：\\n{}",
                    self.cause()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "姿势调整：\\n{}",
                    self.position()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活习惯：\\n{}",
                    self.habit()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医指征：\\n{}",
                    self.seek_help()
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
    fn test_snoringsleepbasicrules_basic() {
        let rules = SnoringSleepBasicRules::new();
        assert_eq!(rules.metadata().name, "打鼾的认识与缓解");
        assert!(!rules.cause().is_empty());
        assert!(!rules.position().is_empty());
        assert!(!rules.habit().is_empty());
        assert!(!rules.seek_help().is_empty());
    }

    #[test]
    fn test_snoringsleepbasicrules_validation() {
        let rules = SnoringSleepBasicRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("snoring_sleep"));
    }

    #[test]
    fn test_snoringsleepbasicrules_explain() {
        let rules = SnoringSleepBasicRules::new();
        let e = rules.explain();
        assert!(e.contains("成因认知"));
        assert!(e.contains("姿势调整"));
        assert!(e.contains("生活习惯"));
    }
}
