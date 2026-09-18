//! 指甲修剪保健
//!
//! 科学修剪指甲、预防嵌甲与保护甲周的日常方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: NailTrimCareRules,
    name: "指甲修剪保健",
    desc: "科学修剪指甲、预防嵌甲与保护甲周的日常方法",
    origin: "中国",
    tags: ["健康", "指甲", "修剪", "护理"]
}

impl NailTrimCareRules {
    /// 修剪要点
    pub fn trim(&self) -> Vec<&'static str> {
        vec!["泡软后剪", "沿甲缘平剪", "不过深剪入肉", "留一点保护边缘"]
    }

    /// 修形方法
    pub fn shape(&self) -> Vec<&'static str> {
        vec!["剪成平直", "边角轻磨", "不用力抠", "圆润自然"]
    }

    /// 防嵌甲
    pub fn prevent(&self) -> Vec<&'static str> {
        vec!["勿剪太短", "勿剪两侧过深", "穿合脚鞋", "嵌甲痛早处理"]
    }

    /// 指甲健康
    pub fn health(&self) -> Vec<&'static str> {
        vec!["均衡营养", "勿咬指甲", "甲面异常留意", "红肿感染就医"]
    }
}

impl Rule for NailTrimCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("nail_care")
    }

    fn explain(&self) -> String {
        format!(
            "【指甲修剪保健】\n{}",
            [
                format!(
                    "修剪要点：\\n{}",
                    self.trim()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "修形方法：\\n{}",
                    self.shape()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "防嵌甲：\\n{}",
                    self.prevent()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "指甲健康：\\n{}",
                    self.health()
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
    fn test_nailtrimcarerules_basic() {
        let rules = NailTrimCareRules::new();
        assert_eq!(rules.metadata().name, "指甲修剪保健");
        assert!(!rules.trim().is_empty());
        assert!(!rules.shape().is_empty());
        assert!(!rules.prevent().is_empty());
        assert!(!rules.health().is_empty());
    }

    #[test]
    fn test_nailtrimcarerules_validation() {
        let rules = NailTrimCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("nail_care"));
    }

    #[test]
    fn test_nailtrimcarerules_explain() {
        let rules = NailTrimCareRules::new();
        let e = rules.explain();
        assert!(e.contains("修剪要点"));
        assert!(e.contains("修形方法"));
        assert!(e.contains("防嵌甲"));
    }
}
