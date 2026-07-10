//! 金拉米规则 (Gin Rummy)
//!
//! 经典双人拉米牌游戏，以"敲"Gin为核心玩法。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: GinRummyRules,
    name: "金拉米规则",
    desc: "金拉米(Gin Rummy)双人卡牌游戏规则",
    origin: "美国",
    tags: ["游戏", "卡牌", "拉米", "双人"],
}

impl GinRummyRules {
    /// 游戏设置
    pub fn section_setup(&self) -> Vec<&'static str> {
        vec![
            "双人游戏",
            "使用标准52张牌（不含大小王）",
            "每人发10张牌",
            "剩余牌作抽牌堆，翻开第一张作弃牌堆",
            "非发牌方先选择抽牌堆或弃牌堆顶牌",
        ]
    }

    /// 游戏流程
    pub fn section_play(&self) -> Vec<&'static str> {
        vec![
            "每回合：抽一张牌，然后弃一张牌",
            "可从抽牌堆顶部抽牌（不给对手信息）",
            "或从弃牌堆顶部抽牌（公开信息）",
            "弃牌必须放到弃牌堆顶部",
            "手牌组成组和顺子，未组成牌为死牌(Deadwood)",
        ]
    }

    /// 组牌规则
    pub fn section_melds(&self) -> Vec<&'static str> {
        vec![
            "组(Sets)：3-4张同点数不同花色",
            "顺子(Runs)：3张以上同花色连续牌",
            "A只能作为最低牌（A-2-3），不能环绕（Q-K-A）",
            "组和顺子可以后续添加牌扩展",
            "死牌点数：A=1，J/Q/K=10，数字牌按面值",
        ]
    }

    /// 敲牌(Gin)规则
    pub fn section_gin(&self) -> Vec<&'static str> {
        vec![
            "敲牌(Knock)：死牌点数≤10时可宣布敲牌",
            "敲牌后展示手牌，对手也可亮牌",
            "对手可添加牌到敲牌者的组和顺子(lay off)",
            "Gin：死牌为0，敲牌者得25分+对手死牌分",
            "敲牌者死牌≤对手死牌：敲牌者得差值分",
            "敲牌者死牌>对手死牌：对手得10分+差值分",
        ]
    }

    /// 计分规则
    pub fn section_scoring(&self) -> Vec<&'static str> {
        vec![
            "游戏结束：任一方累计达到100分",
            "方盒(Box)奖励：累计100分对手得100方盒分",
            "金拉米(Gin)奖励：额外25分",
            "大敲牌(Big Gin)：抽牌后直接Gin，额外31分",
            "策略：尽快降低死牌，适时敲牌",
            "尽量从抽牌堆取牌，避免给对手信息",
        ]
    }
}

impl Rule for GinRummyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("gin_rummy")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "金拉米规则",
            &[
                ("游戏设置", &self.section_setup()),
                ("游戏流程", &self.section_play()),
                ("组牌规则", &self.section_melds()),
                ("敲牌规则", &self.section_gin()),
                ("计分规则", &self.section_scoring()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gin_rummy_rules() {
        let rules = GinRummyRules::new();
        assert_eq!(rules.metadata().name, "金拉米规则");
        assert!(rules.explain().contains("敲牌"));
        assert!(rules.explain().contains("死牌"));
        assert!(rules.explain().contains("25分"));
    }

    #[test]
    fn test_gin_rummy_category() {
        let rules = GinRummyRules::new();
        assert_eq!(rules.category(), RuleCategory::games("gin_rummy"));
    }
}