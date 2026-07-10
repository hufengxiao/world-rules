//! 新加坡麻将规则
//!
//! 新加坡麻将使用动物牌和简化计分系统

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 新加坡麻将规则
pub struct SingaporeMahjongRules {
    metadata: RuleMetadata,
}

impl SingaporeMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("新加坡麻将规则", "新加坡地区流行的麻将规则")
                .with_origin("新加坡")
                .with_tags(vec!["游戏".into(), "麻将".into(), "新加坡".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用148张牌(含动物牌)",
            "每人起手13张牌",
            "东南西北四方",
            "东家为庄开始",
            "动物牌作为特色",
        ]
    }

    /// 动物牌规则
    pub fn animal_rules(&self) -> Vec<&'static str> {
        vec![
            "猫、鼠、鸡、蟑螂四种动物牌",
            "动物牌不参与牌组构成",
            "动物牌立即补牌",
            "动物牌可获额外得分",
            "动物牌计入结算",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<(&'static str, u8)> {
        vec![
            // 基本牌型
            ("平胡", 1),
            ("对对胡", 2),
            ("七对子", 3),
            // 花色牌型
            ("混一色", 2),
            ("清一色", 4),
            ("字一色", 6),
            // 特殊牌型
            ("天和", 6),
            ("地和", 4),
            ("十三幺", 6),
            ("全动物牌", 4),
            // 附加番
            ("自摸加番", 1),
            ("庄家加番", 1),
            ("动物牌每张1番", 1),
            ("花牌匹配加番", 1),
        ]
    }

    /// 花牌规则
    pub fn flower_rules(&self) -> Vec<&'static str> {
        vec![
            "四季牌和四种花牌",
            "花牌匹配座位加分",
            "花牌立即补牌",
            "花牌额外计分",
            "花牌不计入牌组",
        ]
    }

    /// 吃碰杠规则
    pub fn chi_peng_rules(&self) -> Vec<&'static str> {
        vec![
            "可以吃上家牌",
            "可以碰任意家牌",
            "可以明杠暗杠",
            "杠后补牌",
            "杠上可胡",
        ]
    }

    /// 庄家规则
    pub fn banker_rules(&self) -> Vec<&'static str> {
        vec![
            "东家为庄开始",
            "庄家胡牌连庄",
            "闲家胡牌轮庄",
            "庄家番数加倍",
            "流局庄家听牌连庄",
        ]
    }

    /// 结算规则
    pub fn settlement_rules(&self) -> Vec<&'static str> {
        vec![
            "自摸三家支付",
            "点炮一家支付",
            "底分乘番数",
            "动物牌额外计分",
            "花牌额外计分",
        ]
    }

    /// 游戏流程
    pub fn game_flow(&self) -> Vec<&'static str> {
        vec![
            "洗牌开门",
            "每人13张牌",
            "动物花牌立即补",
            "轮流摸打",
            "胡牌结束",
        ]
    }
}

impl Default for SingaporeMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SingaporeMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_singapore")
    }

    fn explain(&self) -> String {
        let scoring_list: String = self
            .scoring_rules()
            .iter()
            .map(|(name, fan)| format!("  • {}: {}番", name, fan))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "【新加坡麻将规则】\n\n\
            基本设置:\n{}\n\n\
            动物牌规则:\n{}\n\n\
            计分规则:\n{}\n\n\
            花牌规则:\n{}\n\n\
            结算规则:\n{}\n",
            self.basic_settings()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.animal_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            scoring_list,
            self.flower_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.settlement_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singapore_mahjong_rules() {
        let rules = SingaporeMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
        assert!(rules.scoring_rules().len() > 0);
    }

    #[test]
    fn test_singapore_animal_cards() {
        let rules = SingaporeMahjongRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("动物牌"));
        assert!(explanation.contains("148张"));
    }

    #[test]
    fn test_singapore_animal_scoring() {
        let rules = SingaporeMahjongRules::new();
        let scoring = rules.scoring_rules();
        assert!(scoring.iter().any(|(name, _)| name.contains("动物")));
    }
}
