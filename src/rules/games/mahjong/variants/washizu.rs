//! 和志麻将规则
//!
//! 特殊透明牌变体，部分牌可见

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 和志麻将规则
pub struct WashizuMahjongRules {
    metadata: RuleMetadata,
}

impl WashizuMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("和志麻将规则", "透明牌麻将变体")
                .with_origin("日本")
                .with_tags(vec!["游戏".into(), "麻将".into(), "变体".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌",
            "透明牌规则：部分牌为透明",
            "透明牌可见所有玩家",
            "增加策略深度",
            "四人竞技",
        ]
    }

    /// 透明牌规则
    pub fn transparent_rules(&self) -> Vec<&'static str> {
        vec![
            "透明牌：可见所有玩家",
            "不透明牌：仅持有者可见",
            "透明比例可调整",
            "标准：每张牌3透明1不透明",
            "极端：全部透明",
        ]
    }

    /// 特殊役种
    pub fn special_yaku(&self) -> Vec<(&'static str, u8)> {
        vec![
            // 基础役种同立直麻将
            ("立直", 1),
            ("断幺九", 1),
            ("平和", 1),
            // 信息优势役
            ("信息制胜", 1),
            ("明牌胡", 1),
            // 高级役
            ("透明一气", 2),
            ("预测胜利", 3),
            ("完美防守", 3),
            // 役满
            ("透明国士", 13),
            ("透明九莲", 13),
        ]
    }

    /// 策略要点
    pub fn strategy_points(&self) -> Vec<&'static str> {
        vec![
            "观察透明牌预测对手手牌",
            "根据可见牌调整听牌",
            "计算概率更精确",
            "防守策略更重要",
            "诈胡和反诈胡技巧",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "基本计分同立直麻将",
            "透明胡额外加番",
            "防守成功有补偿",
            "信息差计分调整",
        ]
    }

    /// 和志特殊规则
    pub fn washizu_specific(&self) -> Vec<&'static str> {
        vec![
            "透明牌数量可调",
            "可设置半透明牌",
            "允许明牌立直",
            "增加心理博弈",
            "适合高阶玩家",
        ]
    }

    /// 禁止事项
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "禁止记录所有牌",
            "禁止使用外部辅助工具",
            "禁止串通",
            "禁止利用bug",
        ]
    }
}

impl Rule for WashizuMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_washizu")
    }

    fn validate(
        &self,
        _ctx: &crate::rules::core::ValidateContext,
    ) -> crate::rules::core::RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "和志麻将规则",
            &[
                ("基本设置", &self.basic_settings()),
                ("透明牌规则", &self.transparent_rules()),
                (
                    "特殊役种",
                    &self
                        .special_yaku()
                        .iter()
                        .map(|(name, fan)| format!("{}: {}番", name, fan))
                        .collect::<Vec<_>>(),
                ),
                ("策略要点", &self.strategy_points()),
                ("计分规则", &self.scoring_rules()),
                ("和志特殊规则", &self.washizu_specific()),
                ("禁止事项", &self.prohibited_actions()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let rules = WashizuMahjongRules::new();
        assert_eq!(rules.metadata().name, "和志麻将规则");
        assert!(!rules.explain().is_empty());
    }
}
