//! 信息论详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: InformationTheoryDetailedRules, name: "信息论详细定律", desc: "信息论详细定律", origin: "国际", tags: ["科学", "数学"] }
impl InformationTheoryDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["信息熵信道容量"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["哈夫曼纠错码"]
    }
}
impl Rule for InformationTheoryDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("information_theory_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "信息论详细定律",
            &[("基本", &self.section_0()), ("编码", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = InformationTheoryDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
