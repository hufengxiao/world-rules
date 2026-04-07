//! 北京麻将规则
//!
//! 北京麻将特点是讲究"飘胡"，计分相对简单

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 北京麻将规则
pub struct BeijingMahjongRules {
    metadata: RuleMetadata,
}

impl BeijingMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "北京麻将规则",
                "北京地区流行麻将规则"
            )
            .with_origin("北京")
            .with_tags(vec!["游戏".into(), "麻将".into(), "北京".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌(无花牌)",
            "每人起手13张",
            "可以吃碰杠",
            "可以点炮胡牌",
            "讲究门清飘胡",
        ]
    }

    /// 飘胡规则
    pub fn piao_hu_rules(&self) -> Vec<&'static str> {
        vec![
            "飘胡: 门清自摸胡牌",
            "飘胡番数最高",
            "必须门清(不吃不碰不杠)",
            "必须自摸",
            "飘胡是北京麻将核心",
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
            // 飘胡番型
            ("飘胡", 4),
            ("清飘", 8),
            ("混飘", 6),
            // 高级番型
            ("十三幺", 10),
            ("天胡", 10),
            ("地胡", 8),
            ("杠开", 2),
        ]
    }

    /// 庄家规则
    pub fn banker_rules(&self) -> Vec<&'static str> {
        vec![
            "庄家胡牌连庄",
            "流局庄家听牌连庄",
            "闲家胡牌下家做庄",
            "庄家胡牌番数翻倍",
            "点炮庄家付双倍",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "底分 × 番数",
            "自摸三家付",
            "点炮一家付",
            "庄家翻倍",
            "飘胡翻倍",
        ]
    }

    /// 吃碰规则
    pub fn chi_peng_rules(&self) -> Vec<&'static str> {
        vec![
            "可以吃上家牌",
            "可以碰任意家牌",
            "可以明杠暗杠",
            "吃碰后不再门清",
            "不能门清飘胡",
        ]
    }

    /// 特殊规定
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "不许截胡",
            "一家胡牌后游戏结束",
            "相公(牌数不对)判负",
            "荒牌流局",
            "可以报听",
        ]
    }
}

impl Default for BeijingMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BeijingMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_beijing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        let fan_list: String = self.fan_types()
            .iter()
            .map(|(name, fan)| format!("  • {}: {}番", name, fan))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "【北京麻将规则】\n\n\
            基本设置:\n{}\n\n\
            飘胡规则:\n{}\n\n\
            番型规则:\n{}\n\n\
            计分规则:\n{}\n",
            self.basic_settings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.piao_hu_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            fan_list,
            self.scoring_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beijing_mahjong_rules() {
        let rules = BeijingMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
        assert!(!rules.piao_hu_rules().is_empty());
    }
}