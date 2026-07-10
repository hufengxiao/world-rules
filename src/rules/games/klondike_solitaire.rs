//! 经典接龙规则 (Klondike Solitaire)
//!
//! Windows经典单人纸牌游戏，目标是将所有牌移到基础牌堆。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: KlondikeSolitaireRules,
    name: "经典接龙规则",
    desc: "经典接龙(Klondike Solitaire)单人纸牌游戏规则",
    origin: "美国",
    tags: ["游戏", "卡牌", "单人", "接龙"],
}

impl KlondikeSolitaireRules {
    /// 游戏设置
    pub fn section_setup(&self) -> Vec<&'static str> {
        vec![
            "单人游戏",
            "使用标准52张牌（不含大小王）",
            "发牌：7列工作牌堆(Tableau)",
            "  - 第1列：1张牌面朝上",
            "  - 第2列：1张朝上+1张朝下",
            "  - 第3列：1张朝上+2张朝下",
            "  - ...以此类推至第7列",
            "剩余24张作抽牌堆(Stock)",
            "建立4个基础牌堆(Foundation)，初始为空",
        ]
    }

    /// 工作牌堆(Tableau)规则
    pub fn section_tableau(&self) -> Vec<&'static str> {
        vec![
            "工作牌堆可按降序红黑交替堆叠",
            "例：红K上可放黑Q，黑Q上可放红J",
            "可移动整组已翻开的降序红黑交替牌",
            "空工作牌堆只能放K（或含K的整组牌）",
            "翻开朝下的牌成为新的顶牌",
        ]
    }

    /// 基础牌堆(Foundation)规则
    pub fn section_foundation(&self) -> Vec<&'static str> {
        vec![
            "4个基础牌堆对应4种花色",
            "从A开始按升序堆叠（A→2→3→...→K）",
            "必须同花色连续堆叠",
            "一旦放入基础牌堆不可移回",
            "目标：将所有52张牌移到基础牌堆",
        ]
    }

    /// 抽牌堆(Stock)规则
    pub fn section_stock(&self) -> Vec<&'static str> {
        vec![
            "每次从抽牌堆翻1张或3张（根据设定）",
            "翻开的牌放到弃牌堆(Waste)",
            "弃牌堆顶牌可移到工作牌堆或基础牌堆",
            "抽牌堆用完后，弃牌堆整叠翻回抽牌堆",
            "可多次循环使用抽牌堆",
        ]
    }

    /// 获胜与策略
    pub fn section_strategy(&self) -> Vec<&'static str> {
        vec![
            "获胜：所有牌移到基础牌堆",
            "优先移动朝下的牌尽快翻开",
            "尽量保持空工作牌堆用于放K",
            "不要急于将牌移到基础牌堆",
            "有时保留牌在工作牌堆更有利",
            "标准难度：翻1张较易，翻3张较难",
        ]
    }
}

impl Rule for KlondikeSolitaireRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("klondike_solitaire")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "经典接龙规则",
            &[
                ("游戏设置", &self.section_setup()),
                ("工作牌堆", &self.section_tableau()),
                ("基础牌堆", &self.section_foundation()),
                ("抽牌堆", &self.section_stock()),
                ("策略建议", &self.section_strategy()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_klondike_solitaire_rules() {
        let rules = KlondikeSolitaireRules::new();
        assert_eq!(rules.metadata().name, "经典接龙规则");
        assert!(rules.explain().contains("基础牌堆"));
        assert!(rules.explain().contains("工作牌堆"));
        assert!(rules.explain().contains("红黑交替"));
    }

    #[test]
    fn test_klondike_solitaire_category() {
        let rules = KlondikeSolitaireRules::new();
        assert_eq!(rules.category(), RuleCategory::games("klondike_solitaire"));
    }
}
