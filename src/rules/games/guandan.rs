//! 掼蛋规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 掼蛋牌型
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuanDanPattern(&'static str);

/// 掼蛋规则
pub struct GuanDanRules {
    metadata: RuleMetadata,
}

impl GuanDanRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "掼蛋规则",
                "掼蛋扑克游戏规则"
            )
            .with_origin("中国")
            .with_tags(vec!["游戏".into(), "扑克".into(), "掼蛋".into()]),
        }
    }

    /// 牌数
    pub fn total_cards(&self) -> u8 {
        108 // 两副牌
    }

    /// 玩家人数
    pub fn player_count(&self) -> u8 {
        4
    }

    /// 基本规则
    pub fn basic_rules(&self) -> Vec<&'static str> {
        vec![
            "四人对战，两人一组",
            "使用两副牌共108张",
            "每人27张牌",
            "目标是打完手中的牌",
            "先打完一方获胜",
        ]
    }

    /// 牌型说明
    pub fn pattern_descriptions(&self) -> Vec<&'static str> {
        vec![
            "单张: 任意一张牌",
            "对子: 两张相同点数",
            "三张: 三张相同点数",
            "三带二: 三张+一对",
            "顺子: 五张或更多连续单牌",
            "连对: 三对或更多连续对子",
            "钢板(双飞): 两个或更多连续三张",
            "炸弹: 四张或更多相同点数",
            "同花顺: 五张同花色顺子",
            "四大天王: 四张大王或小王",
        ]
    }

    /// 牌型大小
    pub fn hand_ranking(&self) -> Vec<&'static str> {
        vec![
            "四大天王 > 同花顺 > 炸弹",
            "炸弹张数越多越大",
            "同炸弹张数时比较点数",
            "红桃级牌可配任何牌",
        ]
    }

    /// 级牌规则
    pub fn level_rules(&self) -> Vec<&'static str> {
        vec![
            "从2打起，依次升级",
            "打几级，几级的牌为级牌",
            "红桃级牌是百搭牌",
            "级牌可配成任意牌型",
        ]
    }

    /// 逢人配规则
    pub fn wildcard_rules(&self) -> Vec<&'static str> {
        vec![
            "红桃级牌是逢人配",
            "可以代替任意牌",
            "可以多张级牌配",
            "大王小王不能配",
        ]
    }

    /// 进贡规则
    pub fn tribute_rules(&self) -> Vec<&'static str> {
        vec![
            "下游给上游进贡最大牌",
            "末游给头游进贡",
            "还贡: 接受进贡后还一张牌",
            "抗贡: 有大王可抗贡",
        ]
    }

    /// 升级规则
    pub fn level_up_rules(&self) -> Vec<&'static str> {
        vec![
            "头游方升级",
            "双下(头游末游同组): 升3级",
            "单下: 升2级",
            "打A必须头游才能赢",
        ]
    }
}

impl Default for GuanDanRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for GuanDanRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("guandan")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【掼蛋规则】\n\n\
            牌数: {}张(两副牌)\n\
            玩家: {}人\n\n\
            基本规则:\n{}\n\n\
            牌型说明:\n{}\n\n\
            级牌规则:\n{}\n\n\
            进贡规则:\n{}\n\n\
            升级规则:\n{}\n",
            self.total_cards(),
            self.player_count(),
            self.basic_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.pattern_descriptions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.level_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.tribute_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.level_up_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guandan_rules() {
        let rules = GuanDanRules::new();
        assert_eq!(rules.player_count(), 4);
    }
}