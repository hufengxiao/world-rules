//! 广义相对论定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: GeneralRelativityRules, name: "广义相对论定律", desc: "广义相对论定律", origin: "国际", tags: ["科学", "物理"] }
impl GeneralRelativityRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["爱因斯坦场方程", "测地线方程"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["引力时间膨胀", "引力透镜", "引力波"]
    }
}
impl Rule for GeneralRelativityRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("general_relativity")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "广义相对论定律",
            &[("方程", &self.section_0()), ("效应", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = GeneralRelativityRules::new();
        assert!(!r.explain().is_empty());
    }
}
