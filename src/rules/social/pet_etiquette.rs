//! 宠物礼仪

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: PetEtiquetteRules,
    name: "宠物礼仪",
    desc: "养宠社交礼仪",
    origin: "国际",
    tags: ["社交", "宠物"]
}

impl PetEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["牵绳遛狗", "清理宠物粪便", "避开怕动物的人"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["不带入餐厅", "控制宠物行为", "防止吠叫扰民"]
    }
}

impl Rule for PetEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("pet_etiquette")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "宠物礼仪",
            &[("外出", &self.section_0()), ("公共场所", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pet_etiquette_rules() {
        let r = PetEtiquetteRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
