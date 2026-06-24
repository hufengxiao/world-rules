//! WSL冲浪规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SurfingWslRules, name: "WSL冲浪规则", desc: "世界冲浪联盟规则", origin: "美国", tags: ["体育", "水上"] }
impl SurfingWslRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["优先权", "计分"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["浪高浪型"]
    }
}
impl Rule for SurfingWslRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("surfing_wsl")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "WSL冲浪规则",
            &[("比赛", &self.section_0()), ("浪型", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SurfingWslRules::new();
        assert!(!r.explain().is_empty());
    }
}
