//! 香港麻将规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MahjongHongkongRules, name: "香港麻将规则", desc: "香港麻将规则", origin: "香港", tags: ["游戏", "麻将"] }
impl MahjongHongkongRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "香港麻将使用鸡胡规则",
            "鸡胡即可胡牌无最低番数要求",
            "使用144张牌含花牌",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "平胡:基本胡牌",
            "碰碰胡:全部刻子",
            "清一色:全同花色",
            "混一色:一种花色加字牌",
            "小三元:两种箭牌刻子加一种箭牌对子",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "鸡胡1番",
            "碰碰胡2番",
            "清一色4番",
            "混一色2番",
            "自摸加1番",
        ]
    }
}
impl Rule for MahjongHongkongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_hongkong")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "香港麻将规则",
            &[
                ("基本规则", &self.section_0()),
                ("番种", &self.section_1()),
                ("计分规则", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MahjongHongkongRules::new();
        assert!(!r.explain().is_empty());
    }
}
