//! 百家乐规则 (Baccarat)
//!
//! 赌场最流行的游戏之一，规则简单，以9点为最大。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: BaccaratRules,
    name: "百家乐规则",
    desc: "百家乐(Baccarat)规则，赌场经典游戏",
    origin: "意大利/法国",
    tags: ["游戏", "卡牌", "赌场"],
}

impl BaccaratRules {
    /// 游戏设置规则
    pub fn section_setup(&self) -> Vec<&'static str> {
        vec![
            "玩家可押注：庄家(Banker)、闲家(Player)、平局(Tie)",
            "使用6-8副标准扑克牌",
            "庄家和闲家各发2张牌",
            "可选择是否发第三张牌（按固定规则）",
            "所有牌面朝下，荷官负责开牌",
        ]
    }

    /// 点数计算
    pub fn section_counting(&self) -> Vec<&'static str> {
        vec![
            "A = 1点",
            "2-9 = 牌面点数",
            "10、J、Q、K = 0点",
            "总点数为各牌点数之和，取个位数",
            "例：7+8=15，点数为5",
            "最大点数为9，最小为0",
        ]
    }

    /// 补牌规则
    pub fn section_drawing(&self) -> Vec<&'static str> {
        vec![
            "自然赢：庄家或闲家前两张牌为8或9点，不再补牌",
            "闲家补牌规则：",
            "  - 0-5点：补第三张牌",
            "  - 6-7点：停牌",
            "  - 8-9点：自然赢，不再补牌",
            "庄家补牌规则（取决于闲家第三张牌）：",
            "  - 闲家不补牌时，庄家按0-5补，6-7停",
            "  - 闲家补牌后，庄家按复杂规则补牌",
        ]
    }

    /// 庄家补牌详表
    pub fn section_banker_table(&self) -> Vec<&'static str> {
        vec![
            "庄家0-2点：必定补牌",
            "庄家3点：闲家补0-7或9时庄家补，闲家补8或没补时庄家停",
            "庄家4点：闲家补2-7时庄家补，闲家补0,1,8,9或没补时庄家停",
            "庄家5点：闲家补4-7时庄家补，闲家补0-3,8,9或没补时庄家停",
            "庄家6点：闲家补6-7时庄家补，否则停",
            "庄家7点及以上：停牌",
        ]
    }

    /// 赔率规则
    pub fn section_payout(&self) -> Vec<&'static str> {
        vec![
            "押庄家赢：赔率1:1，抽取5%佣金",
            "押闲家赢：赔率1:1，无佣金",
            "押平局：赔率8:1（部分赌场9:1）",
            "押庄对：赌庄家前两张成对，赔率11:1",
            "押闲对：赌闲家前两张成对，赔率11:1",
            "庄家赢时收取5%佣金（实际赔率约0.95:1）",
        ]
    }
}

impl Rule for BaccaratRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("baccarat")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "百家乐规则",
            &[
                ("游戏设置", &self.section_setup()),
                ("点数计算", &self.section_counting()),
                ("补牌规则", &self.section_drawing()),
                ("庄家补牌详表", &self.section_banker_table()),
                ("赔率规则", &self.section_payout()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_baccarat_rules() {
        let rules = BaccaratRules::new();
        assert_eq!(rules.metadata().name, "百家乐规则");
        assert!(rules.explain().contains("自然赢"));
        assert!(rules.explain().contains("5%佣金"));
    }

    #[test]
    fn test_baccarat_category() {
        let rules = BaccaratRules::new();
        assert_eq!(rules.category(), RuleCategory::games("baccarat"));
    }
}