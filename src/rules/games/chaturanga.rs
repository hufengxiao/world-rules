//! 查图兰加规则 (Chaturanga)
//!
//! 印度古代棋类游戏，被认为是国际象棋和中国象棋的共同祖先。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ChaturangaRules,
    name: "查图兰加规则",
    desc: "查图兰加(Chaturanga)印度古代象棋规则，国际象棋祖先",
    origin: "印度",
    tags: ["游戏", "棋类", "象棋", "古代", "历史"],
}

impl ChaturangaRules {
    /// 历史背景
    pub fn section_history(&self) -> Vec<&'static str> {
        vec![
            "起源于公元6世纪的印度",
            "名称意为\"四支队\"（四种军种）",
            "被认为是国际象棋和中国象棋的共同祖先",
            "后传播至波斯演化为查图兰格",
            "再传播至欧洲演化为国际象棋",
        ]
    }

    /// 棋盘设置
    pub fn section_board(&self) -> Vec<&'static str> {
        vec![
            "8x8格棋盘（与现代象棋相同）",
            "棋盘无颜色交替（原始版本）",
            "四个角落各有特殊标记",
            "棋盘称为\"Ashtapada\"",
        ]
    }

    /// 棋子配置
    pub fn section_pieces(&self) -> Vec<&'static str> {
        vec![
            "每方16枚棋子，四种类型：",
            "Raja（王）：类似现代的王",
            "Mantri（大臣）：类似后，但走法较弱",
            "Ratha（战车）：类似车，走法相同",
            "Ashva（马）：类似马，走法相同",
            "Gaja（象）：斜走两格（与现代象不同）",
            "Padati（步兵）：类似兵，只能向前一格",
        ]
    }

    /// 走法规则
    pub fn section_movement(&self) -> Vec<&'static str> {
        vec![
            "Raja（王）：周围一格",
            "Mantri（大臣）：斜向一格（较弱）",
            "Ratha（车）：横竖任意距离",
            "Ashva（马）：L形跳跃",
            "Gaja（象）：斜向跳两格",
            "Padati（兵）：向前一格，吃棋也向前",
        ]
    }

    /// 特殊规则
    pub fn section_special(&self) -> Vec<&'static str> {
        vec![
            "无王车易位（现代规则）",
            "兵到达底线可升变为大臣",
            "无吃过路兵规则",
            "将死对方王即获胜",
            "原始版本可能为四人游戏",
        ]
    }

    /// 文化意义
    pub fn section_culture(&self) -> Vec<&'static str> {
        vec![
            "反映古代印度军事四分队体制",
            "象征国王、大臣、战车、骑兵、象兵、步兵",
            "是棋类游戏发展史的重要里程碑",
            "影响深远，传播至世界各地",
            "现代象棋的\"象\"名称源于此",
        ]
    }
}

impl Rule for ChaturangaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("chaturanga")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "查图兰加规则",
            &[
                ("历史背景", &self.section_history()),
                ("棋盘设置", &self.section_board()),
                ("棋子配置", &self.section_pieces()),
                ("走法规则", &self.section_movement()),
                ("特殊规则", &self.section_special()),
                ("文化意义", &self.section_culture()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chaturanga_rules_basic() {
        let rules = ChaturangaRules::new();
        assert_eq!(rules.metadata().name, "查图兰加规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn chaturanga_has_history() {
        let rules = ChaturangaRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("印度") || explanation.contains("祖先"));
    }
}
