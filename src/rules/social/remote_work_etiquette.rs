//! 远程办公礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: RemoteWorkEtiquetteRules, name: "远程办公礼仪", desc: "远程办公社交礼仪", origin: "国际", tags: ["社交", "职场"] }
impl RemoteWorkEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["背景整洁"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["及时回复"]
    }
}
impl Rule for RemoteWorkEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("remote_work_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "远程办公礼仪",
            &[("视频", &self.section_0()), ("消息", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = RemoteWorkEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
