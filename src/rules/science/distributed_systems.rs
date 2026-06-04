//! 分布式系统理论
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: DistributedSystemsRules, name: "分布式系统理论", desc: "分布式系统理论定律", origin: "国际", tags: ["科学", "计算机"] }
impl DistributedSystemsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["CAP定理", "Paxos算法", "Raft共识"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["拜占庭容错", "故障检测"]
    }
}
impl Rule for DistributedSystemsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("distributed_systems")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "分布式系统理论",
            &[("一致性", &self.section_0()), ("容错", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DistributedSystemsRules::new();
        assert!(!r.explain().is_empty());
    }
}
