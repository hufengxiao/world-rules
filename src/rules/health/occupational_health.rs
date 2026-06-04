//! 职业健康规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: OccupationalHealthRules,
    name: "职业健康规则",
    desc: "职业健康与安全规则",
    origin: "国际",
    tags: ["健康", "职业"]
}

impl OccupationalHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["个人防护装备", "工作环境通风", "噪音防护"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["工作压力管理", "职业倦怠预防", "工作生活平衡"]
    }
}

impl Rule for OccupationalHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("occupational_health")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "职业健康规则",
            &[("防护", &self.section_0()), ("心理", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_occupational_health_rules() {
        let r = OccupationalHealthRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
