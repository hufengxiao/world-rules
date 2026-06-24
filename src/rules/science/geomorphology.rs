//! 地貌学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: GeomorphologyRules, name: "地貌学定律", desc: "地貌学定律", origin: "国际", tags: ["科学", "地球"] }
impl GeomorphologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["侵蚀沉积"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["河流冰川"]
    }
}
impl Rule for GeomorphologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("geomorphology")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "地貌学定律",
            &[("过程", &self.section_0()), ("地貌", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = GeomorphologyRules::new();
        assert!(!r.explain().is_empty());
    }
}
