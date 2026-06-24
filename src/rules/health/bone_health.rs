//! 骨骼健康规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BoneHealthRules, name: "骨骼健康规则", desc: "骨骼健康护理规则", origin: "国际", tags: ["健康", "骨骼"] }
impl BoneHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["补钙维D"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["负重运动"]
    }
}
impl Rule for BoneHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("bone_health")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "骨骼健康规则",
            &[("营养", &self.section_0()), ("运动", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BoneHealthRules::new();
        assert!(!r.explain().is_empty());
    }
}
