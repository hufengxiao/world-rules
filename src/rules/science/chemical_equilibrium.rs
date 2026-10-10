//! 化学平衡
//!
//! 可逆反应达到动态平衡及影响因素

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ChemicalEquilibriumRules,
    name: "化学平衡",
    desc: "可逆反应达到动态平衡及影响因素",
    origin: "科学常识",
    tags: ["化学", "平衡", "反应"]
}

impl ChemicalEquilibriumRules {
    /// 动态平衡
    pub fn dynamic(&self) -> Vec<&'static str> {
        vec![
            "正逆反应速率相等",
            "各组分浓度不变",
            "反应并未停止",
            "是动态的相对平衡",
        ]
    }

    /// 影响因素
    pub fn factors(&self) -> Vec<&'static str> {
        vec![
            "浓度改变平衡移动",
            "温度影响平衡方向",
            "压强影响气体反应",
            "催化剂不改变平衡",
        ]
    }

    /// 勒夏特列原理
    pub fn principle(&self) -> Vec<&'static str> {
        vec![
            "改变条件平衡向减弱该改变方向移动",
            "增加反应物利于正反应",
            "升温向吸热方向移动",
            "加压向气体少的方向移动",
        ]
    }

    /// 实际应用
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "合成氨工业调控条件",
            "控制反应提高产率",
            "催化剂加快达平衡",
            "平衡移动指导生产",
        ]
    }
}

impl Rule for ChemicalEquilibriumRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("chemical_equilibrium")
    }

    fn explain(&self) -> String {
        format!(
            "【化学平衡】\n{}",
            [
                format!(
                    "动态平衡：\\n{}",
                    self.dynamic()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "影响因素：\\n{}",
                    self.factors()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "勒夏特列原理：\\n{}",
                    self.principle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "实际应用：\\n{}",
                    self.applications()
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
    fn test_chemicalequilibriumrules_basic() {
        let rules = ChemicalEquilibriumRules::new();
        assert_eq!(rules.metadata().name, "化学平衡");
        assert!(!rules.dynamic().is_empty());
        assert!(!rules.factors().is_empty());
        assert!(!rules.principle().is_empty());
        assert!(!rules.applications().is_empty());
    }

    #[test]
    fn test_chemicalequilibriumrules_validation() {
        let rules = ChemicalEquilibriumRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::science("chemical_equilibrium")
        );
    }

    #[test]
    fn test_chemicalequilibriumrules_explain() {
        let rules = ChemicalEquilibriumRules::new();
        let e = rules.explain();
        assert!(e.contains("动态平衡"));
        assert!(e.contains("影响因素"));
        assert!(e.contains("勒夏特列原理"));
    }
}
