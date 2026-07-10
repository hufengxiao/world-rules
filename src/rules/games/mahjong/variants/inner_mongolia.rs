//! 内蒙古麻将规则
//!
//! 内蒙古麻将特点是"大胡"玩法，追求高番牌型

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 内蒙古麻将规则
pub struct InnerMongoliaMahjongRules {
    metadata: RuleMetadata,
}

impl InnerMongoliaMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("内蒙古麻将规则", "内蒙古自治区地区流行麻将规则")
                .with_origin("内蒙古")
                .with_tags(vec!["游戏".into(), "麻将".into(), "内蒙古".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌(无花牌)",
            "每人起手13张",
            "可以吃碰杠",
            "可以点炮胡牌",
            "追求大胡牌型",
        ]
    }

    /// 大胡规则
    pub fn dahu_rules(&self) -> Vec<&'static str> {
        vec![
            "大胡: 高番牌型",
            "清一色、对对胡等",
            "番数越高收益越大",
            "鼓励追求大胡",
            "内蒙古麻将特色",
        ]
    }

    /// 番型规则
    pub fn fan_types(&self) -> Vec<(&'static str, u8)> {
        vec![
            // 基本番型
            ("平胡", 1),
            ("门清", 1),
            ("自摸", 1),
            // 中等番型
            ("对对胡", 2),
            ("七对子", 2),
            ("混一色", 2),
            // 大胡番型
            ("清一色", 4),
            ("清对", 6),
            ("清七对", 8),
            // 高级番型
            ("十三幺", 12),
            ("天胡", 12),
            ("地胡", 10),
            ("杠开", 2),
            ("海底捞", 2),
        ]
    }

    /// 庄家规则
    pub fn banker_rules(&self) -> Vec<&'static str> {
        vec![
            "庄家胡牌连庄",
            "流局庄家听牌连庄",
            "闲家胡牌轮庄",
            "庄家番数翻倍",
            "大胡庄家收益大",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "底分 × 番数",
            "自摸三家付",
            "点炮一家付",
            "庄家翻倍",
            "大胡收益丰厚",
        ]
    }

    /// 吃碰杠规则
    pub fn chi_peng_rules(&self) -> Vec<&'static str> {
        vec![
            "可以吃上家牌",
            "可以碰任意家牌",
            "可以明杠暗杠",
            "吃碰可继续追求大胡",
            "杠后补牌",
        ]
    }

    /// 特殊规定
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "一家胡牌后游戏结束",
            "鼓励追求大胡",
            "荒牌流局",
            "可以抢杠胡",
            "讲究番数",
        ]
    }
}

impl Default for InnerMongoliaMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for InnerMongoliaMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_inner_mongolia")
    }

    fn explain(&self) -> String {
        let fan_list: String = self
            .fan_types()
            .iter()
            .map(|(name, fan)| format!("  • {}: {}番", name, fan))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "【内蒙古麻将规则】\n\n\
            基本设置:\n{}\n\n\
            大胡规则:\n{}\n\n\
            番型规则:\n{}\n\n\
            计分规则:\n{}\n",
            self.basic_settings()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.dahu_rules()
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
    fn test_inner_mongolia_mahjong_rules() {
        let rules = InnerMongoliaMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
        assert!(!rules.dahu_rules().is_empty());
        assert!(rules.fan_types().len() > 0);
    }
}
