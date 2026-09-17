//! 塑料分类回收
//!
//! 塑料类型辨识、分类回收与减塑常识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PlasticRecyclingBasicsRules,
    name: "塑料分类回收",
    desc: "塑料类型辨识、分类回收与减塑常识",
    origin: "环保",
    tags: ["科学", "塑料", "回收", "环保"]
}

impl PlasticRecyclingBasicsRules {
    /// 塑料类型
    pub fn types(&self) -> Vec<&'static str> {
        vec!["三角形标数字", "标号种类不同", "用途各有别", "辨清再回收"]
    }

    /// 分类投放
    pub fn sort(&self) -> Vec<&'static str> {
        vec!["洗净晾干再投", "按回收箱分类", "不混其他垃圾", "减少污染"]
    }

    /// 减塑生活
    pub fn reduce(&self) -> Vec<&'static str> {
        vec!["少用一次性", "自带购物袋", "重复利用容器", "减碳环保"]
    }

    /// 回收价值
    pub fn benefit(&self) -> Vec<&'static str> {
        vec!["废塑料可再造", "节约石油资源", "减少白色污染", "资源再生"]
    }
}

impl Rule for PlasticRecyclingBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("plastic")
    }

    fn explain(&self) -> String {
        format!(
            "【塑料分类回收】\n{}",
            [
                format!(
                    "塑料类型：\\n{}",
                    self.types()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "分类投放：\\n{}",
                    self.sort()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "减塑生活：\\n{}",
                    self.reduce()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "回收价值：\\n{}",
                    self.benefit()
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
    fn test_plasticrecyclingbasicsrules_basic() {
        let rules = PlasticRecyclingBasicsRules::new();
        assert_eq!(rules.metadata().name, "塑料分类回收");
        assert!(!rules.types().is_empty());
        assert!(!rules.sort().is_empty());
        assert!(!rules.reduce().is_empty());
        assert!(!rules.benefit().is_empty());
    }

    #[test]
    fn test_plasticrecyclingbasicsrules_validation() {
        let rules = PlasticRecyclingBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("plastic"));
    }

    #[test]
    fn test_plasticrecyclingbasicsrules_explain() {
        let rules = PlasticRecyclingBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("塑料类型"));
        assert!(e.contains("分类投放"));
        assert!(e.contains("减塑生活"));
    }
}
