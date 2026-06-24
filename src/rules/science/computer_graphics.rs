//! 计算机图形学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ComputerGraphicsRules, name: "计算机图形学定律", desc: "计算机图形学定律", origin: "国际", tags: ["科学", "计算机"] }
impl ComputerGraphicsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["光线追踪"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["变换投影"]
    }
}
impl Rule for ComputerGraphicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("computer_graphics")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "计算机图形学定律",
            &[("渲染", &self.section_0()), ("几何", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ComputerGraphicsRules::new();
        assert!(!r.explain().is_empty());
    }
}
