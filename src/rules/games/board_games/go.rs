//! 围棋规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 围棋棋子颜色
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stone {
    Black,
    White,
}

impl Stone {
    pub fn opposite(&self) -> Self {
        match self {
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,
        }
    }
}

/// 围棋规则变体
#[derive(Debug, Clone)]
pub enum GoVariant {
    /// 中国规则
    Chinese,
    /// 日本规则
    Japanese,
    /// 韩国规则
    Korean,
    /// 应氏规则
    Ing,
}

/// 围棋规则
pub struct GoRules {
    metadata: RuleMetadata,
    variant: GoVariant,
    board_size: u8,
}

impl GoRules {
    pub fn new(board_size: u8) -> Self {
        Self {
            metadata: RuleMetadata::new(
                "围棋规则",
                "围棋标准规则说明"
            )
            .with_origin("中国"),
            variant: GoVariant::Chinese,
            board_size,
        }
    }

    pub fn with_variant(mut self, variant: GoVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn board_size(&self) -> u8 {
        self.board_size
    }

    /// 获取贴目规则
    pub fn komi(&self) -> f32 {
        match (&self.variant, self.board_size) {
            (GoVariant::Chinese, 19) => 7.5,
            (GoVariant::Japanese, 19) => 6.5,
            (GoVariant::Korean, 19) => 6.5,
            (GoVariant::Ing, 19) => 7.0,
            (_, 9) => 5.5,
            (_, 13) => 5.5,
            _ => 0.5,
        }
    }

    /// 计算胜负
    pub fn calculate_result(&self, black_territory: u32, white_territory: u32) -> GoResult {
        let adjusted_white = white_territory as f32 + self.komi();
        if black_territory as f32 > adjusted_white {
            GoResult::BlackWins(black_territory as f32 - adjusted_white)
        } else {
            GoResult::WhiteWins(adjusted_white - black_territory as f32)
        }
    }
}

/// 围棋结果
#[derive(Debug, Clone)]
pub enum GoResult {
    BlackWins(f32),
    WhiteWins(f32),
    Draw,
}

impl Rule for GoRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("go")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【围棋规则】\n\n\
            棋盘大小: {}×{}\n\
            贴目: {}目\n\n\
            基本规则:\n\
            1. 黑先白后，交替落子\n\
            2. 落子后不能移动\n\
            3. 气尽被提 (无气的棋子被吃掉)\n\
            4. 禁止全局同形再现 (打劫规则)\n\
            5. 终局计算地盘胜负\n\n\
            计分方式:\n\
            {}规则下，白方获得{}目贴目",
            self.board_size,
            self.board_size,
            self.komi(),
            match self.variant {
                GoVariant::Chinese => "中国",
                GoVariant::Japanese => "日本",
                GoVariant::Korean => "韩国",
                GoVariant::Ing => "应氏",
            },
            self.komi()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_go_rules() {
        let rules = GoRules::new(19);
        assert_eq!(rules.board_size(), 19);
        assert_eq!(rules.komi(), 7.5);
    }
}