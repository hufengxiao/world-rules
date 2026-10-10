//! 肌肉收缩机制
//!
//! 肌丝滑行学说解释肌肉收缩的生理过程

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MuscleContractionMechRules,
    name: "肌肉收缩机制",
    desc: "肌丝滑行学说解释肌肉收缩的生理过程",
    origin: "科学常识",
    tags: ["生理", "肌肉", "机制"]
}

impl MuscleContractionMechRules {
    /// 结构基础
    pub fn structure(&self) -> Vec<&'static str> {
        vec![
            "肌纤维内含肌原纤维",
            "由粗肌丝与细肌丝组成",
            "粗细肌丝交叠排列",
            "构成明带与暗带",
        ]
    }

    /// 滑行原理
    pub fn sliding(&self) -> Vec<&'static str> {
        vec![
            "细肌丝向粗肌丝间滑入",
            "肌节缩短产生收缩",
            "肌丝本身长度不变",
            "故称肌丝滑行学说",
        ]
    }

    /// 收缩触发
    pub fn trigger(&self) -> Vec<&'static str> {
        vec![
            "神经冲动传导至肌纤维",
            "钙离子释放是关键",
            "钙触发横桥摆动",
            "消耗能量ATP",
        ]
    }

    /// 舒张过程
    pub fn relaxation(&self) -> Vec<&'static str> {
        vec![
            "钙离子被回收",
            "横桥脱离肌丝",
            "肌肉恢复舒张",
            "能量不足则僵硬",
        ]
    }
}

impl Rule for MuscleContractionMechRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("muscle_contraction_mech")
    }

    fn explain(&self) -> String {
        format!(
            "【肌肉收缩机制】\n{}",
            [
                format!(
                    "结构基础：\\n{}",
                    self.structure()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "滑行原理：\\n{}",
                    self.sliding()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "收缩触发：\\n{}",
                    self.trigger()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "舒张过程：\\n{}",
                    self.relaxation()
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
    fn test_musclecontractionmechrules_basic() {
        let rules = MuscleContractionMechRules::new();
        assert_eq!(rules.metadata().name, "肌肉收缩机制");
        assert!(!rules.structure().is_empty());
        assert!(!rules.sliding().is_empty());
        assert!(!rules.trigger().is_empty());
        assert!(!rules.relaxation().is_empty());
    }

    #[test]
    fn test_musclecontractionmechrules_validation() {
        let rules = MuscleContractionMechRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::science("muscle_contraction_mech")
        );
    }

    #[test]
    fn test_musclecontractionmechrules_explain() {
        let rules = MuscleContractionMechRules::new();
        let e = rules.explain();
        assert!(e.contains("结构基础"));
        assert!(e.contains("滑行原理"));
        assert!(e.contains("收缩触发"));
    }
}
