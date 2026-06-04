//! 约会礼仪

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: DatingEtiquetteRules,
    name: "约会礼仪",
    desc: "约会社交礼仪",
    origin: "国际",
    tags: ["社交", "约会"]
}

impl DatingEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["准时到达", "穿着得体", "准备话题"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["手机静音", "主动买单或AA", "尊重对方"]
    }
}

impl Rule for DatingEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("dating_etiquette")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "约会礼仪",
            &[("准备", &self.section_0()), ("行为", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dating_etiquette_rules() {
        let r = DatingEtiquetteRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
