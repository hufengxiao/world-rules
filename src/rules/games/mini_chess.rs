//! 迷你象棋规则 (Mini Chess)
//!
//! 缩小版国际象棋，棋盘为 5x6 或 4x5，简化规则便于快速对弈。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: MiniChessRules,
    name: "迷你象棋规则",
    desc: "迷你象棋(Mini Chess)简化版国际象棋规则",
    origin: "国际",
    tags: ["游戏", "棋类", "象棋", "简化"],
}

impl MiniChessRules {
    /// 棋盘规格
    pub fn section_board(&self) -> Vec<&'static str> {
        vec![
            "标准迷你棋盘：5x6格（5列6行）",
            "极简棋盘：4x5格（4列5行）",
            "棋盘颜色交替，白方在下方",
            "比标准棋盘小，游戏节奏更快",
        ]
    }

    /// 棋子配置
    pub fn section_pieces(&self) -> Vec<&'static str> {
        vec![
            "5x6棋盘棋子配置：",
            "  白方：王、后、象、马、车 + 5兵",
            "  黑方：王、后、象、马、车 + 5兵",
            "4x5棋盘棋子配置：",
            "  白方：王、后、马、车 + 4兵",
            "  黑方：王、后、马、车 + 4兵",
            "棋子走法与标准象棋相同",
        ]
    }

    /// 特殊规则
    pub fn section_special(&self) -> Vec<&'static str> {
        vec![
            "王车易位通常取消或简化",
            "兵升变规则保持（到达底线升变）",
            "无吃过路兵规则（棋盘太小）",
            "目标：将死对方王即获胜",
            "和棋条件与标准象棋类似",
        ]
    }

    /// 策略特点
    pub fn section_strategy(&self) -> Vec<&'static str> {
        vec![
            "棋盘小，战术更紧凑",
            "开局即进入中局",
            "错误更容易导致败局",
            "适合初学者和快速对弈",
            "平均游戏时间5-10分钟",
        ]
    }
}

impl Rule for MiniChessRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mini_chess")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "迷你象棋规则",
            &[
                ("棋盘规格", &self.section_board()),
                ("棋子配置", &self.section_pieces()),
                ("特殊规则", &self.section_special()),
                ("策略特点", &self.section_strategy()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mini_chess_rules_basic() {
        let rules = MiniChessRules::new();
        assert_eq!(rules.metadata().name, "迷你象棋规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn mini_chess_has_board_info() {
        let rules = MiniChessRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("5x6") || explanation.contains("4x5"));
    }
}