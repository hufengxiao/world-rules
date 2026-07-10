//! 播棋规则（Mancala / Oware）
//!
//! 非洲古老棋类游戏，种子移动和捕获规则，历史悠久。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: MancalaRules,
    name: "播棋规则",
    desc: "播棋（Mancala/Oware）非洲古老种子棋类规则",
    origin: "非洲",
    tags: ["游戏", "棋类", "播棋", "非洲", "古老"],
}

impl MancalaRules {
    /// 历史背景
    pub fn section_history(&self) -> Vec<&'static str> {
        vec![
            "起源于非洲，历史超过3000年",
            "是最古老的棋类游戏之一",
            "传播至中东和亚洲",
            "不同地区有不同变体",
            "Oware是西非主要变体",
        ]
    }

    /// 棋盘设置
    pub fn section_board(&self) -> Vec<&'static str> {
        vec![
            "棋盘为两排各6个坑",
            "每方控制自己一侧的6个坑",
            "棋盘两端通常有存储坑",
            "每个坑初始放4颗种子",
            "总共48颗种子",
        ]
    }

    /// 基本规则
    pub fn section_basic(&self) -> Vec<&'static str> {
        vec![
            "两人对弈",
            "每方控制自己一侧的坑",
            "目标是捕获更多种子",
            "轮流进行播种和捕获",
            "游戏结束时种子多者获胜",
        ]
    }

    /// 播种规则
    pub fn section_sowing(&self) -> Vec<&'static str> {
        vec![
            "选择自己一侧的一个坑",
            "取出该坑所有种子",
            "按顺序一颗颗放入后续坑中",
            "播种方向通常逆时针",
            "跳过起始坑（不放种子）",
        ]
    }

    /// 捕获规则
    pub fn section_capture(&self) -> Vec<&'static str> {
        vec![
            "播种结束时检查对面坑",
            "若最后种子落在对面坑",
            "且对面坑种子数为2或3",
            "则捕获对面坑的种子",
            "可连续捕获（逆序）",
        ]
    }

    /// 连续捕获
    pub fn section_chain(&self) -> Vec<&'static str> {
        vec![
            "捕获后检查前一坑",
            "若前一坑也在对面且种子为2或3",
            "可继续捕获",
            "连续捕获直到条件不满足",
            "不能捕获起始侧的坑",
        ]
    }

    /// 特殊规则
    pub fn section_special(&self) -> Vec<&'static str> {
        vec![
            "Grand Slam：若捕获对方所有种子",
            "某些规则禁止Grand Slam",
            "若对方一侧无种子，必须喂子",
            "不能选择会导致对方无子的坑",
            "游戏结束条件各变体略有不同",
        ]
    }

    /// 终局规则
    pub fn section_endgame(&self) -> Vec<&'static str> {
        vec![
            "当一方捕获超过24颗种子获胜",
            "或双方各24颗为和棋",
            "一方无合法走棋时游戏结束",
            "剩余种子归控制方",
            "种子数多者获胜",
        ]
    }

    /// 策略要点
    pub fn section_strategy(&self) -> Vec<&'static str> {
        vec![
            "计算播种路径很重要",
            "创造连续捕获机会",
            "避免给对方留下好机会",
            "保持自己坑中种子分布",
            "终局计算种子数量",
        ]
    }

    /// 变体说明
    pub fn section_variants(&self) -> Vec<&'static str> {
        vec![
            "Oware：西非主流变体",
            "Bao：东非复杂变体",
            "Kalah：美国简化变体",
            "Warri：加勒比变体",
            "各地规则略有差异",
        ]
    }
}

impl Rule for MancalaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mancala")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "播棋规则",
            &[
                ("历史背景", &self.section_history()),
                ("棋盘设置", &self.section_board()),
                ("基本规则", &self.section_basic()),
                ("播种规则", &self.section_sowing()),
                ("捕获规则", &self.section_capture()),
                ("连续捕获", &self.section_chain()),
                ("特殊规则", &self.section_special()),
                ("终局规则", &self.section_endgame()),
                ("策略要点", &self.section_strategy()),
                ("变体说明", &self.section_variants()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mancala_rules_basic() {
        let rules = MancalaRules::new();
        assert_eq!(rules.metadata().name, "播棋规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn mancala_has_board_info() {
        let rules = MancalaRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("6个坑") || explanation.contains("坑"));
        assert!(explanation.contains("种子"));
    }

    #[test]
    fn mancala_has_sowing() {
        let rules = MancalaRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("播种"));
    }

    #[test]
    fn mancala_capture_rules() {
        let rules = MancalaRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("捕获"));
    }

    #[test]
    fn mancala_variants() {
        let rules = MancalaRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("Oware") || explanation.contains("变体"));
    }
}