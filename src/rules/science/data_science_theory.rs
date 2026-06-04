//! 数据科学理论
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: DataScienceTheoryRules, name: "数据科学理论", desc: "数据科学理论定律", origin: "国际", tags: ["科学", "计算机"] }
impl DataScienceTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["探索性分析", "特征工程"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["SQL", "可视化"]
    }
}
impl Rule for DataScienceTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("data_science_theory")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "数据科学理论",
            &[("方法", &self.section_0()), ("工具", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DataScienceTheoryRules::new();
        assert!(!r.explain().is_empty());
    }
}
