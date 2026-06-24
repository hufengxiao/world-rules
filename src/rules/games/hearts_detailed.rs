//! 红心大战详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: HeartsDetailedRules, name: "红心大战详细规则", desc: "红心大战(Hearts)卡牌游戏详细规则", origin: "美国", tags: ["游戏", "卡牌"] }
impl HeartsDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "4人游戏，使用标准52张牌",
            "每人发13张牌",
            "第一轮传3张牌给指定方向(左/右/对面/不传)",
            "持有梅花2的玩家先出牌",
        ]
    }
    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "必须跟首攻花色，无则可出任意牌",
            "不能首攻红心(除非已破红心)",
            "破红心: 当某人无首攻花色时出红心",
            "同花色中最大牌赢墩",
            "赢墩的玩家开始下一回合",
        ]
    }
    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "每张红心: 1分",
            "黑桃Q(猪牌): 13分",
            "目标: 避免吃到红心和黑桃Q",
            "收齐(Shooting the Moon): 吃到全部红心+黑桃Q",
            "收齐成功: 其他三人各加26分，自己不加分",
            "收齐失败: 自己加26分",
            "先到100分时得分最低者获胜",
        ]
    }
}
impl Rule for HeartsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("hearts_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "红心大战详细规则",
            &[
                ("基本设置", &self.section_0()),
                ("出牌规则", &self.section_1()),
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
        let r = HeartsDetailedRules::new();
        assert!(r.explain().contains("收齐"));
    }
}
