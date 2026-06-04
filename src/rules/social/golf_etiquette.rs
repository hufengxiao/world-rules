//! 高尔夫社交礼仪

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: GolfEtiquetteRules,
    name: "高尔夫社交礼仪",
    desc: "高尔夫球场社交礼仪",
    origin: "国际",
    tags: ["社交", "运动"]
}

impl GolfEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["保持安静", "修复球痕", "不踩推击线"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["保持打球节奏", "让后组先行", "准备好再打"]
    }
}

impl Rule for GolfEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("golf_etiquette")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "高尔夫社交礼仪",
            &[("球场", &self.section_0()), ("速度", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_golf_etiquette_rules() {
        let r = GolfEtiquetteRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
