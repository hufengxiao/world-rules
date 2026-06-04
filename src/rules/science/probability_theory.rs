//! 概率论定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: ProbabilityTheoryRules, name: "概率论定律", desc: "概率论基础定律", origin: "国际", tags: ["科学", "数学"] }
impl ProbabilityTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["大数定律", "中心极限定理", "贝叶斯定理"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["正态分布", "泊松分布"]
    }
}
impl Rule for ProbabilityTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("probability_theory")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "概率论定律",
            &[("基础", &self.section_0()), ("分布", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ProbabilityTheoryRules::new();
        assert!(!r.explain().is_empty());
    }
}
