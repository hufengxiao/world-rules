//! 帆船奥运规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SailingOlympicRules, name: "帆船奥运规则", desc: "帆船奥运会规则", origin: "国际", tags: ["体育", "水上"] }
impl SailingOlympicRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["激光级470级"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["绕标右舷优先"]
    }
}
impl Rule for SailingOlympicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("sailing_olympic")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "帆船奥运规则",
            &[("级别", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SailingOlympicRules::new();
        assert!(!r.explain().is_empty());
    }
}
