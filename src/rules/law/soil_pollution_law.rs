//! 土壤污染防治法

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: SoilPollutionLawRules,
    name: "土壤污染防治法",
    desc: "土壤污染防治法律规则",
    origin: "中国",
    tags: ["法律", "环境"]
}

impl SoilPollutionLawRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["源头防控", "重点监管单位", "灌溉水质"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["修复责任", "修复标准", "效果评估"]
    }
}

impl Rule for SoilPollutionLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("soil_pollution_law")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "土壤污染防治法",
            &[("预防", &self.section_0()), ("修复", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_soil_pollution_law_rules() {
        let r = SoilPollutionLawRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
