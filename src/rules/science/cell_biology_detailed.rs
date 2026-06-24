//! 细胞生物学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CellBiologyDetailedRules, name: "细胞生物学详细定律", desc: "细胞生物学定律", origin: "国际", tags: ["科学", "生物"] }
impl CellBiologyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["细胞器"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["有丝减数分裂"]
    }
}
impl Rule for CellBiologyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("cell_biology_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "细胞生物学详细定律",
            &[("结构", &self.section_0()), ("分裂", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CellBiologyDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
