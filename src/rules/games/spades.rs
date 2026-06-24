//! 黑桃王规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SpadesRules, name: "黑桃王规则", desc: "黑桃王(Spades)卡牌游戏规则", origin: "美国", tags: ["游戏", "卡牌"] }
impl SpadesRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "4人分2队，对家为队友",
            "使用标准52张牌(无大小王)",
            "每人发13张牌",
            "黑桃永远是王牌花色",
        ]
    }
    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "发牌后每人叫牌(预测本回合能赢几墩)",
            "叫牌范围: 1-13墩或零Nil",
            "Nil: 声明一墩不赢，成功得100分，失败扣100分",
            "队伍叫牌数相加为目标墩数",
        ]
    }
    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "首攻不能出黑桃(除非手牌只有黑桃)",
            "必须跟首攻花色，无则可出任意牌",
            "王牌(黑桃)赢非王牌，同花色比点数大小",
            "达到叫牌墩数: 每墩得10分，超额每墩1分(沙袋分)",
            "未达叫牌墩数: 扣除叫牌数×10分",
            "10个沙袋分额外扣100分",
            "先到500分的队伍获胜",
        ]
    }
}
impl Rule for SpadesRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("spades")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "黑桃王规则",
            &[
                ("基本设置", &self.section_0()),
                ("叫牌规则", &self.section_1()),
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
        let r = SpadesRules::new();
        assert!(r.explain().contains("黑桃永远是王牌"));
    }
}
