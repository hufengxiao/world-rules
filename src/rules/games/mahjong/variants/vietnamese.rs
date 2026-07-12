//! 越南麻将规则
//!
//! 越南麻将特点是16张牌手牌和独特的计分系统

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 越南麻将规则
pub struct VietnameseMahjongRules {
    metadata: RuleMetadata,
}

impl VietnameseMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("越南麻将规则", "越南地区流行的16张麻将规则")
                .with_origin("越南")
                .with_tags(vec!["游戏".into(), "麻将".into(), "越南".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张标准牌",
            "每人起手16张牌(多3张)",
            "胡牌需要17张",
            "东南西北四方",
            "包含花牌和季节牌",
        ]
    }

    /// 牌组结构
    pub fn hand_structure(&self) -> Vec<&'static str> {
        vec![
            "胡牌需要5组+1对将",
            "每组可以是刻子或顺子",
            "可以有明组或暗组",
            "17张牌组成完整牌型",
            "16张手牌+1张进牌",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<(&'static str, u8)> {
        vec![
            // 基本番型
            ("平胡", 1),
            ("对对胡", 3),
            ("七对子", 4),
            // 花色番型
            ("混一色", 3),
            ("清一色", 6),
            ("字一色", 10),
            // 特殊番型
            ("天和", 10),
            ("地和", 8),
            ("人和", 6),
            // 高级番型
            ("十三幺", 10),
            ("四暗刻", 8),
            ("大三元", 10),
            ("小三元", 6),
            // 附加番
            ("杠上花", 2),
            ("杠上炮", 2),
            ("抢杠", 2),
            ("海底捞", 1),
        ]
    }

    /// 花牌规则
    pub fn flower_rules(&self) -> Vec<&'static str> {
        vec![
            "春、夏、秋、冬四季牌",
            "梅、兰、菊、竹四种花牌",
            "花牌匹配座位可得番",
            "花牌不参与牌组构成",
            "摸花牌立即补牌",
        ]
    }

    /// 吃碰杠规则
    pub fn chi_peng_rules(&self) -> Vec<&'static str> {
        vec![
            "可以吃上家牌",
            "可以碰任意家牌",
            "可以明杠暗杠",
            "杠后需要补牌",
            "吃碰后可继续胡",
        ]
    }

    /// 庄家规则
    pub fn banker_rules(&self) -> Vec<&'static str> {
        vec![
            "东家为庄开始",
            "庄家胡牌连庄",
            "闲家胡牌轮庄",
            "流局庄家听牌连庄",
            "庄家番数翻倍",
        ]
    }

    /// 特殊规定
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "点炮三家支付",
            "自摸三家支付",
            "可抢杠胡",
            "可海底捞",
            "一家胡牌本局结束",
        ]
    }

    /// 禁止规则
    pub fn forbidden_rules(&self) -> Vec<&'static str> {
        vec![
            "不能诈胡",
            "必须报听才能胡(部分地区)",
            "不能吃三家牌",
            "流局未听牌罚分",
            "禁止作弊行为",
        ]
    }
}

impl Default for VietnameseMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for VietnameseMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_vietnamese")
    }

    fn explain(&self) -> String {
        let scoring_list: String = self
            .scoring_rules()
            .iter()
            .map(|(name, fan)| format!("  • {}: {}番", name, fan))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "【越南麻将规则】\n\n\
            基本设置:\n{}\n\n\
            牌组结构:\n{}\n\n\
            计分规则:\n{}\n\n\
            花牌规则:\n{}\n\n\
            庄家规则:\n{}\n",
            self.basic_settings()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.hand_structure()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            scoring_list,
            self.flower_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.banker_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vietnamese_mahjong_rules() {
        let rules = VietnameseMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
        assert!(rules.scoring_rules().len() > 0);
    }

    #[test]
    fn test_vietnamese_16_cards() {
        let rules = VietnameseMahjongRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("16张"));
        assert!(explanation.contains("17张"));
    }

    #[test]
    fn test_vietnamese_scoring() {
        let rules = VietnameseMahjongRules::new();
        let scoring = rules.scoring_rules();
        assert!(scoring.iter().any(|(name, _)| *name == "清一色"));
        assert!(scoring.iter().any(|(name, _)| *name == "十三幺"));
    }
}
