//! 游泳详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: SwimmingDetailedRules, name: "游泳详细规则", desc: "游泳详细比赛规则", origin: "FINA", tags: ["体育", "水上"] }
impl SwimmingDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["自由泳", "蛙泳", "蝶泳", "仰泳"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["转身规则"]
    }
}
impl Rule for SwimmingDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("swimming_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "游泳详细规则",
            &[("泳姿", &self.section_0()), ("转身", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SwimmingDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
