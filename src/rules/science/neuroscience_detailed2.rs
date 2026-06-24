//! 神经科学详细定律2
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: NeuroscienceDetailed2Rules, name: "神经科学详细定律2", desc: "神经科学定律2", origin: "国际", tags: ["科学", "生物"] }
impl NeuroscienceDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["动作电位突触"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["功能分区"]
    }
}
impl Rule for NeuroscienceDetailed2Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("neuroscience_detailed2")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "神经科学详细定律2",
            &[("神经元", &self.section_0()), ("脑区", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = NeuroscienceDetailed2Rules::new();
        assert!(!r.explain().is_empty());
    }
}
