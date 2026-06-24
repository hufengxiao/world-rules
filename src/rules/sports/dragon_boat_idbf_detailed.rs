//! 龙舟IDBF详细
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DragonBoatIdbfDetailedRules, name: "龙舟IDBF详细", desc: "龙舟国际联合会规则", origin: "国际", tags: ["体育", "水上"] }
impl DragonBoatIdbfDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["200m500m2000m"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["鼓手舵手划手"]
    }
}
impl Rule for DragonBoatIdbfDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("dragon_boat_idbf_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "龙舟IDBF详细",
            &[("项目", &self.section_0()), ("人员", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DragonBoatIdbfDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
