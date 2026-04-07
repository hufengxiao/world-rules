//! 桥牌规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 花色等级 (从高到低)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BridgeSuit {
    NoTrump,  // 无将
    Spades,   // 黑桃
    Hearts,   // 红心
    Diamonds, // 方块
    Clubs,    // 梅花
}

impl BridgeSuit {
    pub fn name(&self) -> &'static str {
        match self {
            BridgeSuit::NoTrump => "无将",
            BridgeSuit::Spades => "黑桃",
            BridgeSuit::Hearts => "红心",
            BridgeSuit::Diamonds => "方块",
            BridgeSuit::Clubs => "梅花",
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            BridgeSuit::NoTrump => "NT",
            BridgeSuit::Spades => "♠",
            BridgeSuit::Hearts => "♥",
            BridgeSuit::Diamonds => "♦",
            BridgeSuit::Clubs => "♣",
        }
    }
}

/// 叫品
#[derive(Debug, Clone)]
pub struct Bid {
    pub level: u8,      // 1-7
    pub suit: BridgeSuit,
}

/// 桥牌规则
pub struct BridgeRules {
    metadata: RuleMetadata,
}

impl BridgeRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "桥牌规则",
                "世界桥牌联合会标准规则"
            )
            .with_origin("国际")
            .with_tags(vec!["游戏".into(), "扑克".into(), "桥牌".into()]),
        }
    }

    /// 每人牌数
    pub fn cards_per_player(&self) -> u8 {
        13
    }

    /// 叫牌级别
    pub fn bid_levels(&self) -> u8 {
        7
    }

    /// 花色等级 (从低到高)
    pub fn suit_order(&self) -> Vec<BridgeSuit> {
        vec![
            BridgeSuit::Clubs,
            BridgeSuit::Diamonds,
            BridgeSuit::Hearts,
            BridgeSuit::Spades,
            BridgeSuit::NoTrump,
        ]
    }

    /// 基本墩数
    pub fn base_tricks(&self) -> u8 {
        6 // 前6墩为基本墩
    }

    /// 计算定约所需墩数
    pub fn tricks_needed(&self, level: u8) -> u8 {
        6 + level
    }

    /// 叫牌规则
    pub fn bidding_rules(&self) -> Vec<&'static str> {
        vec![
            "从发牌者开始顺时针叫牌",
            "后叫的必须比前面的高",
            "可以Pass (不叫)",
            "连续三个Pass后叫牌结束",
            "最后一个叫品成为定约",
        ]
    }

    /// 打牌规则
    pub fn play_rules(&self) -> Vec<&'static str> {
        vec![
            "定约方左边的人首攻",
            "必须跟出同花色的牌",
            "没有该花色可以出其他牌",
            "将牌可以吃其他花色",
            "每墩最大的牌赢得该墩",
        ]
    }

    /// 计分规则 (简化)
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "完成定约: 基本分 + 奖分",
            "超额完成: 额外墩数得分",
            "宕牌: 被罚分",
            "满贯奖分: 小满贯500/大满贯1000",
            "加倍: 定约分翻倍",
        ]
    }

    /// 满贯定义
    pub fn slam_definitions(&self) -> Vec<&'static str> {
        vec![
            "小满贯 (Small Slam): 6级定约，需拿12墩",
            "大满贯 (Grand Slam): 7级定约，需拿13墩",
            "完成满贯有额外奖分",
        ]
    }

    /// 常用术语
    pub fn common_terms(&self) -> Vec<&'static str> {
        vec![
            "开叫 (Opening): 首次叫牌",
            "应叫 (Response): 同伴叫牌后的回应",
            "争叫 (Overcall): 对方叫牌后的叫牌",
            "加倍 (Double): 罚倍对方定约",
            "再加倍 (Redouble): 对加倍的反制",
            "Pass: 不叫",
        ]
    }
}

impl Default for BridgeRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BridgeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("bridge")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【桥牌规则】\n\n\
            牌数: 每人{}张，共52张\n\
            对家: 4人两两对抗\n\n\
            叫牌规则:\n{}\n\n\
            打牌规则:\n{}\n\n\
            计分规则:\n{}\n\n\
            满贯定义:\n{}\n\n\
            常用术语:\n{}\n",
            self.cards_per_player(),
            self.bidding_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.play_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.slam_definitions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.common_terms().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge_rules() {
        let rules = BridgeRules::new();
        assert_eq!(rules.cards_per_player(), 13);
        assert_eq!(rules.tricks_needed(7), 13);
    }
}