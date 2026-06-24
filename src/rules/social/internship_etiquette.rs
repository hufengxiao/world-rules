//! 实习礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: InternshipEtiquetteRules, name: "实习礼仪", desc: "实习期间礼仪", origin: "国际", tags: ["社交", "职场"] }
impl InternshipEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["主动学习"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["尊重前辈"]
    }
}
impl Rule for InternshipEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("internship_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "实习礼仪",
            &[("主动", &self.section_0()), ("尊重", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = InternshipEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
