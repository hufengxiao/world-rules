//! 飞盘高尔夫PDGA
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DiscGolfPdgaDetailedRules, name: "飞盘高尔夫PDGA", desc: "PDGA飞盘高尔夫规则", origin: "美国", tags: ["体育", "休闲"] }
impl DiscGolfPdgaDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["18洞最少投掷"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["推杆盘远距离盘"]
    }
}
impl Rule for DiscGolfPdgaDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("disc_golf_pdga_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "飞盘高尔夫PDGA",
            &[("比赛", &self.section_0()), ("盘型", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DiscGolfPdgaDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
