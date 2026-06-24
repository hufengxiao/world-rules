//! 分析力学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MechanicsAnalyticalRules, name: "分析力学定律", desc: "分析力学定律", origin: "国际", tags: ["科学", "物理"] }
impl MechanicsAnalyticalRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["拉格朗日方程"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["哈密顿方程"]
    }
}
impl Rule for MechanicsAnalyticalRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("mechanics_analytical")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "分析力学定律",
            &[
                ("拉格朗日", &self.section_0()),
                ("哈密顿", &self.section_1()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MechanicsAnalyticalRules::new();
        assert!(!r.explain().is_empty());
    }
}
