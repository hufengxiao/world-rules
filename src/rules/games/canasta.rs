//! 卡纳斯塔规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: CanastaRules, name: "卡纳斯塔规则", desc: "卡纳斯塔(Canasta)卡牌游戏规则", origin: "乌拉圭", tags: ["游戏", "卡牌"] }
impl CanastaRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "2-6人游戏，通常4人分2队",
            "使用108张牌(2副标准牌含大小王)",
            "每人发11张牌(2人)或15张牌(4人)",
            "剩余牌作抽牌堆，翻一张作弃牌堆",
            "大小王和所有2为百搭牌(Wild Card)",
        ]
    }
    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "回合: 抽一张牌(从抽牌堆或弃牌堆)",
            "可打出一组或多组同点数的牌(至少3张)",
            "可将手牌加入已打出的组",
            "回合结束弃一张牌",
            "从弃牌堆取牌时必须取整堆，且需立即用顶牌组成新组",
        ]
    }
    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "卡纳斯塔: 7张同点数的牌组成，分自然(无百搭)和混合(含百搭)",
            "自然卡纳斯塔: 500分，混合卡纳斯塔: 300分",
            "红3: 每张100分(全部4张额外加800分)",
            "黑3: 仅用于冻结弃牌堆",
            "初始出牌门槛: 第一次出牌至少50分",
            "手牌剩余牌扣分(红3除外)",
            "先出完手牌且有至少一个卡纳斯塔的一方获胜",
        ]
    }
}
impl Rule for CanastaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("canasta")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "卡纳斯塔规则",
            &[
                ("基本设置", &self.section_0()),
                ("游戏流程", &self.section_1()),
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
        let r = CanastaRules::new();
        assert!(r.explain().contains("卡纳斯塔"));
    }
}
