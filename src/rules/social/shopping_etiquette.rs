//! 购物礼仪

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: ShoppingEtiquetteRules,
    name: "购物礼仪",
    desc: "购物社交礼仪",
    origin: "国际",
    tags: ["社交", "消费"]
}

impl ShoppingEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["爱护商品", "归还原位", "不过度试穿"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["遵守排队秩序", "不插队", "结账时不玩手机"]
    }
}

impl Rule for ShoppingEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("shopping_etiquette")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "购物礼仪",
            &[("试穿", &self.section_0()), ("排队", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_shopping_etiquette_rules() {
        let r = ShoppingEtiquetteRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
