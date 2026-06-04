//! 材料工程定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: MaterialsEngineeringRules, name: "材料工程定律", desc: "材料工程定律", origin: "国际", tags: ["科学", "材料"] }
impl MaterialsEngineeringRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["合金设计", "热处理", "腐蚀防护"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["纤维增强", "层合板"]
    }
}
impl Rule for MaterialsEngineeringRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("materials_engineering")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "材料工程定律",
            &[("金属", &self.section_0()), ("复合材料", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MaterialsEngineeringRules::new();
        assert!(!r.explain().is_empty());
    }
}
