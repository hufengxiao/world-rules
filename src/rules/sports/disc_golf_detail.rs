//! 飞盘高尔夫详细
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: DiscGolfDetailRules, name: "飞盘高尔夫详细", desc: "飞盘高尔夫详细规则", origin: "PDGA", tags: ["体育", "休闲"] }
impl DiscGolfDetailRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["18洞", "最少投掷"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["推杆盘", "远距离盘"]
    }
}
impl Rule for DiscGolfDetailRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("disc_golf_detail")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "飞盘高尔夫详细",
            &[("比赛", &self.section_0()), ("盘型", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DiscGolfDetailRules::new();
        assert!(!r.explain().is_empty());
    }
}
