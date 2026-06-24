//! 拉米牌详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: RummyDetailedRules, name: "拉米牌详细规则", desc: "拉米牌(Rummy)游戏详细规则", origin: "国际", tags: ["游戏", "卡牌"] }
impl RummyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "2-6人游戏",
            "使用1-2副标准牌(52或104张)",
            "每人发7张牌(2人)或10张牌(3-6人)",
            "剩余牌作抽牌堆，翻一张作弃牌堆",
        ]
    }
    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "回合: 抽一张牌(抽牌堆或弃牌堆顶)",
            "可打出组(Sets): 3-4张同点数不同花色",
            "可打出顺子(Runs): 3张以上同花色连续牌",
            "可将手牌加入已打出的组或顺子",
            "回合结束弃一张牌",
            "清卡(Going Out): 手牌全部组成组和顺子",
        ]
    }
    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "A在顺子中可作1或14(不可环绕)",
            "J/Q/K各10分，A作1时1分，数字牌按面值",
            "对手剩余手牌点数总和加到赢家得分",
            "先到目标分数(通常100或500)获胜",
            "变体: 金拉米(Gin Rummy)、印度拉米、500拉米",
        ]
    }
}
impl Rule for RummyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("rummy_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "拉米牌详细规则",
            &[
                ("基本设置", &self.section_0()),
                ("游戏流程", &self.section_1()),
                ("计分与变体", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = RummyDetailedRules::new();
        assert!(r.explain().contains("清卡"));
    }
}
