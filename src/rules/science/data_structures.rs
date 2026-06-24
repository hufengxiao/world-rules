//! 数据结构定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DataStructuresRules, name: "数据结构定律", desc: "数据结构定律", origin: "国际", tags: ["科学", "计算机"] }
impl DataStructuresRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["树图哈希表"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["B树红黑树"]
    }
}
impl Rule for DataStructuresRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("data_structures")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "数据结构定律",
            &[("基本", &self.section_0()), ("高级", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DataStructuresRules::new();
        assert!(!r.explain().is_empty());
    }
}
