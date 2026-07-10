//! 朝鲜将棋规则（Janggi / Korean Chess）
//!
//! 朝鲜半岛传统象棋游戏，与中国象棋类似但规则有所不同。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: JanggiRules,
    name: "朝鲜将棋规则",
    desc: "朝鲜将棋（Janggi）朝鲜半岛传统象棋规则",
    origin: "朝鲜",
    tags: ["游戏", "棋类", "象棋", "朝鲜", "将棋"],
}

impl JanggiRules {
    /// 历史背景
    pub fn section_history(&self) -> Vec<&'static str> {
        vec![
            "起源于朝鲜半岛，与中国象棋同源",
            "历史可追溯至11世纪",
            "与中国象棋规则相似但有独特变化",
            "现代规则定型于朝鲜王朝时期",
            "是朝鲜半岛流行的棋类游戏",
        ]
    }

    /// 棋盘设置
    pub fn section_board(&self) -> Vec<&'static str> {
        vec![
            "棋盘为9x10格（与中国象棋相同）",
            "棋盘有九宫格（宫殿）区域",
            "宫殿为3x3格，位于棋盘中央两侧",
            "宫殿内有斜线连接（允许斜走）",
            "棋盘没有河流分隔（与中国象棋不同）",
        ]
    }

    /// 棋子配置
    pub fn section_pieces(&self) -> Vec<&'static str> {
        vec![
            "每方16枚棋子：",
            "将（楚/汉）：1枚，类似帅/将",
            "士：2枚，可斜走（与中国象棋不同）",
            "象：2枚，走法独特（先直后斜）",
            "车：2枚，走法与中国象棋相同",
            "马：2枚，走法与中国象棋相同",
            "炮：2枚，走法与中国象棋不同（可不吃子跳）",
            "兵/卒：5枚，可横走（与中国象棋不同）",
        ]
    }

    /// 走法规则
    pub fn section_movement(&self) -> Vec<&'static str> {
        vec![
            "将（楚/汉）：宫内一格（可斜走）",
            "士：宫内一格（可斜走）",
            "象：先走一格直线，再斜走两格",
            "车：横竖任意距离（与中国象棋相同）",
            "马：L形跳跃（与中国象棋相同）",
            "炮：可跳任意棋子（不一定吃子）",
            "兵/卒：可向前或横向移动一格",
        ]
    }

    /// 初始布局
    pub fn section_initial(&self) -> Vec<&'static str> {
        vec![
            "棋子初始位置与中国象棋略有不同",
            "象和马的位置可以互换",
            "开局前可选择象马位置（独特规则）",
            "炮的初始位置固定",
            "兵/卒位置与中国象棋相同",
        ]
    }

    /// 特殊规则
    pub fn section_special(&self) -> Vec<&'static str> {
        vec![
            "无河流限制（象可过河）",
            "炮可以跳过棋子移动（不一定吃子）",
            "将可在宫内斜走",
            "兵可横向移动",
            "开局可选择象马布局",
        ]
    }

    /// 获胜条件
    pub fn section_win(&self) -> Vec<&'static str> {
        vec![
            "将死对方将即获胜",
            "困毙（对方无合法走棋）判胜",
            "双方将可面对面（不像中国象棋）",
            "和棋条件：三次重复局面",
            "和棋条件：双方同意",
        ]
    }

    /// 策略要点
    pub fn section_strategy(&self) -> Vec<&'static str> {
        vec![
            "象的走法更灵活，攻击力更强",
            "炮可以移动性跳跃，战术多样",
            "兵可横走，控制力更强",
            "宫内战斗更加复杂",
            "开局象马布局选择很重要",
        ]
    }
}

impl Rule for JanggiRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("janggi")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "朝鲜将棋规则",
            &[
                ("历史背景", &self.section_history()),
                ("棋盘设置", &self.section_board()),
                ("棋子配置", &self.section_pieces()),
                ("走法规则", &self.section_movement()),
                ("初始布局", &self.section_initial()),
                ("特殊规则", &self.section_special()),
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
    fn janggi_rules_basic() {
        let rules = JanggiRules::new();
        assert_eq!(rules.metadata().name, "朝鲜将棋规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn janggi_has_board_info() {
        let rules = JanggiRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("9x10"));
        assert!(explanation.contains("宫"));
    }

    #[test]
    fn janggi_has_pieces() {
        let rules = JanggiRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("将"));
        assert!(explanation.contains("象"));
        assert!(explanation.contains("炮"));
    }

    #[test]
    fn janggi_unique_rules() {
        let rules = JanggiRules::new();
        let explanation = rules.explain();
        // 检查朝鲜将棋独特规则
        assert!(explanation.contains("无河流") || explanation.contains("兵可横"));
    }
}
