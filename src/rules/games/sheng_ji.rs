//! 升级规则 (拖拉机)

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 升级规则
pub struct ShengJiRules {
    metadata: RuleMetadata,
}

impl ShengJiRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "升级规则",
                "升级扑克游戏规则(拖拉机)"
            )
            .with_origin("中国")
            .with_tags(vec!["游戏".into(), "扑克".into(), "升级".into()]),
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
            "四人游戏，两两对战",
            "使用两副牌共108张",
            "目标: 从2升到A",
            "先打完A的一方获胜",
            "每局根据得分升级",
        ]
    }

    /// 牌型说明
    pub fn pattern_descriptions(&self) -> Vec<&'static str> {
        vec![
            "单张: 任意一张牌",
            "对子: 两张相同点数的牌",
            "连对(拖拉机): 两对或更多连续对子",
            "三张: 三张相同点数的牌",
            "炸弹: 四张或更多相同点数的牌",
        ]
    }

    /// 主牌规则
    pub fn trump_rules(&self) -> Vec<&'static str> {
        vec![
            "主牌包括: 当前级别的牌、与主花色同花色的牌",
            "大小王始终是主牌",
            "主牌大于副牌",
            "同为主牌时按大小排序",
        ]
    }

    /// 升级规则
    pub fn level_up_rules(&self) -> Vec<&'static str> {
        vec![
            "得分80-120: 升1级",
            "得分120-160: 升2级",
            "得分160以上: 升3级",
            "得分0-40: 对方升1级",
            "得分40-80: 维持原级",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "5、10、K为分牌",
            "5分牌每张5分",
            "10分牌每张10分",
            "K分牌每张10分",
            "总分100分",
        ]
    }

    /// 亮主规则
    pub fn declare_trump_rules(&self) -> Vec<&'static str> {
        vec![
            "亮主: 出示一对当前级别的牌",
            "反主: 用更大的对子亮主",
            "没人亮主则翻底牌决定",
        ]
    }
}

impl Default for ShengJiRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ShengJiRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("sheng_ji")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【升级规则】\n\n\
            牌数: {}张(两副牌)\n\
            玩家: {}人\n\n\
            基本规则:\n{}\n\n\
            牌型:\n{}\n\n\
            主牌规则:\n{}\n\n\
            升级规则:\n{}\n\n\
            计分规则:\n{}\n",
            self.total_cards(),
            self.player_count(),
            self.basic_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.pattern_descriptions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.trump_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.level_up_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sheng_ji_rules() {
        let rules = ShengJiRules::new();
        assert_eq!(rules.player_count(), 4);
    }
}