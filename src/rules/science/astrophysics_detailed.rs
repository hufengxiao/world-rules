//! 天体物理详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: AstrophysicsDetailedRules, name: "天体物理详细定律", desc: "天体物理详细定律", origin: "国际", tags: ["科学", "天文"] }
impl AstrophysicsDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["赫罗图演化"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["大爆炸暗能量"]
    }
}
impl Rule for AstrophysicsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("astrophysics_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "天体物理详细定律",
            &[("恒星", &self.section_0()), ("宇宙", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = AstrophysicsDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
