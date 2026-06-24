//! 中医学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: TraditionalChineseMedicineRules, name: "中医学定律", desc: "中医学定律", origin: "中国", tags: ["科学", "医学"] }
impl TraditionalChineseMedicineRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["阴阳五行"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["望闻问切"]
    }
}
impl Rule for TraditionalChineseMedicineRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("traditional_chinese_medicine")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中医学定律",
            &[("理论", &self.section_0()), ("诊断", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = TraditionalChineseMedicineRules::new();
        assert!(!r.explain().is_empty());
    }
}
