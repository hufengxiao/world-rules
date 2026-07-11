//! 盘式桥牌规则 (Rubber Bridge)
//!
//! 盘式桥牌是最传统的桥牌形式，以"盘"(Rubber)为单位计分。
//! 先赢两盘的一方获胜，每盘由完成定约获得游戏分累积而成。
//!
//! # Examples
//!
//! ```rust
//! use world_rules::rules::games::bridge_rubber::BridgeRubberRules;
//! use world_rules::rules::core::Rule;
//!
//! let rules = BridgeRubberRules::new();
//! assert_eq!(rules.name(), "盘式桥牌规则");
//! assert!(!rules.explain().is_empty());
//! ```

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: BridgeRubberRules,
    name: "盘式桥牌规则",
    desc: "传统盘式桥牌（Rubber Bridge）完整规则",
    origin: "英国",
    tags: ["游戏", "卡牌", "桥牌"],
}

impl BridgeRubberRules {
    /// 游戏概述
    pub fn overview(&self) -> Vec<&'static str> {
        vec![
            "盘式桥牌是最传统的桥牌形式",
            "以盘(Rubber)为单位进行比赛",
            "先赢两盘的一方获胜",
            "适合4人娱乐对局",
        ]
    }

    /// 盘的定义
    pub fn rubber_definition(&self) -> Vec<&'static str> {
        vec![
            "一盘 = 一方获得100分游戏分",
            "游戏分来自完成的定约",
            "部分定约可累积游戏分",
            "成局定约立即获得一盘",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "梅花/方块: 每墩20分",
            "红心/黑桃: 每墩30分",
            "无将: 首墩40分，后续30分",
            "加倍后分数翻倍",
            "再加倍后分数四倍",
        ]
    }

    /// 奖分规则
    pub fn bonus_scores(&self) -> Vec<&'static str> {
        vec![
            "满贯: 小满贯500分，大满贯1000分",
            "超墩: 每墩50分(无局)或100分(有局)",
            "宕墩: 每墩50分(无局)或100分(有局)",
            "成局奖分: 300分(无局)或500分(有局)",
            "盘奖分: 700分(对手未成局)或500分(对手已成一盘)",
        ]
    }

    /// 局况规则
    pub fn vulnerability_rules(&self) -> Vec<&'static str> {
        vec![
            "开始时双方都是无局",
            "完成一盘后成为有局",
            "有局时宕墩罚分更高",
            "有局时满贯奖分更高",
        ]
    }
}

impl Rule for BridgeRubberRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("bridge_rubber")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "盘式桥牌规则 (Rubber Bridge)",
            &[
                ("概述", &self.overview()),
                ("盘的定义", &self.rubber_definition()),
                ("计分规则", &self.scoring_rules()),
                ("奖分规则", &self.bonus_scores()),
                ("局况规则", &self.vulnerability_rules()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge_rubber_rules() {
        let rules = BridgeRubberRules::new();
        assert_eq!(rules.name(), "盘式桥牌规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.overview().is_empty());
        assert!(!rules.scoring_rules().is_empty());
    }
}
