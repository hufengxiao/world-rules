//! 马来西亚麻将规则
//!
//! 马来西亚麻将使用动物牌和独特计分系统

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 马来西亚麻将规则
pub struct MalaysianMahjongRules {
    metadata: RuleMetadata,
}

impl MalaysianMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("马来西亚麻将规则", "马来西亚地区流行的麻将规则")
                .with_origin("马来西亚")
                .with_tags(vec!["游戏".into(), "麻将".into(), "马来西亚".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用148张牌(含动物牌)",
            "每人起手13张牌",
            "东南西北四方",
            "东家为庄开始",
            "动物牌特色明显",
        ]
    }

    /// 动物牌规则
    pub fn animal_rules(&self) -> Vec<&'static str> {
        vec![
            "猫、老鼠、公鸡、蟑螂四种动物牌",
            "动物牌不参与牌组构成",
            "动物牌立即补牌",
            "收集全部动物牌获高分",
            "动物牌在结算时计入",
        ]
    }

    /// 特殊番种
    pub fn special_fan_types(&self) -> Vec<(&'static str, u8)> {
        vec![
            // 动物牌番种
            ("全动物", 8),
            ("三动物", 4),
            ("两动物", 2),
            ("单动物", 1),
            // 基本牌型
            ("平胡", 1),
            ("对对胡", 3),
            ("七对子", 4),
            // 花色牌型
            ("混一色", 3),
            ("清一色", 6),
            ("字一色", 10),
            // 高级牌型
            ("天和", 10),
            ("地和", 8),
            ("十三幺", 10),
            ("大三元", 10),
            ("四暗刻", 8),
            // 附加番
            ("自摸加番", 1),
            ("庄家加倍", 2),
            ("杠上加番", 2),
        ]
    }

    /// 花牌规则
    pub fn flower_rules(&self) -> Vec<&'static str> {
        vec![
            "四季牌和四种花牌",
            "花牌匹配座位加番",
            "花牌立即补牌",
            "收集全部花牌高分",
            "花牌不计入牌组",
        ]
    }

    /// 吃碰杠规则
    pub fn chi_peng_rules(&self) -> Vec<&'static str> {
        vec![
            "可以吃上家牌",
            "可以碰任意家牌",
            "可以明杠暗杠",
            "杠后必须补牌",
            "杠上可以胡牌",
        ]
    }

    /// 庄家规则
    pub fn banker_rules(&self) -> Vec<&'static str> {
        vec![
            "东家为庄开始",
            "庄家胡牌连庄",
            "闲家胡牌轮庄",
            "庄家番数翻倍",
            "连续连庄额外加分",
        ]
    }

    /// 结算规则
    pub fn settlement_rules(&self) -> Vec<&'static str> {
        vec![
            "自摸三家支付",
            "点炮一家支付",
            "底分乘总番数",
            "动物牌额外计分",
            "花牌额外计分",
        ]
    }

    /// 游戏流程
    pub fn game_flow(&self) -> Vec<&'static str> {
        vec![
            "洗牌开门",
            "每人13张起手",
            "动物花牌立即补",
            "轮流摸打牌",
            "胡牌本局结束",
        ]
    }

    /// 特殊规定
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "收集全部动物牌有奖",
            "抢杠胡可获高分",
            "海底捞月加分",
            "杠上开花加分",
            "流局罚分规则",
        ]
    }
}

impl Default for MalaysianMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MalaysianMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_malaysian")
    }

    fn explain(&self) -> String {
        let fan_list: String = self
            .special_fan_types()
            .iter()
            .map(|(name, fan)| format!("  • {}: {}番", name, fan))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "【马来西亚麻将规则】\n\n\
            基本设置:\n{}\n\n\
            动物牌规则:\n{}\n\n\
            特殊番种:\n{}\n\n\
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
            fan_list,
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
    fn test_malaysian_mahjong_rules() {
        let rules = MalaysianMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
        assert!(rules.special_fan_types().len() > 0);
    }

    #[test]
    fn test_malaysian_animal_cards() {
        let rules = MalaysianMahjongRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("动物牌"));
        assert!(explanation.contains("148张"));
    }

    #[test]
    fn test_malaysian_special_fans() {
        let rules = MalaysianMahjongRules::new();
        let fans = rules.special_fan_types();
        assert!(fans.iter().any(|(name, _)| name.contains("动物")));
        assert!(fans.iter().any(|(name, _)| *name == "清一色"));
    }

    #[test]
    fn test_malaysian_animal_scoring() {
        let rules = MalaysianMahjongRules::new();
        let fans = rules.special_fan_types();
        // 验证动物牌番种计分
        let full_animal = fans.iter().find(|(name, _)| *name == "全动物");
        assert!(full_animal.is_some());
        assert_eq!(full_animal.unwrap().1, 8);
    }
}
