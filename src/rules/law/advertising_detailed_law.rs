//! 广告法详解
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: AdvertisingDetailedLawRules, name: "广告法详解", desc: "广告法详解", origin: "中国", tags: ["法律", "广告"] }
impl AdvertisingDetailedLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["虚假广告"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["药品食品"]
    }
}
impl Rule for AdvertisingDetailedLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("advertising_detailed_law")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "广告法详解",
            &[("禁止", &self.section_0()), ("特殊", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AdvertisingDetailedLawRules::new();
        assert!(!r.explain().is_empty());
    }
}
