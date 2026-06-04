//! 房地产法详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: RealEstateDetailedRules, name: "房地产法详解", desc: "房地产法详解", origin: "中国", tags: ["法律", "房产"] }
impl RealEstateDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["建设用地", "规划许可"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["商品房预售", "产权登记"]
    }
}
impl Rule for RealEstateDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("real_estate_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "房地产法详解",
            &[("开发", &self.section_0()), ("交易", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = RealEstateDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
