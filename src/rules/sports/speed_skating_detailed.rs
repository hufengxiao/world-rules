//! 速滑详细规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: SpeedSkatingDetailedRules, name: "速滑详细规则", desc: "速度滑冰详细规则", origin: "ISU", tags: ["体育", "冬季"] }
impl SpeedSkatingDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["500米", "5000米"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["内外道交换"]
    }
}
impl Rule for SpeedSkatingDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("speed_skating_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "速滑详细规则",
            &[("项目", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SpeedSkatingDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
