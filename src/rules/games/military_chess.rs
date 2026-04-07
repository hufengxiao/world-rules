//! 军棋规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 军棋棋子
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MilitaryPiece {
    /// 司令
    Commander,
    /// 军长
    ArmyCommander,
    /// 师长
    DivisionCommander,
    /// 旅长
    BrigadeCommander,
    /// 团长
    RegimentCommander,
    /// 营长
    BattalionCommander,
    /// 连长
    CompanyCommander,
    /// 排长
    PlatoonCommander,
    /// 工兵
    Engineer,
    /// 地雷
    Mine,
    /// 炸弹
    Bomb,
    /// 军旗
    Flag,
}

impl MilitaryPiece {
    pub fn name(&self) -> &'static str {
        match self {
            MilitaryPiece::Commander => "司令",
            MilitaryPiece::ArmyCommander => "军长",
            MilitaryPiece::DivisionCommander => "师长",
            MilitaryPiece::BrigadeCommander => "旅长",
            MilitaryPiece::RegimentCommander => "团长",
            MilitaryPiece::BattalionCommander => "营长",
            MilitaryPiece::CompanyCommander => "连长",
            MilitaryPiece::PlatoonCommander => "排长",
            MilitaryPiece::Engineer => "工兵",
            MilitaryPiece::Mine => "地雷",
            MilitaryPiece::Bomb => "炸弹",
            MilitaryPiece::Flag => "军旗",
        }
    }

    /// 棋子大小 (用于吃子判断)
    pub fn rank(&self) -> u8 {
        match self {
            MilitaryPiece::Commander => 9,
            MilitaryPiece::ArmyCommander => 8,
            MilitaryPiece::DivisionCommander => 7,
            MilitaryPiece::BrigadeCommander => 6,
            MilitaryPiece::RegimentCommander => 5,
            MilitaryPiece::BattalionCommander => 4,
            MilitaryPiece::CompanyCommander => 3,
            MilitaryPiece::PlatoonCommander => 2,
            MilitaryPiece::Engineer => 1,
            MilitaryPiece::Mine => 0,
            MilitaryPiece::Bomb => 0,
            MilitaryPiece::Flag => 0,
        }
    }
}

/// 军棋规则
pub struct MilitaryChessRules {
    metadata: RuleMetadata,
}

impl MilitaryChessRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "军棋规则",
                "军棋游戏规则"
            )
            .with_origin("中国")
            .with_tags(vec!["游戏".into(), "军棋".into()]),
        }
    }

    /// 玩家人数
    pub fn player_count(&self) -> u8 {
        2 // 或4人
    }

    /// 每方棋子数
    pub fn pieces_per_side(&self) -> u8 {
        25
    }

    /// 基本规则
    pub fn basic_rules(&self) -> Vec<&'static str> {
        vec![
            "两人或四人对战",
            "每方25枚棋子",
            "目标: 夺取对方军旗",
            "大吃小，同级同归于尽",
            "棋子只能走直线或铁路线",
        ]
    }

    /// 吃子规则
    pub fn capture_rules(&self) -> Vec<&'static str> {
        vec![
            "大吃小: 司令最大",
            "同级相遇: 同归于尽",
            "炸弹: 与任何棋子同归于尽",
            "地雷: 只有工兵能挖",
            "工兵: 最小但能挖地雷",
            "军旗: 不能移动，被夺则输",
        ]
    }

    /// 特殊位置
    pub fn special_positions(&self) -> Vec<&'static str> {
        vec![
            "行营: 安全区，不能被吃",
            "大本营: 军旗位置",
            "铁路线: 可快速移动",
            "公路: 普通移动",
        ]
    }

    /// 棋子移动
    pub fn movement_rules(&self) -> Vec<&'static str> {
        vec![
            "普通棋子: 每次一步",
            "铁路线上: 可直线移动多步",
            "工兵: 铁路线上可转弯",
            "地雷和军旗: 不能移动",
        ]
    }

    /// 获胜条件
    pub fn winning_conditions(&self) -> Vec<&'static str> {
        vec![
            "夺取对方军旗",
            "对方无棋可走",
            "对方认输",
        ]
    }
}

impl Default for MilitaryChessRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MilitaryChessRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("military_chess")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【军棋规则】\n\n\
            玩家: {}人\n\
            每方棋子: {}枚\n\n\
            基本规则:\n{}\n\n\
            吃子规则:\n{}\n\n\
            特殊位置:\n{}\n\n\
            获胜条件:\n{}\n",
            self.player_count(),
            self.pieces_per_side(),
            self.basic_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.capture_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.special_positions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.winning_conditions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_military_chess_rules() {
        let rules = MilitaryChessRules::new();
        assert_eq!(rules.pieces_per_side(), 25);
    }
}