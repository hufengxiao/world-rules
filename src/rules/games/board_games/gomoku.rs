//! 五子棋规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 五子棋规则变体
#[derive(Debug, Clone)]
pub enum GomokuVariant {
    /// 标准五子棋
    Standard,
    /// 连珠规则 (有禁手)
    Renju,
    /// 自由规则
    FreeStyle,
}

/// 五子棋规则
pub struct GomokuRules {
    metadata: RuleMetadata,
    variant: GomokuVariant,
}

impl GomokuRules {
    pub fn new(variant: GomokuVariant) -> Self {
        Self {
            metadata: RuleMetadata::new(
                "五子棋规则",
                "五子棋标准规则"
            )
            .with_origin("中国")
            .with_tags(vec!["游戏".into(), "五子棋".into()]),
            variant,
        }
    }

    pub fn board_size(&self) -> (u8, u8) {
        // 标准棋盘 15×15
        (15, 15)
    }

    /// 连五获胜
    pub fn winning_length(&self) -> u8 {
        5
    }

    /// 禁手规则 (仅连珠规则)
    pub fn forbidden_moves(&self) -> Vec<&'static str> {
        match self.variant {
            GomokuVariant::Renju => vec![
                "三三禁手: 同时形成两个活三",
                "四四禁手: 同时形成两个四",
                "长连禁手: 连成六子或更多",
            ],
            _ => vec!["无禁手"],
        }
    }

    /// 开局规则
    pub fn opening_rules(&self) -> Vec<&'static str> {
        match self.variant {
            GomokuVariant::Renju => vec![
                "黑方先行",
                "前三手有开局限制",
                "五手两打: 第五手黑方下两个点，白方选一个",
            ],
            _ => vec![
                "黑方先行",
                "无开局限制",
            ],
        }
    }

    /// 检查是否获胜 (简化版)
    pub fn check_win(&self, board: &[[Option<bool>; 15]; 15], last_move: (u8, u8)) -> Option<bool> {
        let (x, y) = last_move;
        let player = board[x as usize][y as usize]?;

        // 检查四个方向
        let directions = [(1, 0), (0, 1), (1, 1), (1, -1)];

        for (dx, dy) in directions {
            let mut count = 1;

            // 正方向计数
            let mut i = 1;
            while let Some(Some(p)) = Self::get_cell(board, x, y, dx * i, dy * i) {
                if p == player { count += 1; i += 1; }
                else { break; }
            }

            // 反方向计数
            i = 1;
            while let Some(Some(p)) = Self::get_cell(board, x, y, -dx * i, -dy * i) {
                if p == player { count += 1; i += 1; }
                else { break; }
            }

            if count >= self.winning_length() {
                return Some(player);
            }
        }

        None
    }

    fn get_cell(board: &[[Option<bool>; 15]; 15], x: u8, y: u8, dx: i8, dy: i8) -> Option<Option<bool>> {
        let nx = (x as i8 + dx) as usize;
        let ny = (y as i8 + dy) as usize;
        if nx < 15 && ny < 15 {
            Some(board[nx][ny])
        } else {
            None
        }
    }
}

impl Rule for GomokuRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("gomoku")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【{}五子棋规则】\n\n\
            棋盘: {}×{}\n\
            获胜条件: 连成{}子\n\n\
            开局规则:\n{}\n\n\
            禁手规则:\n{}\n",
            match self.variant {
                GomokuVariant::Standard => "标准",
                GomokuVariant::Renju => "连珠",
                GomokuVariant::FreeStyle => "自由",
            },
            self.board_size().0,
            self.board_size().1,
            self.winning_length(),
            self.opening_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.forbidden_moves().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gomoku_rules() {
        let rules = GomokuRules::new(GomokuVariant::Standard);
        assert_eq!(rules.board_size(), (15, 15));
        assert_eq!(rules.winning_length(), 5);
    }
}