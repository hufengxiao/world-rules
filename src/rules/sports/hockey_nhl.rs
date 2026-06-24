//! NHL冰球规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: HockeyNhlRules, name: "NHL冰球规则", desc: "北美职业冰球联盟", origin: "加拿大", tags: ["体育", "冬季"] }
impl HockeyNhlRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["3节60分钟"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["越位", "icing"]
    }
}
impl Rule for HockeyNhlRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("hockey_nhl")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "NHL冰球规则",
            &[("比赛", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = HockeyNhlRules::new();
        assert!(!r.explain().is_empty());
    }
}
