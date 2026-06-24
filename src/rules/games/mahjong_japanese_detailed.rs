//! 日本麻将详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MahjongJapaneseDetailedRules, name: "日本麻将详细规则", desc: "日本立直麻将", origin: "日本", tags: ["游戏", "麻将"] }
impl MahjongJapaneseDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "日本麻将立直麻将",
            "使用136张牌无花牌",
            "每人发13张牌",
            "宝牌指示牌决定宝牌",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "立直:听牌时宣布立直需支付1000点",
            "一发:立直后一圈内胡牌加1翻",
            "里宝牌:立直胡牌时翻开里宝牌",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "大四喜:四种风牌刻子",
            "绿一色:全由条子23468组成",
            "九莲宝灯:同花色1112345678999加任意一张",
            "四暗刻:四个暗刻",
            "国士无双:十三幺",
        ]
    }
}
impl Rule for MahjongJapaneseDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_japanese_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "日本麻将详细规则",
            &[
                ("基本规则", &self.section_0()),
                ("立直规则", &self.section_1()),
                ("役满", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MahjongJapaneseDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
