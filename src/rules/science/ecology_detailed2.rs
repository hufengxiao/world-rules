//! 生态学详细定律2
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: EcologyDetailed2Rules, name: "生态学详细定律2", desc: "生态学定律2", origin: "国际", tags: ["科学", "生物"] }
impl EcologyDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["增长模型"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["演替"]
    }
}
impl Rule for EcologyDetailed2Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("ecology_detailed2")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "生态学详细定律2",
            &[("种群", &self.section_0()), ("群落", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = EcologyDetailed2Rules::new();
        assert!(!r.explain().is_empty());
    }
}
