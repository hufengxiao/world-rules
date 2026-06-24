//! 席拉PERSILAT详细
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SilatPesilatDetailedRules, name: "席拉PERSILAT详细", desc: "席拉国际联盟规则", origin: "印尼", tags: ["体育", "格斗"] }
impl SilatPesilatDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["有效攻击"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["体重分级"]
    }
}
impl Rule for SilatPesilatDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("silat_pesilat_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "席拉PERSILAT详细",
            &[("得分", &self.section_0()), ("级别", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SilatPesilatDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
