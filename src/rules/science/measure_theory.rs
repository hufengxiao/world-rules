//! 测度论定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: MeasureTheoryRules, name: "测度论定律", desc: "测度论定律", origin: "国际", tags: ["科学", "数学"] }
impl MeasureTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["勒贝格测度", "测度空间"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["勒贝格积分", "控制收敛"]
    }
}
impl Rule for MeasureTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("measure_theory")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "测度论定律",
            &[("测度", &self.section_0()), ("积分", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MeasureTheoryRules::new();
        assert!(!r.explain().is_empty());
    }
}
