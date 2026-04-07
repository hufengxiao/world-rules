//! 广东麻将规则
//!
//! 广东麻将是最流行的麻将玩法之一，特点是花样繁多，计分复杂

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 广东麻将规则
pub struct GuangdongMahjongRules {
    metadata: RuleMetadata,
}

impl GuangdongMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "广东麻将规则",
                "广东地区流行麻将规则"
            )
            .with_origin("广东")
            .with_tags(vec!["游戏".into(), "麻将".into(), "广东".into()]),
        }
    }

    /// 基本规则
    pub fn basic_rules(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌(无花牌)",
            "4人对战，每人13张起手",
            "庄家起手14张",
            "可以吃、碰、杠",
            "允许点炮胡牌",
        ]
    }

    /// 番型规则
    pub fn fan_types(&self) -> Vec<(&'static str, u8)>        {
        vec![
            ("鸡胡", 1),
            ("平胡", 1),
            ("对对胡", 3),
            ("混一色", 3),
            ("清一色", 6),
            ("七对子", 4),
            ("十三幺", 13),
            ("大三元", 13),
            ("小三元", 8),
            ("大四喜", 13),
            ("小四喜", 8),
            ("字一色", 13),
            ("清老头", 10),
            ("绿一色", 10),
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "买马: 胡牌后翻牌定输赢倍数",
            "马牌: 根据马牌数量乘倍",
            "无马: 1倍",
            "一匹马: 2倍",
            "两匹马: 3倍",
            "三匹马: 4倍",
            "四匹马及以上: 6倍",
        ]
    }

    /// 鸡胡规则
    pub fn chicken_hand_rules(&self) -> Vec<&'static str> {
        vec![
            "鸡胡是最基本的胡牌型",
            "没有特殊番型",
            "一般组合即可",
            "番数最低",
            "通常作为起胡门槛",
        ]
    }

    /// 杠牌规则
    pub fn kong_rules(&self) -> Vec<&'static str> {
        vec![
            "明杠: 暴露杠牌",
            "暗杠: 隐藏杠牌",
            "补杠: 加杠",
            "杠开: 杠后自摸胡牌",
            "抢杠: 抢他人补杠胡牌",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "底分: 基础分数",
            "番数: 番型番数",
            "马数: 买马倍数",
            "总分 = 底分 × 番数 × 马数",
            "自摸三家付",
            "点炮一家付",
        ]
    }

    /// 禁忌规则
    pub fn forbidden_rules(&self) -> Vec<&'static str> {
        vec![
            "不能截胡(三家都可以胡)",
            "不能漏胡(过牌后不能胡同一张)",
            "不能诈胡(无胡牌型宣布胡牌)",
        ]
    }
}

impl Default for GuangdongMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for GuangdongMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_guangdong")
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
            "【广东麻将规则】\n\n\
            基本规则:\n{}\n\n\
            番型规则:\n{}\n\n\
            特殊规则:\n{}\n\n\
            计分规则:\n{}\n",
            self.basic_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            fan_list,
            self.special_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guangdong_mahjong_rules() {
        let rules = GuangdongMahjongRules::new();
        assert!(!rules.basic_rules().is_empty());
        assert!(!rules.fan_types().is_empty());
    }
}