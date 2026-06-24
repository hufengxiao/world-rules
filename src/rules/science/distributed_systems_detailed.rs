//! 分布式系统详细
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DistributedSystemsDetailedRules, name: "分布式系统详细", desc: "分布式系统定律", origin: "国际", tags: ["科学", "计算机"] }
impl DistributedSystemsDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["CAP Raft"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["拜占庭"]
    }
}
impl Rule for DistributedSystemsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("distributed_systems_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "分布式系统详细",
            &[("一致性", &self.section_0()), ("容错", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DistributedSystemsDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
