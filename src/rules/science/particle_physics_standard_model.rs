//! 粒子物理标准模型
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ParticlePhysicsStandardModelRules, name: "粒子物理标准模型", desc: "粒子物理标准模型", origin: "国际", tags: ["科学", "物理"] }
impl ParticlePhysicsStandardModelRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["夸克轻子"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["强弱电磁引力"]
    }
}
impl Rule for ParticlePhysicsStandardModelRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("particle_physics_standard_model")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "粒子物理标准模型",
            &[
                ("基本粒子", &self.section_0()),
                ("相互作用", &self.section_1()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ParticlePhysicsStandardModelRules::new();
        assert!(!r.explain().is_empty());
    }
}
