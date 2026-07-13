//! 迷你桥牌规则 (Minibridge)
//!
//! 迷你桥牌是桥牌的简化版本，专为初学者设计。
//! 省略叫牌过程，直接根据点力决定定约，适合快速入门。
//!
//! # Examples
//!
//! ```rust
//! use world_rules::rules::games::bridge_minibridge::BridgeMinibridgeRules;
//! use world_rules::rules::core::Rule;
//!
//! let rules = BridgeMinibridgeRules::new();
//! assert_eq!(rules.metadata().name, "迷你桥牌规则");
//! assert!(!rules.explain().is_empty());
//! ```

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: BridgeMinibridgeRules,
    name: "迷你桥牌规则",
    desc: "迷你桥牌（Minibridge）简化规则，适合初学者",
    origin: "国际",
    tags: ["游戏", "卡牌", "桥牌", "入门"],
}

impl BridgeMinibridgeRules {
    /// 游戏概述
    pub fn overview(&self) -> Vec<&'static str> {
        vec![
            "迷你桥牌是桥牌的简化版本",
            "专为初学者和青少年设计",
            "省略复杂的叫牌过程",
            "保留桥牌的核心打牌技巧",
        ]
    }

    /// 发牌和明手
    pub fn deal_and_dummy(&self) -> Vec<&'static str> {
        vec![
            "每人发13张牌",
            "计算手中大牌点(A=4, K=3, Q=2, J=1)",
            "点力最高的人成为庄家",
            "庄家的搭档成为明手，摊牌",
        ]
    }

    /// 定约确定
    pub fn contract_determination(&self) -> Vec<&'static str> {
        vec![
            "庄家宣布定约花色",
            "可选择无将或有将",
            "庄家宣布定约级别",
            "明手牌面朝上放在桌上",
        ]
    }

    /// 打牌规则
    pub fn play_rules(&self) -> Vec<&'static str> {
        vec![
            "庄家左边的人首攻",
            "必须跟出同花色的牌",
            "无此花色可出其他牌",
            "将牌可吃其他花色",
            "每墩最大的牌赢得该墩",
        ]
    }

    /// 计分简化
    pub fn simplified_scoring(&self) -> Vec<&'static str> {
        vec![
            "完成定约得分",
            "墩分: 梅花/方块每墩20分",
            "墩分: 红心/黑桃每墩30分",
            "无将: 首墩40分，后续30分",
            "超墩和宕墩按简化规则计算",
        ]
    }

    /// 学习优势
    pub fn learning_benefits(&self) -> Vec<&'static str> {
        vec![
            "快速上手桥牌核心技巧",
            "无需学习叫牌体系",
            "练习打牌和信号技术",
            "逐步过渡到标准桥牌",
        ]
    }
}

impl Rule for BridgeMinibridgeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("bridge_minibridge")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "迷你桥牌规则 (Minibridge)",
            &[
                ("概述", &self.overview()),
                ("发牌和明手", &self.deal_and_dummy()),
                ("定约确定", &self.contract_determination()),
                ("打牌规则", &self.play_rules()),
                ("计分简化", &self.simplified_scoring()),
                ("学习优势", &self.learning_benefits()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge_minibridge_rules() {
        let rules = BridgeMinibridgeRules::new();
        assert_eq!(rules.metadata().name, "迷你桥牌规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.learning_benefits().is_empty());
    }
}
