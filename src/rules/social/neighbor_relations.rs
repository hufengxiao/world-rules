//! 邻里相处
//!
//! 邻里见面、互助与保持合适距离的相处之道

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: NeighborRelationsRules,
    name: "邻里相处",
    desc: "邻里见面、互助与保持合适距离的相处之道",
    origin: "中国",
    tags: ["社交", "邻里", "相处", "和睦"]
}

impl NeighborRelationsRules {
    /// 友好招呼
    pub fn greeting(&self) -> Vec<&'static str> {
        vec!["见面主动问好", "微笑点头示意", "不冷眼相对", "长久和睦"]
    }

    /// 互相帮助
    pub fn help(&self) -> Vec<&'static str> {
        vec!["有难处可搭把手", "取快递顺帮带", "借东西及时还", "感恩回礼"]
    }

    /// 保持距离
    pub fn boundary(&self) -> Vec<&'static str> {
        vec!["不探问私事", "不随便串门", "不评论家事", "尊重界限"]
    }

    /// 遇事商量
    pub fn reconcile(&self) -> Vec<&'static str> {
        vec!["有摩擦好好说", "不争吵动手", "物业可协调", "和为贵"]
    }
}

impl Rule for NeighborRelationsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("neighbor")
    }

    fn explain(&self) -> String {
        format!(
            "【邻里相处】\n{}",
            [
                format!(
                    "友好招呼：\\n{}",
                    self.greeting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "互相帮助：\\n{}",
                    self.help()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "保持距离：\\n{}",
                    self.boundary()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "遇事商量：\\n{}",
                    self.reconcile()
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
    fn test_neighborrelationsrules_basic() {
        let rules = NeighborRelationsRules::new();
        assert_eq!(rules.metadata().name, "邻里相处");
        assert!(!rules.greeting().is_empty());
        assert!(!rules.help().is_empty());
        assert!(!rules.boundary().is_empty());
        assert!(!rules.reconcile().is_empty());
    }

    #[test]
    fn test_neighborrelationsrules_validation() {
        let rules = NeighborRelationsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("neighbor"));
    }

    #[test]
    fn test_neighborrelationsrules_explain() {
        let rules = NeighborRelationsRules::new();
        let e = rules.explain();
        assert!(e.contains("友好招呼"));
        assert!(e.contains("互相帮助"));
        assert!(e.contains("保持距离"));
    }
}
