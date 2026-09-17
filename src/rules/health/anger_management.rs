//! 愤怒管理
//!
//! 识别愤怒信号、冷静调节与健康表达方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: AngerManagementRules,
    name: "愤怒管理",
    desc: "识别愤怒信号、冷静调节与健康表达方法",
    origin: "心理学",
    tags: ["健康", "愤怒", "情绪", "调节"]
}

impl AngerManagementRules {
    /// 觉察信号
    pub fn signal(&self) -> Vec<&'static str> {
        vec![
            "注意心跳加快脸红",
            "识别想发怒的身体信号",
            "及时察觉情绪升级",
            "不等到爆发才处理",
        ]
    }

    /// 冷静调节
    pub fn calm(&self) -> Vec<&'static str> {
        vec![
            "深呼吸平复激动",
            "暂时离开冲突现场",
            "数数或喝口水降温",
            "转移注意舒缓",
        ]
    }

    /// 健康表达
    pub fn express(&self) -> Vec<&'static str> {
        vec![
            "用我开头说感受",
            "对事不对人指责",
            "选时机说清楚",
            "不辱骂不破坏",
        ]
    }

    /// 长期成长
    pub fn growth(&self) -> Vec<&'static str> {
        vec![
            "识别愤怒背后的需求",
            "练习换位体谅",
            "必要时寻求疏导",
            "规律运动纾解",
        ]
    }
}

impl Rule for AngerManagementRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("anger")
    }

    fn explain(&self) -> String {
        format!(
            "【愤怒管理】\n{}",
            [
                format!(
                    "觉察信号：\\n{}",
                    self.signal()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "冷静调节：\\n{}",
                    self.calm()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "健康表达：\\n{}",
                    self.express()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "长期成长：\\n{}",
                    self.growth()
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
    fn test_angermanagementrules_basic() {
        let rules = AngerManagementRules::new();
        assert_eq!(rules.metadata().name, "愤怒管理");
        assert!(!rules.signal().is_empty());
        assert!(!rules.calm().is_empty());
        assert!(!rules.express().is_empty());
        assert!(!rules.growth().is_empty());
    }

    #[test]
    fn test_angermanagementrules_validation() {
        let rules = AngerManagementRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("anger"));
    }

    #[test]
    fn test_angermanagementrules_explain() {
        let rules = AngerManagementRules::new();
        let e = rules.explain();
        assert!(e.contains("觉察信号"));
        assert!(e.contains("冷静调节"));
        assert!(e.contains("健康表达"));
    }
}
