//! 数据科学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DataScienceDetailedRules, name: "数据科学详细定律", desc: "数据科学定律", origin: "国际", tags: ["科学", "计算机"] }
impl DataScienceDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["探索性分析"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["特征工程"]
    }
}
impl Rule for DataScienceDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("data_science_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "数据科学详细定律",
            &[("分析", &self.section_0()), ("工程", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DataScienceDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
