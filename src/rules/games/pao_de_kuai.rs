//! 跑得快规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 跑得快牌型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaoDeKuaiPattern {
    /// 单张
    Single,
    /// 对子
    Pair,
    /// 三张
    Triple,
    /// 三带一
    TripleWithOne,
    /// 三带二
    TripleWithPair,
    /// 顺子
    Straight,
    /// 连对
    DoubleStraight,
    /// 飞机
    Plane,
    /// 炸弹
    Bomb,
    /// 四带二
    FourWithTwo,
}

impl PaoDeKuaiPattern {
    pub fn name(&self) -> &'static str {
        match self {
            PaoDeKuaiPattern::Single => "单张",
            PaoDeKuaiPattern::Pair => "对子",
            PaoDeKuaiPattern::Triple => "三张",
            PaoDeKuaiPattern::TripleWithOne => "三带一",
            PaoDeKuaiPattern::TripleWithPair => "三带二",
            PaoDeKuaiPattern::Straight => "顺子",
            PaoDeKuaiPattern::DoubleStraight => "连对",
            PaoDeKuaiPattern::Plane => "飞机",
            PaoDeKuaiPattern::Bomb => "炸弹",
            PaoDeKuaiPattern::FourWithTwo => "四带二",
        }
    }
}

/// 跑得快规则
pub struct PaoDeKuaiRules {
    metadata: RuleMetadata,
}

impl PaoDeKuaiRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "跑得快规则",
                "跑得快扑克游戏规则"
            )
            .with_origin("中国")
            .with_tags(vec!["游戏".into(), "扑克".into()]),
        }
    }

    /// 牌数分配
    pub fn card_distribution(&self) -> (u8, u8, u8) {
        (17, 17, 16) // 三人，总共51张(去掉大小王)
    }

    /// 基本规则
    pub fn basic_rules(&self) -> Vec<&'static str> {
        vec![
            "三人对战",
            "51张牌，去掉大小王",
            "最先出完牌的人获胜",
            "必须出比上家大的牌",
            "可以选择不出(过牌)",
        ]
    }

    /// 牌型说明
    pub fn pattern_descriptions(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("单张", "任意一张牌"),
            ("对子", "两张点数相同的牌"),
            ("三张", "三张点数相同的牌"),
            ("三带一", "三张+任意一张"),
            ("三带二", "三张+任意一对"),
            ("顺子", "五张或更多连续的牌(不含2)"),
            ("连对", "三对或更多连续对子"),
            ("飞机", "两个或更多连续三张"),
            ("炸弹", "四张相同点数的牌"),
            ("四带二", "四张+任意两张单牌或一对"),
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "2为最大牌",
            "黑桃3先出",
            "炸弹可以炸任何牌",
            "首家必须出黑桃3",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "第一名得3分",
            "第二名得1分",
            "第三名得0分",
            "炸弹翻倍",
        ]
    }
}

impl Default for PaoDeKuaiRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PaoDeKuaiRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("pao_de_kuai")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        let patterns = self.pattern_descriptions();
        format!(
            "【跑得快规则】\n\n\
            牌数: {}张\n\n\
            基本规则:\n{}\n\n\
            牌型:\n{}\n\n\
            特殊规则:\n{}\n\n\
            计分规则:\n{}\n",
            51,
            self.basic_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            patterns.iter().map(|(n, d)| format!("  • {}: {}", n, d)).collect::<Vec<_>>().join("\n"),
            self.special_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pao_de_kuai_rules() {
        let rules = PaoDeKuaiRules::new();
        assert_eq!(rules.card_distribution().0, 17);
    }
}