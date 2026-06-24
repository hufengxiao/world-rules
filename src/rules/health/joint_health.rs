//! 关节健康规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: JointHealthRules, name: "关节健康规则", desc: "关节健康护理规则", origin: "国际", tags: ["健康", "关节"] }
impl JointHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["避免过度"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["低冲击运动"]
    }
}
impl Rule for JointHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("joint_health")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "关节健康规则",
            &[("保护", &self.section_0()), ("运动", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = JointHealthRules::new();
        assert!(!r.explain().is_empty());
    }
}
