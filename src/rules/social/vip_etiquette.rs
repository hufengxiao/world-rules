//! 贵宾礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: VipEtiquetteRules, name: "贵宾礼仪", desc: "接待贵宾礼仪", origin: "国际", tags: ["社交", "商务"] }
impl VipEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["迎接规格"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["全程陪同"]
    }
}
impl Rule for VipEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("vip_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "贵宾礼仪",
            &[("迎接", &self.section_0()), ("陪同", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = VipEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
