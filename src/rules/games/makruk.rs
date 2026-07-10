//! 泰国象棋规则（Makruk / Thai Chess）
//!
//! 泰国传统象棋游戏，与国际象棋同源但保留了古老规则特征。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: MakrukRules,
    name: "泰国象棋规则",
    desc: "泰国象棋（Makruk）泰国传统象棋规则，保留古老象棋特征",
    origin: "泰国",
    tags: ["游戏", "棋类", "象棋", "泰国", "传统"],
}

impl MakrukRules {
    /// 历史背景
    pub fn section_history(&self) -> Vec<&'static str> {
        vec![
            "起源于泰国，历史超过1000年",
            "从印度查图兰加经东南亚传入",
            "保留了更多古代象棋特征",
            "与国际象棋同源但演变不同",
            "在泰国和柬埔寨广泛流行",
        ]
    }

    /// 棋盘设置
    pub fn section_board(&self) -> Vec<&'static str> {
        vec![
            "棋盘为8x8格（与国际象棋相同）",
            "棋盘无颜色交替标记",
            "初始布局与国际象棋类似",
            "棋子放在格内而非交叉点",
            "棋盘称为\\\"Ban Chan\\\"",
        ]
    }

    /// 棋子配置
    pub fn section_pieces(&self) -> Vec<&'static str> {
        vec![
            "每方16枚棋子：",
            "Khun（王）：类似国际象棋王",
            "Met（后）：较弱，只能斜走一格",
            "Khon（士）：可斜走或向前一格",
            "Ma（马）：与国际象棋马相同",
            "Ruea（车）：与国际象棋车相同",
            "Bia（兵）：与国际象棋兵类似",
        ]
    }

    /// 走法规则
    pub fn section_movement(&self) -> Vec<&'static str> {
        vec![
            "Khun（王）：周围一格任意方向",
            "Met（后）：只能斜向一格（较弱）",
            "Khon（士）：斜向一格或向前一格",
            "Ma（马）：L形跳跃",
            "Ruea（车）：横竖任意距离",
            "Bia（兵）：向前一格，吃棋斜前",
        ]
    }

    /// 兵升变规则
    pub fn section_promotion(&self) -> Vec<&'static str> {
        vec![
            "兵到达第六行可升变",
            "升变为Met（后）",
            "升变后的后仍只能斜走一格",
            "升变是可选的（不强制）",
            "多个兵可升变",
        ]
    }

    /// 特殊规则
    pub fn section_special(&self) -> Vec<&'static str> {
        vec![
            "无王车易位规则",
            "无吃过路兵规则",
            "后走法较弱（保留古代特征）",
            "士走法独特（斜走或向前）",
            "将死即获胜",
        ]
    }

    /// 计时规则（独特）
    pub fn section_timing(&self) -> Vec<&'static str> {
        vec![
            "传统规则：计数制",
            "当一方只剩王时开始计数",
            "对方需在规定步数内获胜",
            "计数：64步内必须获胜",
            "超时则判和棋",
        ]
    }

    /// 获胜条件
    pub fn section_win(&self) -> Vec<&'static str> {
        vec![
            "将死对方王即获胜",
            "对方无合法走棋判胜",
            "吃光对方所有棋子判胜",
            "计数超时判和",
            "双方同意和棋",
        ]
    }

    /// 策略要点
    pub fn section_strategy(&self) -> Vec<&'static str> {
        vec![
            "后较弱，车更加重要",
            "士可向前移动，灵活性强",
            "兵升变后仍较弱",
            "中局战斗节奏较慢",
            "终局计算计数很重要",
        ]
    }
}

impl Rule for MakrukRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("makruk")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "泰国象棋规则",
            &[
                ("历史背景", &self.section_history()),
                ("棋盘设置", &self.section_board()),
                ("棋子配置", &self.section_pieces()),
                ("走法规则", &self.section_movement()),
                ("兵升变规则", &self.section_promotion()),
                ("特殊规则", &self.section_special()),
                ("计时规则", &self.section_timing()),
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
    fn makruk_rules_basic() {
        let rules = MakrukRules::new();
        assert_eq!(rules.metadata().name, "泰国象棋规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn makruk_has_board_info() {
        let rules = MakrukRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("8x8"));
    }

    #[test]
    fn makruk_has_pieces() {
        let rules = MakrukRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("Khun") || explanation.contains("王"));
        assert!(explanation.contains("Met") || explanation.contains("后"));
    }

    #[test]
    fn makruk_unique_rules() {
        let rules = MakrukRules::new();
        let explanation = rules.explain();
        // 检查泰国象棋独特规则（后较弱）
        assert!(explanation.contains("只能斜走") || explanation.contains("较弱"));
    }
}
