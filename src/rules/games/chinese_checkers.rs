//! 跳棋规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 跳棋规则
pub struct ChineseCheckersRules {
    metadata: RuleMetadata,
}

impl ChineseCheckersRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "跳棋规则",
                "中国跳棋游戏规则"
            )
            .with_origin("中国")
            .with_tags(vec!["游戏".into(), "跳棋".into()]),
        }
    }

    /// 玩家人数范围
    pub fn player_range(&self) -> (u8, u8) {
        (2, 6)
    }

    /// 每人棋子数
    pub fn pieces_per_player(&self) -> u8 {
        10
    }

    /// 基本规则
    pub fn basic_rules(&self) -> Vec<&'static str> {
        vec![
            "棋盘为六角星形",
            "每人10颗棋子",
            "目标: 将所有棋子移到对面",
            "先到达者获胜",
            "可以单人或多人游戏",
        ]
    }

    /// 移动规则
    pub fn movement_rules(&self) -> Vec<&'static str> {
        vec![
            "单步移动: 移动到相邻空位",
            "跳跃: 跳过相邻棋子到空位",
            "连跳: 可以连续跳跃",
            "跳跃方向: 可以向六个方向跳跃",
            "不能移动到已有棋子的位置",
        ]
    }

    /// 策略技巧
    pub fn strategies(&self) -> Vec<&'static str> {
        vec![
            "搭建跳跃桥梁",
            "保持棋子连贯",
            "避免阻挡自己的棋子",
            "利用对方棋子跳跃",
            "优先移动后方的棋子",
        ]
    }

    /// 游戏模式
    pub fn game_modes(&self) -> Vec<&'static str> {
        vec![
            "单人: 两人对战",
            "双人: 四人两两对战",
            "三人: 六人三方对战",
            "混战: 六人各自为战",
        ]
    }
}

impl Default for ChineseCheckersRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ChineseCheckersRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("chinese_checkers")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【跳棋规则】\n\n\
            玩家: {}-{}人\n\
            每人棋子: {}颗\n\n\
            基本规则:\n{}\n\n\
            移动规则:\n{}\n\n\
            策略技巧:\n{}\n\n\
            游戏模式:\n{}\n",
            self.player_range().0,
            self.player_range().1,
            self.pieces_per_player(),
            self.basic_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.movement_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.strategies().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.game_modes().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chinese_checkers_rules() {
        let rules = ChineseCheckersRules::new();
        assert_eq!(rules.pieces_per_player(), 10);
    }
}