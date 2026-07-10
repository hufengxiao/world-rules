//! 金字塔接龙规则 (Pyramid Solitaire)
//!
//! 独特的金字塔布局单人纸牌游戏，通过配对清除金字塔。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: PyramidSolitaireRules,
    name: "金字塔接龙规则",
    desc: "金字塔接龙(Pyramid Solitaire)单人纸牌游戏规则",
    origin: "国际",
    tags: ["游戏", "卡牌", "单人", "接龙", "配对"],
}

impl PyramidSolitaireRules {
    /// 游戏设置
    pub fn section_setup(&self) -> Vec<&'static str> {
        vec![
            "单人游戏",
            "使用标准52张牌（不含大小王）",
            "金字塔布局：28张牌组成金字塔",
            "  - 第1行：1张牌",
            "  - 第2行：2张牌",
            "  - 第3行：3张牌",
            "  - ...以此类推至第7行（7张牌）",
            "剩余24张作抽牌堆(Stock)",
            "金字塔下层牌部分遮挡上层牌",
        ]
    }

    /// 牌值规则
    pub fn section_card_values(&self) -> Vec<&'static str> {
        vec![
            "A = 1点",
            "2-10 = 牌面点数",
            "J = 11点",
            "Q = 12点",
            "K = 13点（特殊：可单独移除）",
            "配对目标：两张牌点数之和 = 13",
        ]
    }

    /// 移除规则
    pub fn section_removal(&self) -> Vec<&'static str> {
        vec![
            "配对移除：两张未遮挡牌点数之和为13",
            "例：6+7=13，Q+1=13，J+2=13",
            "K单独移除：K点数为13，无需配对",
            "未遮挡牌：没有被下层牌覆盖的牌",
            "金字塔最底层（第7行）牌完全未遮挡",
            "移除上层牌后，下层牌可能变为未遮挡",
        ]
    }

    /// 抽牌堆(Stock)规则
    pub fn section_stock(&self) -> Vec<&'static str> {
        vec![
            "每次从抽牌堆翻1张到弃牌堆(Waste)",
            "弃牌堆顶牌可与金字塔未遮挡牌配对",
            "弃牌堆顶牌也可与抽牌堆刚翻牌配对",
            "抽牌堆用完后，弃牌堆整叠翻回",
            "可多次循环使用抽牌堆",
            "限制循环次数增加难度（通常3次）",
        ]
    }

    /// 获胜条件
    pub fn section_winning(&self) -> Vec<&'static str> {
        vec![
            "获胜：清除金字塔所有28张牌",
            "部分规则要求清除抽牌堆和弃牌堆",
            "失败：无法配对且循环次数用尽",
            "评分：清除金字塔后剩余牌数",
            "完美获胜：清除所有52张牌",
            "难度：约50-60%可解概率",
        ]
    }

    /// 策略建议
    pub fn section_strategy(&self) -> Vec<&'static str> {
        vec![
            "优先移除金字塔上层牌（释放下层牌）",
            "K最容易移除，优先处理",
            "保留低点数牌（易配对）",
            "谨慎使用高点数牌配对",
            "记牌，避免关键牌被埋",
            "合理规划抽牌堆循环次数",
        ]
    }
}

impl Rule for PyramidSolitaireRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("pyramid_solitaire")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "金字塔接龙规则",
            &[
                ("游戏设置", &self.section_setup()),
                ("牌值规则", &self.section_card_values()),
                ("移除规则", &self.section_removal()),
                ("抽牌堆", &self.section_stock()),
                ("获胜条件", &self.section_winning()),
                ("策略建议", &self.section_strategy()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pyramid_solitaire_rules() {
        let rules = PyramidSolitaireRules::new();
        assert_eq!(rules.metadata().name, "金字塔接龙规则");
        assert!(rules.explain().contains("金字塔"));
        assert!(rules.explain().contains("配对"));
        assert!(rules.explain().contains("13"));
    }

    #[test]
    fn test_pyramid_solitaire_category() {
        let rules = PyramidSolitaireRules::new();
        assert_eq!(rules.category(), RuleCategory::games("pyramid_solitaire"));
    }
}