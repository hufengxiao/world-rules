//! 常见材料特性
//!
//! 金属塑料陶瓷等常见材料的性质与选用常识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MaterialPropertiesRules,
    name: "常见材料特性",
    desc: "金属塑料陶瓷等常见材料的性质与选用常识",
    origin: "国际",
    tags: ["科学", "材料", "性质"]
}

impl MaterialPropertiesRules {
    /// 金属材料
    pub fn metal(&self) -> Vec<&'static str> {
        vec![
            "金属导电导热性强",
            "铁易锈需防护",
            "铝轻质耐腐蚀",
            "合金增强性能",
        ]
    }

    /// 塑料与陶瓷
    pub fn plastics(&self) -> Vec<&'static str> {
        vec![
            "塑料多样轻便",
            "分热塑热固性",
            "陶瓷耐热硬脆",
            "玻璃透明易碎",
        ]
    }

    /// 木材与织物
    pub fn wood(&self) -> Vec<&'static str> {
        vec![
            "木材隔热有质感",
            "需防潮防腐",
            "天然纤维透气",
            "合成纤维耐磨",
        ]
    }

    /// 按需选用
    pub fn choose(&self) -> Vec<&'static str> {
        vec![
            "用途选合适材料",
            "注意安全热危险",
            "环保再生优先",
            "识别标示再回收",
        ]
    }
}

impl Rule for MaterialPropertiesRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("material")
    }

    fn explain(&self) -> String {
        format!(
            "【常见材料特性】\n{}",
            [
                format!(
                    "金属材料：\\n{}",
                    self.metal()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "塑料与陶瓷：\\n{}",
                    self.plastics()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "木材与织物：\\n{}",
                    self.wood()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "按需选用：\\n{}",
                    self.choose()
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
    fn test_materialpropertiesrules_basic() {
        let rules = MaterialPropertiesRules::new();
        assert_eq!(rules.metadata().name, "常见材料特性");
        assert!(!rules.metal().is_empty());
        assert!(!rules.plastics().is_empty());
        assert!(!rules.wood().is_empty());
        assert!(!rules.choose().is_empty());
    }

    #[test]
    fn test_materialpropertiesrules_validation() {
        let rules = MaterialPropertiesRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("material"));
    }

    #[test]
    fn test_materialpropertiesrules_explain() {
        let rules = MaterialPropertiesRules::new();
        let e = rules.explain();
        assert!(e.contains("金属材料"));
        assert!(e.contains("塑料与陶瓷"));
        assert!(e.contains("木材与织物"));
    }
}
