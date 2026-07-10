//! 迷你将棋规则（Mini Shogi / 5x5 Shogi）
//!
//! 日本将棋的简化版本，棋盘为5x5，快速对弈。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: MiniShogiRules,
    name: "迷你将棋规则",
    desc: "迷你将棋（Mini Shogi）日本将棋简化版本，5x5棋盘快速对弈",
    origin: "日本",
    tags: ["游戏", "棋类", "将棋", "迷你", "简化"],
}

impl MiniShogiRules {
    /// 历史背景
    pub fn section_history(&self) -> Vec<&'static str> {
        vec![
            "日本将棋的简化变体",
            "20世纪发明",
            "棋盘缩小为5x5",
            "保持将棋核心规则",
            "适合快速对弈和学习",
        ]
    }

    /// 棋盘设置
    pub fn section_board(&self) -> Vec<&'static str> {
        vec![
            "棋盘为5x5格（标准将棋为9x9）",
            "每方有5枚棋子",
            "棋盘较小，游戏节奏快",
            "棋子初始位置固定",
            "棋盘使用坐标标记（a1-e5）",
        ]
    }

    /// 棋子配置
    pub fn section_pieces(&self) -> Vec<&'static str> {
        vec![
            "每方5枚棋子：",
            "王（王将）：1枚，核心棋子",
            "金将（金）：1枚，强力棋子",
            "银将（银）：1枚，斜走棋子",
            "角行（角）：1枚，斜向移动",
            "飞车（飞）：1枚，直线移动",
        ]
    }

    /// 走法规则
    pub fn section_movement(&self) -> Vec<&'static str> {
        vec![
            "王：周围一格任意方向",
            "金：前方、横向、斜前方、后方一格",
            "银：前方、斜向一格",
            "角：斜向任意距离",
            "飞：横向或纵向任意距离",
        ]
    }

    /// 升变规则
    pub fn section_promotion(&self) -> Vec<&'static str> {
        vec![
            "棋子进入对方阵地可升变",
            "对方阵地为最远一行",
            "银将升变为金将走法",
            "角行升变为\\\"马\\\"（可斜走+向前一格）",
            "飞车升变为\\\"龙\\\"（可直线+向前后一格）",
            "升变是可选的（部分情况强制）",
        ]
    }

    /// 持驹规则（打入）
    pub fn section_drop(&self) -> Vec<&'static str> {
        vec![
            "吃掉的棋子成为持驹",
            "持驹可随时打入空格",
            "打入视为一步走棋",
            "打入后棋子属于己方",
            "升变棋子打入后恢复原形",
            "不能打入导致立即将死",
        ]
    }

    /// 特殊限制
    pub fn section_restrictions(&self) -> Vec<&'static str> {
        vec![
            "二步禁止：不能在同一列打入两个未升变的步兵",
            "打入限制：不能打入无移动空间的棋子",
            "立即将死禁止：不能通过打入直接将死",
            "升变选择：部分棋子升变可选",
            "强制升变：角行飞车到达底线必须升变",
        ]
    }

    /// 获胜条件
    pub fn section_win(&self) -> Vec<&'static str> {
        vec![
            "将死对方王即获胜",
            "困毙（对方无合法走棋）判胜",
            "王被捕获（打入将死）判胜",
            "双方同意和棋",
            "循环局面特殊判定",
        ]
    }

    /// 策略要点
    pub fn section_strategy(&self) -> Vec<&'static str> {
        vec![
            "持驹打入是关键战术",
            "快速进攻节奏",
            "升变时机很重要",
            "王的安全第一",
            "小型棋盘战术紧凑",
        ]
    }

    /// 与标准将棋差异
    pub fn section_differences(&self) -> Vec<&'static str> {
        vec![
            "棋盘从9x9缩小为5x5",
            "棋子从20枚减少为5枚",
            "游戏时间大幅缩短",
            "规则保持将棋核心",
            "适合初学者入门",
        ]
    }
}

impl Rule for MiniShogiRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mini_shogi")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "迷你将棋规则",
            &[
                ("历史背景", &self.section_history()),
                ("棋盘设置", &self.section_board()),
                ("棋子配置", &self.section_pieces()),
                ("走法规则", &self.section_movement()),
                ("升变规则", &self.section_promotion()),
                ("持驹规则（打入）", &self.section_drop()),
                ("特殊限制", &self.section_restrictions()),
                ("获胜条件", &self.section_win()),
                ("策略要点", &self.section_strategy()),
                ("与标准将棋差异", &self.section_differences()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mini_shogi_rules_basic() {
        let rules = MiniShogiRules::new();
        assert_eq!(rules.metadata().name, "迷你将棋规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn mini_shogi_has_board_info() {
        let rules = MiniShogiRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("5x5"));
    }

    #[test]
    fn mini_shogi_has_pieces() {
        let rules = MiniShogiRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("王"));
        assert!(explanation.contains("金"));
        assert!(explanation.contains("飞"));
    }

    #[test]
    fn mini_shogi_drop_rules() {
        let rules = MiniShogiRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("持驹") || explanation.contains("打入"));
    }

    #[test]
    fn mini_shogi_promotion() {
        let rules = MiniShogiRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("升变"));
    }
}