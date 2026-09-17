//! 科学实验方法
//!
//! 设计实验、控制变量与得出可靠结论的方法规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ExperimentalMethodRules,
    name: "科学实验方法",
    desc: "设计实验、控制变量与得出可靠结论的方法规则",
    origin: "国际",
    tags: ["科学", "实验", "方法", "变量", "严谨"]
}

impl ExperimentalMethodRules {
    /// 提出假设
    pub fn hypothesis(&self) -> Vec<&'static str> {
        vec![
            "明确待检验的问题",
            "提出可验证的假设",
            "界定自变量与因变量",
            "预期结果可度量",
        ]
    }

    /// 控制变量
    pub fn control(&self) -> Vec<&'static str> {
        vec![
            "只改变一个自变量",
            "其余变量保持恒定",
            "设置对照组",
            "重复实验避免偶然",
        ]
    }

    /// 规范数据
    pub fn data(&self) -> Vec<&'static str> {
        vec![
            "如实记录观察数据",
            "标注测量工具与误差",
            "保留原始记录",
            "采用合适统计方法",
        ]
    }

    /// 结论审慎
    pub fn conclusion(&self) -> Vec<&'static str> {
        vec![
            "结论与数据一致",
            "不夸大适用范围",
            "承认局限与不足",
            "可复现支持结论",
        ]
    }
}

impl Rule for ExperimentalMethodRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("experimental_method")
    }

    fn explain(&self) -> String {
        format!(
            "【科学实验方法】\n{}",
            [
                format!(
                    "提出假设：\\n{}",
                    self.hypothesis()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "控制变量：\\n{}",
                    self.control()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "规范数据：\\n{}",
                    self.data()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "结论审慎：\\n{}",
                    self.conclusion()
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
    fn test_experimentalmethodrules_basic() {
        let rules = ExperimentalMethodRules::new();
        assert_eq!(rules.metadata().name, "科学实验方法");
        assert!(!rules.hypothesis().is_empty());
        assert!(!rules.control().is_empty());
        assert!(!rules.data().is_empty());
        assert!(!rules.conclusion().is_empty());
    }

    #[test]
    fn test_experimentalmethodrules_validation() {
        let rules = ExperimentalMethodRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::science("experimental_method")
        );
    }

    #[test]
    fn test_experimentalmethodrules_explain() {
        let rules = ExperimentalMethodRules::new();
        let e = rules.explain();
        assert!(e.contains("提出假设"));
        assert!(e.contains("控制变量"));
        assert!(e.contains("规范数据"));
    }
}
