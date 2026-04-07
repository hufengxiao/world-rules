//! 中国象棋规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 棋子类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceType {
    /// 将/帅
    King,
    /// 士/仕
    Advisor,
    /// 象/相
    Elephant,
    /// 马
    Horse,
    /// 车
    Rook,
    /// 炮
    Cannon,
    /// 兵/卒
    Pawn,
}

impl PieceType {
    pub fn name(&self, is_red: bool) -> &'static str {
        match self {
            PieceType::King => if is_red { "帅" } else { "将" },
            PieceType::Advisor => if is_red { "仕" } else { "士" },
            PieceType::Elephant => if is_red { "相" } else { "象" },
            PieceType::Horse => "马",
            PieceType::Rook => "车",
            PieceType::Cannon => "炮",
            PieceType::Pawn => if is_red { "兵" } else { "卒" },
        }
    }

    /// 棋子价值 (大致参考)
    pub fn value(&self) -> u16 {
        match self {
            PieceType::King => 1000, // 无价
            PieceType::Rook => 9,
            PieceType::Cannon => 4,
            PieceType::Horse => 4,
            PieceType::Elephant => 2,
            PieceType::Advisor => 2,
            PieceType::Pawn => 1,
        }
    }
}

/// 棋子
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Piece {
    pub piece_type: PieceType,
    pub is_red: bool,
    pub position: (u8, u8), // (列, 行) 0-8列, 0-9行
}

/// 中国象棋规则
pub struct ChineseChessRules {
    metadata: RuleMetadata,
}

impl ChineseChessRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "中国象棋规则",
                "中国象棋标准规则"
            )
            .with_origin("中国")
            .with_tags(vec!["游戏".into(), "象棋".into()]),
        }
    }

    /// 棋盘大小
    pub fn board_size(&self) -> (u8, u8) {
        (9, 10) // 9列10行
    }

    /// 检查走法是否合法 (简化版)
    pub fn is_valid_move(&self, piece: &Piece, to: (u8, u8)) -> bool {
        let (x, y) = piece.position;
        let (to_x, to_y) = to;

        // 基本边界检查
        if to_x > 8 || to_y > 9 {
            return false;
        }

        match piece.piece_type {
            PieceType::King => {
                // 将帅只能在九宫内移动，每次一格
                let palace_x = 3..=5;
                let palace_y = if piece.is_red { 7..=9 } else { 0..=2 };
                palace_x.contains(&to_x) && palace_y.contains(&to_y)
                    && ((x == to_x && i32::from(y).saturating_sub(i32::from(to_y)).abs() == 1)
                        || (y == to_y && i32::from(x).saturating_sub(i32::from(to_x)).abs() == 1))
            }
            PieceType::Advisor => {
                // 士仕斜走一格，限九宫
                let palace_x = 3..=5;
                let palace_y = if piece.is_red { 7..=9 } else { 0..=2 };
                palace_x.contains(&to_x) && palace_y.contains(&to_y)
                    && i32::from(x).saturating_sub(i32::from(to_x)).abs() == 1
                    && i32::from(y).saturating_sub(i32::from(to_y)).abs() == 1
            }
            PieceType::Elephant => {
                // 象相走田字，不能过河
                let own_side = if piece.is_red { to_y >= 5 } else { to_y <= 4 };
                own_side
                    && i32::from(x).saturating_sub(i32::from(to_x)).abs() == 2
                    && i32::from(y).saturating_sub(i32::from(to_y)).abs() == 2
            }
            PieceType::Horse => {
                // 马走日字，需检查蹩马腿
                let dx = i32::from(x).saturating_sub(i32::from(to_x)).abs();
                let dy = i32::from(y).saturating_sub(i32::from(to_y)).abs();
                (dx == 1 && dy == 2) || (dx == 2 && dy == 1)
            }
            PieceType::Rook => {
                // 车直线移动
                x == to_x || y == to_y
            }
            PieceType::Cannon => {
                // 炮直线移动，吃子需隔一子
                x == to_x || y == to_y
            }
            PieceType::Pawn => {
                // 兵卒只能前进，过河后可左右
                let dx = i32::from(x).saturating_sub(i32::from(to_x)).abs();
                if piece.is_red {
                    if y >= 5 {
                        // 已过河
                        (to_y == y.saturating_sub(1) && x == to_x) || (y == to_y && dx == 1)
                    } else {
                        // 未过河只能前进
                        to_y == y.saturating_sub(1) && x == to_x
                    }
                } else {
                    if y <= 4 {
                        // 已过河
                        (to_y == y.saturating_add(1) && x == to_x) || (y == to_y && dx == 1)
                    } else {
                        // 未过河只能前进
                        to_y == y.saturating_add(1) && x == to_x
                    }
                }
            }
        }
    }

    /// 初始棋子位置
    pub fn initial_pieces(&self) -> Vec<Piece> {
        vec![
            // 红方
            Piece { piece_type: PieceType::Rook, is_red: true, position: (0, 9) },
            Piece { piece_type: PieceType::Horse, is_red: true, position: (1, 9) },
            Piece { piece_type: PieceType::Elephant, is_red: true, position: (2, 9) },
            Piece { piece_type: PieceType::Advisor, is_red: true, position: (3, 9) },
            Piece { piece_type: PieceType::King, is_red: true, position: (4, 9) },
            Piece { piece_type: PieceType::Advisor, is_red: true, position: (5, 9) },
            Piece { piece_type: PieceType::Elephant, is_red: true, position: (6, 9) },
            Piece { piece_type: PieceType::Horse, is_red: true, position: (7, 9) },
            Piece { piece_type: PieceType::Rook, is_red: true, position: (8, 9) },
            Piece { piece_type: PieceType::Cannon, is_red: true, position: (1, 7) },
            Piece { piece_type: PieceType::Cannon, is_red: true, position: (7, 7) },
            Piece { piece_type: PieceType::Pawn, is_red: true, position: (0, 6) },
            Piece { piece_type: PieceType::Pawn, is_red: true, position: (2, 6) },
            Piece { piece_type: PieceType::Pawn, is_red: true, position: (4, 6) },
            Piece { piece_type: PieceType::Pawn, is_red: true, position: (6, 6) },
            Piece { piece_type: PieceType::Pawn, is_red: true, position: (8, 6) },
            // 黑方
            Piece { piece_type: PieceType::Rook, is_red: false, position: (0, 0) },
            Piece { piece_type: PieceType::Horse, is_red: false, position: (1, 0) },
            Piece { piece_type: PieceType::Elephant, is_red: false, position: (2, 0) },
            Piece { piece_type: PieceType::Advisor, is_red: false, position: (3, 0) },
            Piece { piece_type: PieceType::King, is_red: false, position: (4, 0) },
            Piece { piece_type: PieceType::Advisor, is_red: false, position: (5, 0) },
            Piece { piece_type: PieceType::Elephant, is_red: false, position: (6, 0) },
            Piece { piece_type: PieceType::Horse, is_red: false, position: (7, 0) },
            Piece { piece_type: PieceType::Rook, is_red: false, position: (8, 0) },
            Piece { piece_type: PieceType::Cannon, is_red: false, position: (1, 2) },
            Piece { piece_type: PieceType::Cannon, is_red: false, position: (7, 2) },
            Piece { piece_type: PieceType::Pawn, is_red: false, position: (0, 3) },
            Piece { piece_type: PieceType::Pawn, is_red: false, position: (2, 3) },
            Piece { piece_type: PieceType::Pawn, is_red: false, position: (4, 3) },
            Piece { piece_type: PieceType::Pawn, is_red: false, position: (6, 3) },
            Piece { piece_type: PieceType::Pawn, is_red: false, position: (8, 3) },
        ]
    }

    /// 获取棋子走法说明
    pub fn piece_movement_rules(&self) -> Vec<(PieceType, &'static str)> {
        vec![
            (PieceType::King, "将帅：只能在九宫内移动，每次横或竖一格"),
            (PieceType::Advisor, "士仕：只能在九宫内斜走一格"),
            (PieceType::Elephant, "象相：走田字（斜走两格），不能过河，不能被塞象眼"),
            (PieceType::Horse, "马：走日字（先横/竖一格，再斜一格），会被蹩马腿"),
            (PieceType::Rook, "车：横竖直线任意移动，不能越子"),
            (PieceType::Cannon, "炮：移动时直线走，吃子时必须隔一个棋子（炮架）"),
            (PieceType::Pawn, "兵卒：未过河只能前进一格，过河后可左右移动一格，不能后退"),
        ]
    }
}

impl Default for ChineseChessRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ChineseChessRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("chinese_chess")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        let rules = self.piece_movement_rules();
        format!(
            "【中国象棋规则】\n\n\
            棋盘: {}×{}\n\
            红方先行\n\n\
            棋子走法:\n{}\n\n\
            特殊规则:\n\
            1. 将帅不能直接对面（飞将）\n\
            2. 吃掉对方将帅获胜\n\
            3. 困毙对方无子可动也算赢\n\
            4. 双方同意可和棋",
            self.board_size().0,
            self.board_size().1,
            rules.iter()
                .map(|(p, r)| format!("  • {}: {}", p.name(true), r))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chess_rules() {
        let rules = ChineseChessRules::new();
        assert_eq!(rules.board_size(), (9, 10));
        assert_eq!(rules.initial_pieces().len(), 32);
    }

    #[test]
    fn test_king_move() {
        let rules = ChineseChessRules::new();
        let king = Piece { piece_type: PieceType::King, is_red: true, position: (4, 9) };
        assert!(rules.is_valid_move(&king, (4, 8))); // 前进一格
        assert!(!rules.is_valid_move(&king, (4, 5))); // 出九宫
    }
}