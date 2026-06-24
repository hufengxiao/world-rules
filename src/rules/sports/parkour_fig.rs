//! 跑酷FIG规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ParkourFigRules, name: "跑酷FIG规则", desc: "跑酷国际体操联合会", origin: "国际", tags: ["体育", "极限"] }
impl ParkourFigRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["速度赛"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["自由式赛"]
    }
}
impl Rule for ParkourFigRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("parkour_fig")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "跑酷FIG规则",
            &[("速度", &self.section_0()), ("自由", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ParkourFigRules::new();
        assert!(!r.explain().is_empty());
    }
}
