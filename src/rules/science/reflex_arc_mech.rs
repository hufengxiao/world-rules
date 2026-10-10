//! 反射弧机制
//!
//! 神经反射的五个基本环节及其生理意义

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ReflexArcMechRules,
    name: "反射弧机制",
    desc: "神经反射的五个基本环节及其生理意义",
    origin: "科学常识",
    tags: ["生理", "神经", "反射"]
}

impl ReflexArcMechRules {
    /// 五环结构
    pub fn components(&self) -> Vec<&'static str> {
        vec![
            "感受器接收刺激",
            "传入神经传导冲动",
            "神经中枢整合",
            "传出神经发出指令",
            "效应器产生反应",
        ]
    }

    /// 典型反射
    pub fn examples(&self) -> Vec<&'static str> {
        vec!["膝跳反射", "缩手反射", "眨眼反射", "瞳孔对光反射"]
    }

    /// 意义作用
    pub fn significance(&self) -> Vec<&'static str> {
        vec![
            "快速应对危险刺激",
            "保护机体免受伤害",
            "无需大脑有意识参与",
            "是神经调节基本方式",
        ]
    }

    /// 临床检查
    pub fn clinical(&self) -> Vec<&'static str> {
        vec![
            "反射减弱提示损伤",
            "反射亢进见于病变",
            "医生据此判断神经功能",
            "协助定位病变部位",
        ]
    }
}

impl Rule for ReflexArcMechRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("reflex_arc_mech")
    }

    fn explain(&self) -> String {
        format!(
            "【反射弧机制】\n{}",
            [
                format!(
                    "五环结构：\\n{}",
                    self.components()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "典型反射：\\n{}",
                    self.examples()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "意义作用：\\n{}",
                    self.significance()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "临床检查：\\n{}",
                    self.clinical()
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
    fn test_reflexarcmechrules_basic() {
        let rules = ReflexArcMechRules::new();
        assert_eq!(rules.metadata().name, "反射弧机制");
        assert!(!rules.components().is_empty());
        assert!(!rules.examples().is_empty());
        assert!(!rules.significance().is_empty());
        assert!(!rules.clinical().is_empty());
    }

    #[test]
    fn test_reflexarcmechrules_validation() {
        let rules = ReflexArcMechRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("reflex_arc_mech"));
    }

    #[test]
    fn test_reflexarcmechrules_explain() {
        let rules = ReflexArcMechRules::new();
        let e = rules.explain();
        assert!(e.contains("五环结构"));
        assert!(e.contains("典型反射"));
        assert!(e.contains("意义作用"));
    }
}
