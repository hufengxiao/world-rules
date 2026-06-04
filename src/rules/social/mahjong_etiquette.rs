//! 麻将礼仪

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: MahjongEtiquetteRules,
    name: "麻将礼仪",
    desc: "麻将桌上社交礼仪",
    origin: "中国",
    tags: ["社交", "游戏"]
}

impl MahjongEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["不偷看他人牌", "不故意拖延", "输赢保持风度"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["不暗示他人", "不议论牌局", "尊重对手"]
    }
}

impl Rule for MahjongEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("mahjong_etiquette")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "麻将礼仪",
            &[("行为", &self.section_0()), ("沟通", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mahjong_etiquette_rules() {
        let r = MahjongEtiquetteRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
