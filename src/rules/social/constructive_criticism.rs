//! 建设性批评表达
//!
//! 提出批评建议时给对方自尊与可操作的表达

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ConstructiveCriticismRules,
    name: "建设性批评表达",
    desc: "提出批评建议时给对方自尊与可操作的表达",
    origin: "国际",
    tags: ["社交", "礼仪", "批评", "表达", "建设性"]
}

impl ConstructiveCriticismRules {
    /// 对象与时机
    pub fn timing(&self) -> Vec<&'static str> {
        vec![
            "私下沟通优于当众批评",
            "选择情绪平稳时提出",
            "对事不对人",
            "考虑对方能接受的节奏",
        ]
    }

    /// 表达分寸
    pub fn tone(&self) -> Vec<&'static str> {
        vec![
            "先肯定可取之处",
            "用我表达的观察感受",
            "描述具体行为而非人身",
            "语气平和避免指责",
        ]
    }

    /// 给出建议
    pub fn suggestion(&self) -> Vec<&'static str> {
        vec![
            "提供可行的改进方向",
            "征询对方想法与困难",
            "共同探讨解决",
            "不强迫立即服从",
        ]
    }

    /// 维护关系
    pub fn care(&self) -> Vec<&'static str> {
        vec![
            "表达愿意支持协助",
            "保持尊重与信任",
            "后续给予积极反馈",
            "不翻旧账或标签",
        ]
    }
}

impl Rule for ConstructiveCriticismRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("constructive")
    }

    fn explain(&self) -> String {
        format!(
            "【建设性批评表达】\n{}",
            [
                format!(
                    "对象与时机：\\n{}",
                    self.timing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "表达分寸：\\n{}",
                    self.tone()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "给出建议：\\n{}",
                    self.suggestion()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "维护关系：\\n{}",
                    self.care()
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
    fn test_constructivecriticismrules_basic() {
        let rules = ConstructiveCriticismRules::new();
        assert_eq!(rules.metadata().name, "建设性批评表达");
        assert!(!rules.timing().is_empty());
        assert!(!rules.tone().is_empty());
        assert!(!rules.suggestion().is_empty());
        assert!(!rules.care().is_empty());
    }

    #[test]
    fn test_constructivecriticismrules_validation() {
        let rules = ConstructiveCriticismRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("constructive"));
    }

    #[test]
    fn test_constructivecriticismrules_explain() {
        let rules = ConstructiveCriticismRules::new();
        let e = rules.explain();
        assert!(e.contains("对象与时机"));
        assert!(e.contains("表达分寸"));
        assert!(e.contains("给出建议"));
    }
}
