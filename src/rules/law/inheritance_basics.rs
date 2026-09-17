//! 遗产继承基础
//!
//! 遗产继承顺序、遗嘱与家庭财产处理的常识要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: InheritanceBasicsRules,
    name: "遗产继承基础",
    desc: "遗产继承顺序、遗嘱与家庭财产处理的常识要点",
    origin: "中国",
    tags: ["法律", "遗产", "继承", "遗嘱", "财产"]
}

impl InheritanceBasicsRules {
    /// 法定继承
    pub fn legal_order(&self) -> Vec<&'static str> {
        vec![
            "了解法定继承顺序",
            "配偶子女父母为第一顺序",
            "无遗嘱时按法定继承",
            "清偿债务后按序分配遗产",
        ]
    }

    /// 遗嘱订立
    pub fn will(&self) -> Vec<&'static str> {
        vec![
            "遗嘱需符合法定形式",
            "内容自主真实且有效",
            "保留见证人要求",
            "注明遗产范围与受益对象",
        ]
    }

    /// 程序办理
    pub fn procedure(&self) -> Vec<&'static str> {
        vec![
            "继承人可协商或依法分割",
            "办理公证需准备相应文书",
            "有争议可依法调解诉讼",
            "注意办理时限与手续",
        ]
    }

    /// 家庭安排
    pub fn family(&self) -> Vec<&'static str> {
        vec![
            "尽量提前立嘱减少争执",
            "尊重各继承人合法权益",
            "不利用信息不对称侵占",
            "重大处理请专业咨询",
        ]
    }
}

impl Rule for InheritanceBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("inheritance")
    }

    fn explain(&self) -> String {
        format!(
            "【遗产继承基础】\n{}",
            [
                format!(
                    "法定继承：\\n{}",
                    self.legal_order()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "遗嘱订立：\\n{}",
                    self.will()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "程序办理：\\n{}",
                    self.procedure()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "家庭安排：\\n{}",
                    self.family()
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
    fn test_inheritancebasicsrules_basic() {
        let rules = InheritanceBasicsRules::new();
        assert_eq!(rules.metadata().name, "遗产继承基础");
        assert!(!rules.legal_order().is_empty());
        assert!(!rules.will().is_empty());
        assert!(!rules.procedure().is_empty());
        assert!(!rules.family().is_empty());
    }

    #[test]
    fn test_inheritancebasicsrules_validation() {
        let rules = InheritanceBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("inheritance"));
    }

    #[test]
    fn test_inheritancebasicsrules_explain() {
        let rules = InheritanceBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("法定继承"));
        assert!(e.contains("遗嘱订立"));
        assert!(e.contains("程序办理"));
    }
}
