//! 粒子物理定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ParticlePhysicsRules,
    name: "粒子物理定律",
    desc: "粒子物理定律",
    origin: "国际",
    tags: ["科学", "物理"],
    category: RuleCategory::science("particle_physics"),
    sections: [("标准模型", section_0), ("守恒律", section_1)]
}

impl ParticlePhysicsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["夸克模型", "轻子家族", "玻色子传递力"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["电荷守恒", "重子数守恒", "CP对称性破缺"]
    }
}
