//! 菲律宾麻将规则
//!
//! 菲律宾麻将受美国麻将影响，使用简化计分系统

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 菲律宾麻将规则
pub struct FilipinoMahjongRules {
    metadata: RuleMetadata,
}

impl FilipinoMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("菲律宾麻将规则", "菲律宾地区流行的麻将规则")
                .with_origin("菲律宾")
                .with_tags(vec!["游戏".into(), "麻将".into(), "菲律宾".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用144张牌(含花牌)",
            "每人起手13张牌",
            "东南西北四方",
            "东家为庄开始",
            "受美国麻将影响",
        ]
    }

    /// 花牌规则
    pub fn flower_rules(&self) -> Vec<&'static str> {
        vec![
            "使用四季牌和花牌",
            "花牌不参与牌组构成",
            "匹配座位花牌加分",
            "花牌立即补牌",
            "可使用花牌计分",
        ]
    }

    /// 简化计分
    pub fn simplified_scoring(&self) -> Vec<(&'static str, u8)> {
        vec![
            // 基本牌型
            ("平胡", 1),
            ("对对胡", 2),
            ("七对子", 3),
            // 花色牌型
            ("混一色", 2),
            ("清一色", 4),
            ("字一色", 8),
            // 特殊牌型
            ("天和", 6),
            ("地和", 4),
            ("十三幺", 6),
            // 附加番
            ("自摸加1番", 1),
            ("庄家加1番", 1),
            ("花牌匹配加1番", 1),
            ("杠上加1番", 1),
        ]
    }

    /// 吃碰杠规则
    pub fn chi_peng_rules(&self) -> Vec<&'static str> {
        vec![
            "可以吃上家牌",
            "可以碰任意家牌",
            "可以明杠暗杠",
            "杠后补牌",
            "吃碰后可胡",
        ]
    }

    /// 游戏流程
    pub fn game_flow(&self) -> Vec<&'static str> {
        vec![
            "庄家洗牌开门",
            "每人发13张牌",
            "花牌立即补牌",
            "轮流摸打",
            "胡牌本局结束",
        ]
    }

    /// 庄家规则
    pub fn banker_rules(&self) -> Vec<&'static str> {
        vec![
            "东家开始为庄",
            "庄家胡牌连庄",
            "闲家胡牌轮庄",
            "庄家番数加倍",
            "连庄庄家额外加分",
        ]
    }

    /// 结算规则
    pub fn settlement_rules(&self) -> Vec<&'static str> {
        vec![
            "自摸三家支付",
            "点炮一家支付",
            "底分乘番数",
            "庄家付双倍",
            "流局无人罚分",
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "可抢杠胡",
            "可海底捞",
            "可杠上花",
            "流局可听牌",
            "简化番种计算",
        ]
    }
}

impl Default for FilipinoMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FilipinoMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_filipino")
    }

    fn explain(&self) -> String {
        let scoring_list: String = self
            .simplified_scoring()
            .iter()
            .map(|(name, fan)| format!("  • {}: {}番", name, fan))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "【菲律宾麻将规则】\n\n\
            基本设置:\n{}\n\n\
            花牌规则:\n{}\n\n\
            简化计分:\n{}\n\n\
            庄家规则:\n{}\n\n\
            结算规则:\n{}\n",
            self.basic_settings()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.flower_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            scoring_list,
            self.banker_rules()
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
    fn test_filipino_mahjong_rules() {
        let rules = FilipinoMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
        assert!(rules.simplified_scoring().len() > 0);
    }

    #[test]
    fn test_filipino_simplified_scoring() {
        let rules = FilipinoMahjongRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("简化"));
        assert!(explanation.contains("144张"));
    }
}
