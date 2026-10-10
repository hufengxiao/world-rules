//! 激素反馈调节
//!
//! 内分泌系统负反馈调节维持体内平衡的机制

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HormoneFeedbackLoopRules,
    name: "激素反馈调节",
    desc: "内分泌系统负反馈调节维持体内平衡的机制",
    origin: "科学常识",
    tags: ["生理", "内分泌", "调节"]
}

impl HormoneFeedbackLoopRules {
    /// 调节轴
    pub fn axis(&self) -> Vec<&'static str> {
        vec![
            "下丘脑分泌释放激素",
            "垂体分泌促激素",
            "靶腺分泌最终激素",
            "层层调控形成轴",
        ]
    }

    /// 负反馈
    pub fn negative(&self) -> Vec<&'static str> {
        vec![
            "激素升高抑制上级分泌",
            "浓度降低则解除抑制",
            "维持激素水平稳定",
            "是主要调节方式",
        ]
    }

    /// 典型实例
    pub fn examples(&self) -> Vec<&'static str> {
        vec![
            "甲状腺激素调节",
            "血糖与胰岛素",
            "肾上腺皮质激素",
            "性激素周期性变化",
        ]
    }

    /// 临床意义
    pub fn significance(&self) -> Vec<&'static str> {
        vec![
            "反馈失调引发疾病",
            "外源激素抑制腺体",
            "停药需逐渐减量",
            "故不可骤停药物",
        ]
    }
}

impl Rule for HormoneFeedbackLoopRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("hormone_feedback_loop")
    }

    fn explain(&self) -> String {
        format!(
            "【激素反馈调节】\n{}",
            [
                format!(
                    "调节轴：\\n{}",
                    self.axis()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "负反馈：\\n{}",
                    self.negative()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "典型实例：\\n{}",
                    self.examples()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "临床意义：\\n{}",
                    self.significance()
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
    fn test_hormonefeedbacklooprules_basic() {
        let rules = HormoneFeedbackLoopRules::new();
        assert_eq!(rules.metadata().name, "激素反馈调节");
        assert!(!rules.axis().is_empty());
        assert!(!rules.negative().is_empty());
        assert!(!rules.examples().is_empty());
        assert!(!rules.significance().is_empty());
    }

    #[test]
    fn test_hormonefeedbacklooprules_validation() {
        let rules = HormoneFeedbackLoopRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::science("hormone_feedback_loop")
        );
    }

    #[test]
    fn test_hormonefeedbacklooprules_explain() {
        let rules = HormoneFeedbackLoopRules::new();
        let e = rules.explain();
        assert!(e.contains("调节轴"));
        assert!(e.contains("负反馈"));
        assert!(e.contains("典型实例"));
    }
}
