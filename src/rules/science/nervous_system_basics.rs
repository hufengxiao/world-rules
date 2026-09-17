//! 神经系统基础
//!
//! 神经细胞、脑与感觉传导的基础科学常识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: NervousSystemBasicsRules,
    name: "神经系统基础",
    desc: "神经细胞、脑与感觉传导的基础科学常识",
    origin: "国际",
    tags: ["科学", "神经", "大脑", "生物"]
}

impl NervousSystemBasicsRules {
    /// 神经细胞
    pub fn neurons(&self) -> Vec<&'static str> {
        vec![
            "神经元传递电信号",
            "神经元间靠突触连接",
            "信号有兴奋抑制",
            "网络处理信息",
        ]
    }

    /// 脑功能分区
    pub fn brain(&self) -> Vec<&'static str> {
        vec![
            "大脑分左右半球",
            "不同脑区各司其职",
            "感觉运动与认知",
            "脑具可塑适应",
        ]
    }

    /// 反射与协调
    pub fn reflex(&self) -> Vec<&'static str> {
        vec![
            "反射是迅速的自动反应",
            "小脑协调平衡",
            "睡眠休息助神经恢复",
            "长期压力影响神经",
        ]
    }

    /// 保护神经
    pub fn protect(&self) -> Vec<&'static str> {
        vec![
            "充足睡眠保护神经",
            "良好营养支持脑功能",
            "避免头部损伤",
            "神经症状就医",
        ]
    }
}

impl Rule for NervousSystemBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("nervous_system")
    }

    fn explain(&self) -> String {
        format!(
            "【神经系统基础】\n{}",
            [
                format!(
                    "神经细胞：\\n{}",
                    self.neurons()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "脑功能分区：\\n{}",
                    self.brain()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "反射与协调：\\n{}",
                    self.reflex()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "保护神经：\\n{}",
                    self.protect()
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
    fn test_nervoussystembasicsrules_basic() {
        let rules = NervousSystemBasicsRules::new();
        assert_eq!(rules.metadata().name, "神经系统基础");
        assert!(!rules.neurons().is_empty());
        assert!(!rules.brain().is_empty());
        assert!(!rules.reflex().is_empty());
        assert!(!rules.protect().is_empty());
    }

    #[test]
    fn test_nervoussystembasicsrules_validation() {
        let rules = NervousSystemBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("nervous_system"));
    }

    #[test]
    fn test_nervoussystembasicsrules_explain() {
        let rules = NervousSystemBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("神经细胞"));
        assert!(e.contains("脑功能分区"));
        assert!(e.contains("反射与协调"));
    }
}
