//! 网络犯罪法
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CybercrimeLawRules, name: "网络犯罪法", desc: "网络犯罪法律规则", origin: "国际", tags: ["法律", "网络"] }
impl CybercrimeLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["黑客钓鱼"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["刑罚"]
    }
}
impl Rule for CybercrimeLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("cybercrime_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "网络犯罪法",
            &[("类型", &self.section_0()), ("处罚", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CybercrimeLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
