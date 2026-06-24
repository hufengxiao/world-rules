//! 领导力礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: LeadershipEtiquetteRules, name: "领导力礼仪", desc: "领导力社交礼仪", origin: "国际", tags: ["社交", "职场"] }
impl LeadershipEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["积极倾听"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["建设性反馈"]
    }
}
impl Rule for LeadershipEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("leadership_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "领导力礼仪",
            &[("倾听", &self.section_0()), ("反馈", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = LeadershipEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
