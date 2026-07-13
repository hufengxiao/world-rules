//! 芝加哥桥牌规则 (Chicago Bridge)
//!
//! 芝加哥桥牌又称四人桥牌，是一种简化的桥牌形式。
//! 每局只打4副牌，局况预先设定，适合休闲对局。
//!
//! # Examples
//!
//! ```rust
//! use world_rules::rules::games::bridge_chicago::BridgeChicagoRules;
//! use world_rules::rules::core::Rule;
//!
//! let rules = BridgeChicagoRules::new();
//! assert_eq!(rules.metadata().name, "芝加哥桥牌规则");
//! assert!(!rules.explain().is_empty());
//! ```

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: BridgeChicagoRules,
    name: "芝加哥桥牌规则",
    desc: "芝加哥桥牌（Chicago Bridge）四副牌规则",
    origin: "美国",
    tags: ["游戏", "卡牌", "桥牌"],
}

impl BridgeChicagoRules {
    /// 游戏概述
    pub fn overview(&self) -> Vec<&'static str> {
        vec![
            "芝加哥桥牌是一种简化的桥牌形式",
            "每局只打4副牌",
            "局况按固定顺序设定",
            "适合时间有限的对局",
        ]
    }

    /// 四副牌的局况
    pub fn vulnerability_sequence(&self) -> Vec<&'static str> {
        vec![
            "第1副: 双方无局",
            "第2副: 南北有局，东西无局",
            "第3副: 南北无局，东西有局",
            "第4副: 双方有局",
        ]
    }

    /// 计分特点
    pub fn scoring_features(&self) -> Vec<&'static str> {
        vec![
            "计算方法类似复式桥牌",
            "有局方宕墩罚分更高",
            "部分定约游戏分累积",
            "完成成局定约有奖分",
        ]
    }

    /// 与盘式桥牌的区别
    pub fn differences_from_rubber(&self) -> Vec<&'static str> {
        vec![
            "固定4副牌，无盘的概念",
            "局况预先设定，非累积",
            "更快的游戏节奏",
            "适合轮换座位",
        ]
    }

    /// 适用场景
    pub fn suitable_situations(&self) -> Vec<&'static str> {
        vec![
            "时间有限的对局",
            "新手练习桥牌",
            "休闲娱乐场合",
            "快速轮换赛制",
        ]
    }
}

impl Rule for BridgeChicagoRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("bridge_chicago")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "芝加哥桥牌规则 (Chicago Bridge)",
            &[
                ("概述", &self.overview()),
                ("四副牌局况", &self.vulnerability_sequence()),
                ("计分特点", &self.scoring_features()),
                ("与盘式桥牌的区别", &self.differences_from_rubber()),
                ("适用场景", &self.suitable_situations()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge_chicago_rules() {
        let rules = BridgeChicagoRules::new();
        assert_eq!(rules.metadata().name, "芝加哥桥牌规则");
        assert!(!rules.explain().is_empty());
        assert_eq!(rules.vulnerability_sequence().len(), 4);
    }
}
