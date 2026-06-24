//! 经典力学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MechanicsClassicalRules, name: "经典力学定律", desc: "经典力学定律", origin: "国际", tags: ["科学", "物理"] }
impl MechanicsClassicalRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["三大定律", "万有引力"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["动能定理", "能量守恒"]
    }
}
impl Rule for MechanicsClassicalRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("mechanics_classical")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "经典力学定律",
            &[("牛顿", &self.section_0()), ("功和能", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MechanicsClassicalRules::new();
        assert!(!r.explain().is_empty());
    }
}
