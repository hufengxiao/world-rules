//! 婴儿辅食添加
//!
//! 婴儿辅食的添加时机、顺序与过敏注意

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ToddlerIntroducingSolidsRules,
    name: "婴儿辅食添加",
    desc: "婴儿辅食的添加时机、顺序与过敏注意",
    origin: "医学",
    tags: ["健康", "辅食", "婴儿", "喂养"]
}

impl ToddlerIntroducingSolidsRules {
    /// 添加时机
    pub fn timing(&self) -> Vec<&'static str> {
        vec!["约六个月起加辅食", "四月后看可", "由稀到稠", "逐步引入"]
    }

    /// 顺序种类
    pub fn order(&self) -> Vec<&'static str> {
        vec!["先米粉糊状", "加菜泥果泥", "再加蛋黄肉泥", "一次加一种"]
    }

    /// 过敏观察
    pub fn allergy(&self) -> Vec<&'static str> {
        vec![
            "每添一种换几天试",
            "留意过敏腹泻",
            "鸡蛋坚果谨慎",
            "过敏先停就医",
        ]
    }

    /// 食物质地
    pub fn texture(&self) -> Vec<&'static str> {
        vec!["软烂易于消化", "防噎不硬快", "少盐少糖", "逐步过渡"]
    }
}

impl Rule for ToddlerIntroducingSolidsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("toddler_solids")
    }

    fn explain(&self) -> String {
        format!(
            "【婴儿辅食添加】\n{}",
            [
                format!(
                    "添加时机：\\n{}",
                    self.timing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "顺序种类：\\n{}",
                    self.order()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "过敏观察：\\n{}",
                    self.allergy()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "食物质地：\\n{}",
                    self.texture()
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
    fn test_toddlerintroducingsolidsrules_basic() {
        let rules = ToddlerIntroducingSolidsRules::new();
        assert_eq!(rules.metadata().name, "婴儿辅食添加");
        assert!(!rules.timing().is_empty());
        assert!(!rules.order().is_empty());
        assert!(!rules.allergy().is_empty());
        assert!(!rules.texture().is_empty());
    }

    #[test]
    fn test_toddlerintroducingsolidsrules_validation() {
        let rules = ToddlerIntroducingSolidsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("toddler_solids"));
    }

    #[test]
    fn test_toddlerintroducingsolidsrules_explain() {
        let rules = ToddlerIntroducingSolidsRules::new();
        let e = rules.explain();
        assert!(e.contains("添加时机"));
        assert!(e.contains("顺序种类"));
        assert!(e.contains("过敏观察"));
    }
}
