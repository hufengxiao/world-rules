//! 卡巴迪职业联赛
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: KabaddiProRules, name: "卡巴迪职业联赛", desc: "卡巴迪职业联赛规则", origin: "印度", tags: ["体育", "球类"] }
impl KabaddiProRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["触碰得分"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["拦截追逐"]
    }
}
impl Rule for KabaddiProRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("kabaddi_pro")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "卡巴迪职业联赛",
            &[("进攻", &self.section_0()), ("防守", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = KabaddiProRules::new();
        assert!(!r.explain().is_empty());
    }
}
