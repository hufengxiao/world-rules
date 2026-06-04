//! 粒子物理定律

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: ParticlePhysicsRules,
    name: "粒子物理定律",
    desc: "粒子物理定律",
    origin: "国际",
    tags: ["科学", "物理"]
}

impl ParticlePhysicsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["夸克模型", "轻子家族", "玻色子传递力"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["电荷守恒", "重子数守恒", "CP对称性破缺"]
    }
}

impl Rule for ParticlePhysicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("particle_physics")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "粒子物理定律",
            &[
                ("标准模型", &self.section_0()),
                ("守恒律", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_particle_physics_rules() {
        let r = ParticlePhysicsRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
