//! 摩托艇IJSBA规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: JetSkiIjsbaRules, name: "摩托艇IJSBA规则", desc: "摩托艇国际规则", origin: "国际", tags: ["体育", "水上"] }
impl JetSkiIjsbaRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["坐式立式"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["救生衣"]
    }
}
impl Rule for JetSkiIjsbaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("jet_ski_ijsba")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "摩托艇IJSBA规则",
            &[("组别", &self.section_0()), ("安全", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = JetSkiIjsbaRules::new();
        assert!(!r.explain().is_empty());
    }
}
