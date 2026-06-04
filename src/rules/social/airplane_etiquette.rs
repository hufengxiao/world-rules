//! 飞机礼仪

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: AirplaneEtiquetteRules,
    name: "飞机礼仪",
    desc: "飞机乘坐礼仪",
    origin: "国际",
    tags: ["社交", "旅行"]
}

impl AirplaneEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["按区域排队", "快速入座", "行李放好"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["调低音量", "不脱鞋", "适度使用卫生间"]
    }
}

impl Rule for AirplaneEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("airplane_etiquette")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "飞机礼仪",
            &[("登机", &self.section_0()), ("飞行中", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_airplane_etiquette_rules() {
        let r = AirplaneEtiquetteRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
