//! 皮诺克尔规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PinochleRules, name: "皮诺克尔规则", desc: "皮诺克尔(Pinochle)卡牌游戏规则", origin: "德国", tags: ["游戏", "卡牌"] }
impl PinochleRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "2-4人游戏(最常见为2人对2人)",
            "使用48张牌: 两副牌中9-A各两张",
            "牌面大小: A > 10 > K > Q > J > 9",
            "每人发12张牌(2人)或15张牌(3人)或12张牌(4人)",
            "翻一张牌决定王牌花色",
        ]
    }
    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "组合(Melds)计分:",
            "同花A-K-Q-J-10: 150分(Royal Marriage加A和10)",
            "同花K-Q: 40分(Marriage)",
            "王牌K-Q: 80分(Royal Marriage)",
            "四张J: 400分(皮诺克尔)",
            "四张Q: 600分, 四张K: 800分, 四张A: 1000分",
            "9王牌: 240分(Dix)",
        ]
    }
    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "出牌: 必须跟首攻花色，无则必须出王牌",
            "王牌能赢非王牌，同花色比点数大小",
            "赢墩包含10或A每张10分",
            "叫牌阶段: 最高叫牌者选王牌花色",
            "达到叫牌分数: 得叫牌分+墩分",
            "未达叫牌分数: 扣叫牌分",
            "先到1000或1500分获胜",
        ]
    }
}
impl Rule for PinochleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("pinochle")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "皮诺克尔规则",
            &[
                ("基本设置", &self.section_0()),
                ("组合计分", &self.section_1()),
                ("出牌与计分", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PinochleRules::new();
        assert!(r.explain().contains("皮诺克尔"));
    }
}
