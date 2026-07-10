//! 河北麻将规则
//!
//! 河北麻将特点是"推倒胡"玩法，规则简洁实用

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 河北麻将规则
pub struct HebeiMahjongRules {
    metadata: RuleMetadata,
}

impl HebeiMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("河北麻将规则", "河北省地区流行麻将规则")
                .with_origin("河北")
                .with_tags(vec!["游戏".into(), "麻将".into(), "河北".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌(无花牌)",
            "每人起手13张",
            "可以吃碰杠",
            "可以点炮胡牌",
            "推倒胡玩法",
        ]
    }

    /// 推倒胡规则
    pub fn tuidao_hu_rules(&self) -> Vec<&'static str> {
        vec![
            "推倒胡: 最基本的胡牌方式",
            "只需满足基本牌型",
            "不计复杂番型",
            "强调速度和运气",
            "河北麻将核心玩法",
        ]
    }

    /// 番型规则
    pub fn fan_types(&self) -> Vec<(&'static str, u8)> {
        vec![
            // 基本番型
            ("屁胡", 1),
            ("门清", 1),
            ("自摸", 1),
            // 花色番型
            ("混一色", 2),
            ("清一色", 4),
            ("对对胡", 2),
            ("七对子", 2),
            // 高级番型
            ("十三幺", 8),
            ("天胡", 8),
            ("地胡", 6),
            ("杠开", 2),
        ]
    }

    /// 庄家规则
    pub fn banker_rules(&self) -> Vec<&'static str> {
        vec![
            "庄家胡牌连庄",
            "流局庄家听牌连庄",
            "闲家胡牌轮庄",
            "庄家番数翻倍",
            "庄家优势明显",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "底分 × 番数",
            "自摸三家付",
            "点炮一家付",
            "庄家翻倍",
            "简洁计分",
        ]
    }

    /// 吃碰杠规则
    pub fn chi_peng_rules(&self) -> Vec<&'static str> {
        vec![
            "可以吃上家牌",
            "可以碰任意家牌",
            "可以明杠暗杠",
            "吃碰不影响胡牌",
            "杠后补牌",
        ]
    }

    /// 特殊规定
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "一家胡牌后游戏结束",
            "不设最低番数限制",
            "荒牌流局",
            "相公判负",
            "简单实用",
        ]
    }
}

impl Default for HebeiMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for HebeiMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_hebei")
    }

    fn explain(&self) -> String {
        let fan_list: String = self
            .fan_types()
            .iter()
            .map(|(name, fan)| format!("  • {}: {}番", name, fan))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "【河北麻将规则】\n\n\
            基本设置:\n{}\n\n\
            推倒胡规则:\n{}\n\n\
            番型规则:\n{}\n\n\
            计分规则:\n{}\n",
            self.basic_settings()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.tuidao_hu_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            fan_list,
            self.scoring_rules()
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
    fn test_hebei_mahjong_rules() {
        let rules = HebeiMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
        assert!(!rules.tuidao_hu_rules().is_empty());
        assert!(rules.fan_types().len() > 0);
    }
}
