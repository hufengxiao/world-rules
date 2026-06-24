//! 操作系统定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: OperatingSystemsRules, name: "操作系统定律", desc: "操作系统定律", origin: "国际", tags: ["科学", "计算机"] }
impl OperatingSystemsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["调度同步"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["虚拟内存分页"]
    }
}
impl Rule for OperatingSystemsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("operating_systems")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "操作系统定律",
            &[("进程", &self.section_0()), ("内存", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = OperatingSystemsRules::new();
        assert!(!r.explain().is_empty());
    }
}
