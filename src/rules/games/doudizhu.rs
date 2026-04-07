//! 斗地主规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 牌型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardPattern {
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
    /// 顺子 (至少5张)
    Straight,
    /// 连对 (至少3对)
    DoubleStraight,
    /// 飞机 (至少2个三张)
    Plane,
    /// 飞机带翅膀
    PlaneWithWings,
    /// 四带二
    FourWithTwo,
    /// 炸弹
    Bomb,
    /// 王炸
    Rocket,
}

impl CardPattern {
    pub fn name(&self) -> &'static str {
        match self {
            CardPattern::Single => "单张",
            CardPattern::Pair => "对子",
            CardPattern::Triple => "三张",
            CardPattern::TripleWithOne => "三带一",
            CardPattern::TripleWithPair => "三带二",
            CardPattern::Straight => "顺子",
            CardPattern::DoubleStraight => "连对",
            CardPattern::Plane => "飞机",
            CardPattern::PlaneWithWings => "飞机带翅膀",
            CardPattern::FourWithTwo => "四带二",
            CardPattern::Bomb => "炸弹",
            CardPattern::Rocket => "王炸",
        }
    }

    pub fn priority(&self) -> u8 {
        match self {
            CardPattern::Single => 1,
            CardPattern::Pair => 2,
            CardPattern::Triple => 3,
            CardPattern::TripleWithOne => 4,
            CardPattern::TripleWithPair => 5,
            CardPattern::Straight => 6,
            CardPattern::DoubleStraight => 7,
            CardPattern::Plane => 8,
            CardPattern::PlaneWithWings => 9,
            CardPattern::FourWithTwo => 10,
            CardPattern::Bomb => 11,
            CardPattern::Rocket => 12,
        }
    }
}

/// 斗地主规则
pub struct DouDiZhuRules {
    metadata: RuleMetadata,
}

impl DouDiZhuRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "斗地主规则",
                "斗地主标准规则"
            )
            .with_origin("中国")
            .with_tags(vec!["游戏".into(), "扑克".into(), "斗地主".into()]),
        }
    }

    /// 牌数分配
    pub fn card_distribution(&self) -> (u8, u8, u8, u8) {
        (17, 17, 17, 3) // 三人各17张，3张底牌
    }

    /// 底牌数量
    pub fn landlord_cards(&self) -> u8 {
        20 // 地主20张 (17+3底牌)
    }

    /// 农民牌数
    pub fn farmer_cards(&self) -> u8 {
        17
    }

    /// 牌型说明
    pub fn pattern_descriptions(&self) -> Vec<(CardPattern, &'static str)> {
        vec![
            (CardPattern::Single, "任意一张单牌"),
            (CardPattern::Pair, "两张点数相同的牌"),
            (CardPattern::Triple, "三张点数相同的牌"),
            (CardPattern::TripleWithOne, "三张+任意一张单牌"),
            (CardPattern::TripleWithPair, "三张+任意一对"),
            (CardPattern::Straight, "五张或更多连续单牌（不含2和王）"),
            (CardPattern::DoubleStraight, "三对或更多连续对子（不含2和王）"),
            (CardPattern::Plane, "两个或更多连续三张（不含2和王）"),
            (CardPattern::PlaneWithWings, "飞机+同数量的单牌或对子"),
            (CardPattern::FourWithTwo, "四张+任意两张单牌或两对"),
            (CardPattern::Bomb, "四张点数相同的牌，可打任何非炸弹牌型"),
            (CardPattern::Rocket, "大王+小王，最大的牌型"),
        ]
    }

    /// 牌的大小顺序 (从小到大)
    pub fn card_order(&self) -> Vec<&'static str> {
        vec!["3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A", "2", "小王", "大王"]
    }

    /// 叫地主规则
    pub fn call_rules(&self) -> Vec<&'static str> {
        vec![
            "每人轮流叫分: 1分、2分、3分",
            "叫3分者直接成为地主",
            "无人叫分则重新发牌",
            "地主获得3张底牌",
        ]
    }

    /// 出牌规则
    pub fn play_rules(&self) -> Vec<&'static str> {
        vec![
            "地主先出牌",
            "按顺序轮流出牌",
            "必须出比上家大的同类型牌，或炸弹",
            "可以选择不出（过牌）",
            "直到有人出完所有牌",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "地主赢: 底分×2（春天×3）",
            "地主输: 底分×2（反春×3）",
            "炸弹翻倍: 每出一个炸弹翻倍",
        ]
    }
}

impl Default for DouDiZhuRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for DouDiZhuRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("doudizhu")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        let patterns = self.pattern_descriptions();
        format!(
            "【斗地主规则】\n\n\
            牌数: 地主{}张，农民{}张\n\n\
            叫地主:\n{}\n\n\
            牌型:\n{}\n\n\
            出牌规则:\n{}\n\n\
            计分规则:\n{}\n",
            self.landlord_cards(),
            self.farmer_cards(),
            self.call_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            patterns.iter().map(|(p, d)| format!("  • {}: {}", p.name(), d)).collect::<Vec<_>>().join("\n"),
            self.play_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doudizhu_rules() {
        let rules = DouDiZhuRules::new();
        assert_eq!(rules.landlord_cards(), 20);
        assert_eq!(rules.farmer_cards(), 17);
    }

    #[test]
    fn test_pattern_priority() {
        assert!(CardPattern::Rocket.priority() > CardPattern::Bomb.priority());
    }
}