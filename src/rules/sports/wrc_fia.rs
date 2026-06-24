//! WRC世界拉力赛
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: WrcFiaRules, name: "WRC世界拉力赛", desc: "FIA世界拉力锦标赛", origin: "国际", tags: ["体育", "赛车"] }
impl WrcFiaRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["特殊赛段"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["维修区罚时"]
    }
}
impl Rule for WrcFiaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("wrc_fia")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "WRC世界拉力赛",
            &[("赛段", &self.section_0()), ("规则", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = WrcFiaRules::new();
        assert!(!r.explain().is_empty());
    }
}
