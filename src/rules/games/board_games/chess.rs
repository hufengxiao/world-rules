//! 国际象棋规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 国际象棋棋子类型
///
/// 表示国际象棋中六种不同的棋子类型。
///
/// # 示例
/// ```
/// use world_rules::rules::games::board_games::chess::ChessPieceType;
///
/// assert_eq!(ChessPieceType::King.name(), "王");
/// assert_eq!(ChessPieceType::Queen.english_name(), "Queen");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChessPieceType {
    /// 王 - 游戏的核心，被将死则输
    King,
    /// 后 - 最强的棋子，可横竖斜走
    Queen,
    /// 车 - 可横竖走，参与王车易位
    Rook,
    /// 象 - 可斜走，分黑白格象
    Bishop,
    /// 马 - L形走法，可跳子
    Knight,
    /// 兵 - 前进一步，吃子斜进，可升变
    Pawn,
}

impl ChessPieceType {
    /// 获取棋子中文名称
    ///
    /// # 示例
    /// ```
    /// use world_rules::rules::games::board_games::chess::ChessPieceType;
    ///
    /// assert_eq!(ChessPieceType::King.name(), "王");
    /// assert_eq!(ChessPieceType::Queen.name(), "后");
    /// ```
    pub fn name(&self) -> &'static str {
        match self {
            ChessPieceType::King => "王",
            ChessPieceType::Queen => "后",
            ChessPieceType::Rook => "车",
            ChessPieceType::Bishop => "象",
            ChessPieceType::Knight => "马",
            ChessPieceType::Pawn => "兵",
        }
    }

    /// 获取棋子英文名称
    ///
    /// # 示例
    /// ```
    /// use world_rules::rules::games::board_games::chess::ChessPieceType;
    ///
    /// assert_eq!(ChessPieceType::King.english_name(), "King");
    /// assert_eq!(ChessPieceType::Queen.english_name(), "Queen");
    /// ```
    pub fn english_name(&self) -> &'static str {
        match self {
            ChessPieceType::King => "King",
            ChessPieceType::Queen => "Queen",
            ChessPieceType::Rook => "Rook",
            ChessPieceType::Bishop => "Bishop",
            ChessPieceType::Knight => "Knight",
            ChessPieceType::Pawn => "Pawn",
        }
    }

    /// 获取棋子 Unicode 符号
    ///
    /// # 参数
    /// - `is_white`: 是否为白方棋子
    ///
    /// # 示例
    /// ```
    /// use world_rules::rules::games::board_games::chess::ChessPieceType;
    ///
    /// assert_eq!(ChessPieceType::King.symbol(true), "♔");
    /// assert_eq!(ChessPieceType::King.symbol(false), "♚");
    /// ```
    pub fn symbol(&self, is_white: bool) -> &'static str {
        match self {
            ChessPieceType::King => {
                if is_white {
                    "♔"
                } else {
                    "♚"
                }
            }
            ChessPieceType::Queen => {
                if is_white {
                    "♕"
                } else {
                    "♛"
                }
            }
            ChessPieceType::Rook => {
                if is_white {
                    "♖"
                } else {
                    "♜"
                }
            }
            ChessPieceType::Bishop => {
                if is_white {
                    "♗"
                } else {
                    "♝"
                }
            }
            ChessPieceType::Knight => {
                if is_white {
                    "♘"
                } else {
                    "♞"
                }
            }
            ChessPieceType::Pawn => {
                if is_white {
                    "♙"
                } else {
                    "♟"
                }
            }
        }
    }

    /// 棋子相对价值
    ///
    /// 返回棋子的相对价值分数，用于评估局面。
    ///
    /// # 示例
    /// ```
    /// use world_rules::rules::games::board_games::chess::ChessPieceType;
    ///
    /// assert_eq!(ChessPieceType::Queen.value(), 9);
    /// assert_eq!(ChessPieceType::Pawn.value(), 1);
    /// ```
    pub fn value(&self) -> u16 {
        match self {
            ChessPieceType::King => 1000,
            ChessPieceType::Queen => 9,
            ChessPieceType::Rook => 5,
            ChessPieceType::Bishop => 3,
            ChessPieceType::Knight => 3,
            ChessPieceType::Pawn => 1,
        }
    }
}

/// 国际象棋规则
///
/// 实现国际棋联（FIDE）标准国际象棋规则。
///
/// # 示例
/// ```
/// use world_rules::rules::games::board_games::chess::ChessRules;
///
/// let rules = ChessRules::new();
/// assert_eq!(rules.board_size(), 8);
/// assert_eq!(rules.promotion_pieces().len(), 4);
/// ```
pub struct ChessRules {
    metadata: RuleMetadata,
}

impl ChessRules {
    /// 创建新的国际象棋规则实例
    ///
    /// # 示例
    /// ```
    /// use world_rules::rules::games::board_games::chess::ChessRules;
    ///
    /// let rules = ChessRules::new();
    /// assert_eq!(rules.board_size(), 8);
    /// ```
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("国际象棋规则", "FIDE 国际象棋标准规则")
                .with_origin("国际")
                .with_tags(vec!["游戏".into(), "国际象棋".into()]),
        }
    }

    /// 获取棋盘大小
    ///
    /// # 示例
    /// ```
    /// use world_rules::rules::games::board_games::chess::ChessRules;
    ///
    /// let rules = ChessRules::new();
    /// assert_eq!(rules.board_size(), 8);
    /// ```
    pub fn board_size(&self) -> u8 {
        8 // 8×8棋盘
    }

    /// 获取棋子走法说明
    pub fn piece_movement_rules(&self) -> Vec<(ChessPieceType, &'static str)> {
        vec![
            (
                ChessPieceType::King,
                "王：可向任意方向移动一格，特殊情况下可王车易位",
            ),
            (
                ChessPieceType::Queen,
                "后：可横、竖、斜任意方向任意格数移动",
            ),
            (ChessPieceType::Rook, "车：只能横或竖直线移动任意格数"),
            (ChessPieceType::Bishop, "象：只能斜线移动任意格数"),
            (
                ChessPieceType::Knight,
                "马：走L字形（两格直线+一格斜线），可越子",
            ),
            (
                ChessPieceType::Pawn,
                "兵：前进一格（首次可两格），斜向吃子，可升变",
            ),
        ]
    }

    /// 兵的升变规则
    pub fn promotion_pieces(&self) -> Vec<ChessPieceType> {
        vec![
            ChessPieceType::Queen,
            ChessPieceType::Rook,
            ChessPieceType::Bishop,
            ChessPieceType::Knight,
        ]
    }

    /// 王车易位条件
    pub fn castling_conditions(&self) -> Vec<&'static str> {
        vec![
            "王和车都未曾移动过",
            "王和车之间没有其他棋子",
            "王不在将军状态",
            "王易位过程中不经过被攻击的格子",
            "王易位后不在将军状态",
        ]
    }

    /// 特殊规则说明
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "王车易位 (Castling): 王向车方向移动两格，车越过王放在王旁边",
            "兵的升变 (Promotion): 兵到达对方底线可升变为后/车/象/马",
            "吃过路兵 (En Passant): 兵首次走两格时可被对方兵斜吃",
            "将军 (Check): 王被攻击时必须解除将军",
            "将死 (Checkmate): 王被将军且无法解除则输",
            "和棋 (Stalemate): 无子可动但未被将军则和棋",
        ]
    }
}

impl Default for ChessRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ChessRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("chess")
    }

    fn explain(&self) -> String {
        let movements = self.piece_movement_rules();
        let special = self.special_rules();
        format!(
            "【国际象棋规则】\n\n\
            棋盘: {}×{}\n\
            白方先行\n\n\
            棋子走法:\n{}\n\n\
            特殊规则:\n{}\n\n\
            棋子价值参考:\n{}",
            self.board_size(),
            self.board_size(),
            movements
                .iter()
                .map(|(p, r)| format!("  • {}({}): {}", p.name(), p.symbol(true), r))
                .collect::<Vec<_>>()
                .join("\n"),
            special
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            [
                ChessPieceType::Queen,
                ChessPieceType::Rook,
                ChessPieceType::Bishop,
                ChessPieceType::Knight,
                ChessPieceType::Pawn
            ]
            .iter()
            .map(|p| format!("  • {}: {}", p.name(), p.value()))
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
        let rules = ChessRules::new();
        assert_eq!(rules.board_size(), 8);
        assert_eq!(rules.promotion_pieces().len(), 4);
    }
}
