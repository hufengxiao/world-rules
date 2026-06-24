//! 元宇宙法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MetaverseLawRules, name: "元宇宙法", desc: "元宇宙法律规则", origin: "国际", tags: ["法律", "科技"] }
impl MetaverseLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["虚拟财产"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["数字身份"]
    }
}
impl Rule for MetaverseLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("metaverse_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "元宇宙法",
            &[("问题", &self.section_0()), ("规范", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MetaverseLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
