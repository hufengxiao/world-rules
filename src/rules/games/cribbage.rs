//! 克里比奇规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CribbageRules, name: "克里比奇规则", desc: "克里比奇(Cribbage)卡牌游戏规则", origin: "英国", tags: ["游戏", "卡牌"] }
impl CribbageRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "2人游戏(可扩展到3-4人)",
            "使用标准52张牌",
            "每人发6张牌(2人)或5张牌(3-4人)",
            "每人弃2张(1人)或1张(多人)到克里比奇Crib",
            "翻一张牌作起始牌",
        ]
    }
    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "凑15: 任意组合点数和为15得2分",
            "对子: 两张同点数得2分",
            "三条: 三张同点数得6分",
            "四条: 四张同点数得12分",
            "顺子: 3张以上连续牌每张1分",
            "同花: 4张同花色4分，5张5分",
            "杰克(Jack): 与起始牌同花色得1分Heels",
        ]
    }
    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "出牌阶段: 轮流出牌，累计点数不能超过31",
            "凑15: 累计到15得2分",
            "凑31: 累计到31得2分",
            "对子: 出同点数得2分，三条6分，四条12分",
            "顺子: 出连续牌得对应张数分",
            "Go: 对手无法出牌时得1分",
            "钉板(Peg)计分: 先到121分获胜",
        ]
    }
}
impl Rule for CribbageRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("cribbage")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "克里比奇规则",
            &[
                ("基本设置", &self.section_0()),
                ("手牌计分", &self.section_1()),
                ("出牌计分", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = CribbageRules::new();
        assert!(r.explain().contains("凑15"));
    }
}
