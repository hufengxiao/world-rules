//! 宪法详解
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: ConstitutionDetailedRules, name: "宪法详解", desc: "宪法基本权利详解", origin: "中国", tags: ["法律", "宪法"] }
impl ConstitutionDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["平等权", "自由权", "社会权"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["人大制度", "国务院"]
    }
}
impl Rule for ConstitutionDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::law("constitution_detailed")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "宪法详解",
            &[
                ("基本权利", &self.section_0()),
                ("国家机构", &self.section_1()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ConstitutionDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
