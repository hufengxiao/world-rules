//! 飞行棋规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 飞机颜色
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaneColor {
    Red,
    Yellow,
    Blue,
    Green,
}

impl PlaneColor {
    pub fn name(&self) -> &'static str {
        match self {
            PlaneColor::Red => "红色",
            PlaneColor::Yellow => "黄色",
            PlaneColor::Blue => "蓝色",
            PlaneColor::Green => "绿色",
        }
    }
}

/// 飞行棋规则
pub struct AeroplaneChessRules {
    metadata: RuleMetadata,
}

impl AeroplaneChessRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "飞行棋规则",
                "飞行棋游戏规则"
            )
            .with_origin("中国")
            .with_tags(vec!["游戏".into(), "飞行棋".into()]),
        }
    }

    /// 玩家人数
    pub fn player_count(&self) -> u8 {
        4
    }

    /// 每人飞机数
    pub fn planes_per_player(&self) -> u8 {
        4
    }

    /// 棋盘格子数
    pub fn board_cells(&self) -> u8 {
        52 // 外圈
    }

    /// 基本规则
    pub fn basic_rules(&self) -> Vec<&'static str> {
        vec![
            "每人4架飞机",
            "掷骰子移动飞机",
            "掷出6可以起飞或再掷一次",
            "先让4架飞机到达终点者获胜",
            "飞机被撞回起点",
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "掷出6: 起飞一架飞机或移动后再掷一次",
            "踩到对方飞机: 对方飞机返回起点",
            "跳跃格: 飞到相同颜色的格子可跳跃",
            "捷径: 飞到指定格子可直接飞到远处",
            "终点: 必须掷出准确点数到达",
        ]
    }

    /// 骰子规则
    pub fn dice_rules(&self) -> Vec<&'static str> {
        vec![
            "掷1-5: 移动已起飞的飞机",
            "掷6: 起飞新飞机或移动后再掷一次",
            "连续掷三次6: 当前飞机返回起点",
        ]
    }

    /// 碰撞规则
    pub fn collision_rules(&self) -> Vec<&'static str> {
        vec![
            "踩到对方飞机: 对方返回起点",
            "两架同色飞机叠在一起: 组成叠机",
            "叠机可以一起移动",
            "叠机被踩: 两架都返回起点",
        ]
    }
}

impl Default for AeroplaneChessRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AeroplaneChessRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("aeroplane_chess")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【飞行棋规则】\n\n\
            玩家: {}人\n\
            每人飞机: {}架\n\n\
            基本规则:\n{}\n\n\
            特殊规则:\n{}\n\n\
            骰子规则:\n{}\n\n\
            碰撞规则:\n{}\n",
            self.player_count(),
            self.planes_per_player(),
            self.basic_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.special_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.dice_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.collision_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aeroplane_chess_rules() {
        let rules = AeroplaneChessRules::new();
        assert_eq!(rules.player_count(), 4);
        assert_eq!(rules.planes_per_player(), 4);
    }
}