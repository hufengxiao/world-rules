//! 端午节礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DragonBoatFestivalRules, name: "端午节礼仪", desc: "端午节传统礼仪", origin: "中国", tags: ["社交", "节日"] }
impl DragonBoatFestivalRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["送粽子"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["龙舟竞渡"]
    }
}
impl Rule for DragonBoatFestivalRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("dragon_boat_festival")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "端午节礼仪",
            &[("粽子", &self.section_0()), ("龙舟", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DragonBoatFestivalRules::new();
        assert!(!r.explain().is_empty());
    }
}
