//! 图论定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: GraphTheoryRules, name: "图论定律", desc: "图论定律", origin: "国际", tags: ["科学", "数学"] }
impl GraphTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["最短路径网络流"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["社交网络"]
    }
}
impl Rule for GraphTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("graph_theory")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "图论定律",
            &[("基本", &self.section_0()), ("应用", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = GraphTheoryRules::new();
        assert!(!r.explain().is_empty());
    }
}
