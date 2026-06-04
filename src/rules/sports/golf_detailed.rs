//! 高尔夫详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: GolfDetailedRules, name: "高尔夫详细规则", desc: "高尔夫详细比赛规则", origin: "R&A", tags: ["体育", "球类"] }
impl GolfDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["比杆赛", "比洞赛"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["沙坑", "水障碍"]
    }
}
impl Rule for GolfDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("golf_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "高尔夫详细规则",
            &[("比赛", &self.section_0()), ("障碍", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = GolfDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
