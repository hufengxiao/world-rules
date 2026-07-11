//! 开放立直麻将规则
//!
//! 允许明牌手牌的立直麻将变体

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 开放立直麻将规则
pub struct OpenRiichiMahjongRules {
    metadata: RuleMetadata,
}

impl OpenRiichiMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("开放立直麻将规则", "允许明牌的立直麻将变体")
                .with_origin("日本")
                .with_tags(vec!["游戏".into(), "麻将".into(), "变体".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌(无花牌)",
            "四人游戏，每人起手13张",
            "可选择明牌立直",
            "混合门清和副露策略",
            "增加进攻灵活性",
        ]
    }

    /// 开放立直规则
    pub fn open_riichi_rules(&self) -> Vec<&'static str> {
        vec![
            "可选择明牌立直",
            "明牌立直番数降低",
            "明牌后不可更改手牌",
            "对手可见手牌内容",
            "增加心理博弈",
        ]
    }

    /// 役种调整(格式化)
    pub fn yaku_adjustments_formatted(&self) -> Vec<&'static str> {
        vec![
            "立直: 1番",
            "一发: 1番",
            "门前清自摸和: 1番",
            "平和: 1番",
            "断幺九: 1番",
            "役牌: 1番",
            "开放立直: 特殊役种",
            "三色同刻: 2番",
            "一气通贯: 2番",
            "对对和: 2番",
            "三暗刻: 2番",
            "混全带幺九: 2番",
            "七对子: 2番",
        ]
    }

    /// 役种调整
    pub fn yaku_adjustments(&self) -> Vec<(&'static str, u8)> {
        vec![
            // 基础役
            ("暗立直", 1),
            ("明立直", 1),
            ("断幺九", 1),
            ("平和", 1),
            // 明牌特色役
            ("明立直自摸", 1),
            ("明立直荣和", 1),
            ("开放一气", 2),
            ("开放三色", 2),
            // 中级役
            ("对对和", 2),
            ("混一色", 2),
            ("三色同顺(明)", 1),
            ("一气通贯(明)", 1),
            // 高级役
            ("清一色", 5),
            ("完全开放", 6),
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "明立直番数略低",
            "自摸和荣和都有奖励",
            "门清立直番数更高",
            "对手防守更难",
            "增加进攻主动性",
        ]
    }

    /// 策略变化
    pub fn strategy_changes(&self) -> Vec<&'static str> {
        vec![
            "明牌增加进攻性",
            "防守方信息更多",
            "预测对手更容易",
            "心理博弈更复杂",
            "适合高水平玩家",
        ]
    }

    /// 明牌时机
    pub fn timing_rules(&self) -> Vec<&'static str> {
        vec![
            "听牌即可明牌立直",
            "明牌后禁止换牌",
            "可选择是否明牌",
            "明牌时机影响番数",
            "早明牌番数更高",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "适用于竞技比赛",
            "增加观赏性",
            "减少运气成分",
            "重视策略计算",
            "适合转播解说",
        ]
    }
}

impl Rule for OpenRiichiMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_open_riichi")
    }

    fn validate(
        &self,
        _ctx: &crate::rules::core::ValidateContext,
    ) -> crate::rules::core::RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "开放立直麻将规则",
            &[
                ("基本设置", &self.basic_settings()),
                ("开放立直规则", &self.open_riichi_rules()),
                ("役种调整", &self.yaku_adjustments_formatted()),
                ("计分规则", &self.scoring_rules()),
                ("策略变化", &self.strategy_changes()),
                ("明牌时机", &self.timing_rules()),
                ("比赛规则", &self.competition_rules()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let rules = OpenRiichiMahjongRules::new();
        assert_eq!(rules.metadata().name, "开放立直麻将规则");
        assert!(!rules.explain().is_empty());
    }
}
