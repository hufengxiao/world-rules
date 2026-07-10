//! 暗棋规则 (Dark Chess / Chinese Dark Chess)
//!
//! 中国象棋的隐藏棋子版本，棋子翻转后才可见。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: DarkChessRules,
    name: "暗棋规则",
    desc: "暗棋(Dark Chess)中国象棋隐藏棋子版本规则",
    origin: "中国",
    tags: ["游戏", "棋类", "象棋", "暗棋", "翻棋"],
}

impl DarkChessRules {
    /// 棋盘设置
    pub fn section_board(&self) -> Vec<&'static str> {
        vec![
            "棋盘为4x8格（32格）",
            "棋子初始全部翻转朝下（隐藏）",
            "棋子随机摆放，双方都不知道位置",
            "红方和黑方各16枚棋子",
        ]
    }

    /// 棋子类型
    pub fn section_pieces(&self) -> Vec<&'static str> {
        vec![
            "每方棋子：帅/将1、仕/士2、相/象2、车2、马2、炮2、兵/卒5",
            "棋子等级：帅>仕>相>车>马>炮>兵",
            "特殊：炮可隔子吃（与明棋不同）",
            "兵卒只可吃兵卒（不能吃将帅）",
        ]
    }

    /// 翻棋规则
    pub fn section_flip(&self) -> Vec<&'static str> {
        vec![
            "每回合可选择：翻棋或走棋/吃棋",
            "翻棋：翻开任意一格未翻开的棋子",
            "翻开后棋子颜色和类型公开",
            "首次翻棋决定玩家方（翻开红子为红方）",
        ]
    }

    /// 走棋与吃棋
    pub fn section_move(&self) -> Vec<&'static str> {
        vec![
            "已翻开的棋子可移动一格（上下左右）",
            "大棋子可吃小棋子或同级棋子",
            "炮需隔一个棋子（任意颜色）吃棋",
            "兵卒只能吃兵卒，但帅将可吃兵卒",
            "同色棋子不能互相吃",
        ]
    }

    /// 获胜条件
    pub fn section_win(&self) -> Vec<&'static str> {
        vec![
            "吃掉对方将帅即获胜",
            "吃光对方所有棋子即获胜",
            "对方无棋可走判负",
            "和棋：双方都无法获胜",
            "时间限制：通常每方限时10-15分钟",
        ]
    }

    /// 策略要点
    pub fn section_strategy(&self) -> Vec<&'static str> {
        vec![
            "翻棋策略很重要，避免翻出对方大棋",
            "尽量先翻出自己的大棋",
            "保护好自己的帅/将",
            "炮在暗棋中威力更大",
            "兵卒虽小但可阻挡帅将",
        ]
    }
}

impl Rule for DarkChessRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("dark_chess")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "暗棋规则",
            &[
                ("棋盘设置", &self.section_board()),
                ("棋子类型", &self.section_pieces()),
                ("翻棋规则", &self.section_flip()),
                ("走棋与吃棋", &self.section_move()),
                ("获胜条件", &self.section_win()),
                ("策略要点", &self.section_strategy()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dark_chess_rules_basic() {
        let rules = DarkChessRules::new();
        assert_eq!(rules.metadata().name, "暗棋规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn dark_chess_has_flip_rules() {
        let rules = DarkChessRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("翻棋"));
    }
}