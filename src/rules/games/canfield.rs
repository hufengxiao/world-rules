//! 坎菲尔德规则 (Canfield)
//!
//! 经典单人纸牌游戏，赌场风格，难度较高。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: CanfieldRules,
    name: "坎菲尔德规则",
    desc: "坎菲尔德(Canfield)单人纸牌游戏规则",
    origin: "美国",
    tags: ["游戏", "卡牌", "单人", "赌场"],
}

impl CanfieldRules {
    /// 游戏设置
    pub fn section_setup(&self) -> Vec<&'static str> {
        vec![
            "单人游戏",
            "使用标准52张牌（不含大小王）",
            "发牌：13张牌作抽牌堆(Reserve)",
            "抽牌堆顶牌翻开作为基础牌堆起始牌",
            "基础牌堆(Foundation)：4个堆，同花色升序",
            "工作牌堆(Tableau)：4列，每列1张牌",
            "剩余牌作手牌堆(Stock)",
        ]
    }

    /// 工作牌堆(Tableau)规则
    pub fn section_tableau(&self) -> Vec<&'static str> {
        vec![
            "工作牌堆按降序红黑交替堆叠",
            "例：红K上可放黑Q，黑Q上可放红J",
            "空工作牌堆必须从抽牌堆(Reserve)补牌",
            "不能从手牌堆直接补充工作牌堆",
            "可移动整组已翻开的降序红黑交替牌",
        ]
    }

    /// 基础牌堆(Foundation)规则
    pub fn section_foundation(&self) -> Vec<&'static str> {
        vec![
            "基础牌堆按同花色升序堆叠",
            "起始牌决定花色，必须同花色连续",
            "从起始牌点数开始（例：5→6→7→...→K→A→...→4）",
            "升序环绕：K后接A，直到回到起始牌前一张",
            "目标：将所有52张牌移到基础牌堆",
            "一旦放入基础牌堆不可移回",
        ]
    }

    /// 抽牌堆(Reserve)规则
    pub fn section_reserve(&self) -> Vec<&'static str> {
        vec![
            "13张抽牌堆，顶牌翻开",
            "翻开牌可移到工作牌堆或基础牌堆",
            "空工作牌堆必须从抽牌堆补牌",
            "抽牌堆牌用完后游戏继续",
            "抽牌堆是主要牌源，策略关键",
        ]
    }

    /// 手牌堆(Stock)规则
    pub fn section_stock(&self) -> Vec<&'static str> {
        vec![
            "每次从手牌堆翻3张到弃牌堆(Waste)",
            "弃牌堆顶牌可移到工作牌堆或基础牌堆",
            "手牌堆用完后，弃牌堆整叠翻回手牌堆",
            "可多次循环使用手牌堆",
            "每次翻3张增加难度",
        ]
    }

    /// 获胜与难度
    pub fn section_difficulty(&self) -> Vec<&'static str> {
        vec![
            "获胜：所有牌移到基础牌堆",
            "难度：约5-10%获胜率",
            "赌场版本：初始赌注$50，每张基础牌$5",
            "完全获胜额外$500奖金",
            "策略：充分利用抽牌堆顶牌",
            "谨慎处理空工作牌堆",
        ]
    }
}

impl Rule for CanfieldRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("canfield")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "坎菲尔德规则",
            &[
                ("游戏设置", &self.section_setup()),
                ("工作牌堆", &self.section_tableau()),
                ("基础牌堆", &self.section_foundation()),
                ("抽牌堆", &self.section_reserve()),
                ("手牌堆", &self.section_stock()),
                ("难度", &self.section_difficulty()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canfield_rules() {
        let rules = CanfieldRules::new();
        assert_eq!(rules.metadata().name, "坎菲尔德规则");
        assert!(rules.explain().contains("Reserve"));
        assert!(rules.explain().contains("5-10%"));
        assert!(rules.explain().contains("升序环绕"));
    }

    #[test]
    fn test_canfield_category() {
        let rules = CanfieldRules::new();
        assert_eq!(rules.category(), RuleCategory::games("canfield"));
    }
}
