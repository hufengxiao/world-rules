//! 象棋礼仪
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ChineseChessEtiquetteRules, name: "象棋礼仪", desc: "中国象棋礼仪", origin: "中国", tags: ["社交", "棋类"] }
impl ChineseChessEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["落子无悔"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["观棋不语"]
    }
}
impl Rule for ChineseChessEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_chess_etiquette")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "象棋礼仪",
            &[("对弈", &self.section_0()), ("观棋", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ChineseChessEtiquetteRules::new();
        assert!(!r.explain().is_empty());
    }
}
