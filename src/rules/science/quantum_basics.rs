//! 量子物理基础
//!
//! 波粒二象性、不确定性等量子概念基础

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: QuantumBasicsRules,
    name: "量子物理基础",
    desc: "波粒二象性、不确定性等量子概念基础",
    origin: "国际",
    tags: ["科学", "量子", "物理"]
}

impl QuantumBasicsRules {
    /// 波粒二象性
    pub fn duality(&self) -> Vec<&'static str> {
        vec![
            "光既显波动又显粒子",
            "电子有波动性",
            "微观粒子具概率性",
            "观测影响微观行为",
        ]
    }

    /// 不确定性
    pub fn uncertainty(&self) -> Vec<&'static str> {
        vec![
            "位置与动量不可同时精确",
            "量子测量有一定概率",
            "不确定关系为基本原理",
            "不影响日常宏观应用",
        ]
    }

    /// 量子态
    pub fn states(&self) -> Vec<&'static str> {
        vec![
            "微观系统处于量子态",
            "测量后坍缩到本征态",
            "叠加态可同时存在",
            "理解概率幅",
        ]
    }

    /// 应用与发展
    pub fn application(&self) -> Vec<&'static str> {
        vec![
            "解释原子能级与光谱",
            "支撑半导体激光等",
            "量子计算与通信前沿",
            "警惕把量子概念神秘化",
        ]
    }
}

impl Rule for QuantumBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("quantum_basics")
    }

    fn explain(&self) -> String {
        format!(
            "【量子物理基础】\n{}",
            [
                format!(
                    "波粒二象性：\\n{}",
                    self.duality()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "不确定性：\\n{}",
                    self.uncertainty()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "量子态：\\n{}",
                    self.states()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "应用与发展：\\n{}",
                    self.application()
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
    fn test_quantumbasicsrules_basic() {
        let rules = QuantumBasicsRules::new();
        assert_eq!(rules.metadata().name, "量子物理基础");
        assert!(!rules.duality().is_empty());
        assert!(!rules.uncertainty().is_empty());
        assert!(!rules.states().is_empty());
        assert!(!rules.application().is_empty());
    }

    #[test]
    fn test_quantumbasicsrules_validation() {
        let rules = QuantumBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("quantum_basics"));
    }

    #[test]
    fn test_quantumbasicsrules_explain() {
        let rules = QuantumBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("波粒二象性"));
        assert!(e.contains("不确定性"));
        assert!(e.contains("量子态"));
    }
}
