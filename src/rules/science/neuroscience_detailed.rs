//! 神经科学详细定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: NeuroscienceDetailedRules, name: "神经科学详细定律", desc: "神经科学详细定律", origin: "国际", tags: ["科学", "生物"] }
impl NeuroscienceDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["动作电位", "突触传递", "神经可塑性"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["大脑皮层", "海马体"]
    }
}
impl Rule for NeuroscienceDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("neuroscience_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "神经科学详细定律",
            &[("神经元", &self.section_0()), ("脑区", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = NeuroscienceDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
