//! 会议礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MeetingEtiquetteRules, name: "会议礼仪", desc: "商务会议礼仪", origin: "国际", tags: ["社交", "职场"] }
impl MeetingEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["提前到场"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["有序发言"]
    }
}
impl Rule for MeetingEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("meeting_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "会议礼仪",
            &[("准时", &self.section_0()), ("发言", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MeetingEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
