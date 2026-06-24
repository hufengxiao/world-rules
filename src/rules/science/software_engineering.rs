//! 软件工程定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SoftwareEngineeringRules, name: "软件工程定律", desc: "软件工程定律", origin: "国际", tags: ["科学", "计算机"] }
impl SoftwareEngineeringRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["敏捷瀑布"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["测试代码审查"]
    }
}
impl Rule for SoftwareEngineeringRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("software_engineering")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "软件工程定律",
            &[("方法", &self.section_0()), ("质量", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SoftwareEngineeringRules::new();
        assert!(!r.explain().is_empty());
    }
}
