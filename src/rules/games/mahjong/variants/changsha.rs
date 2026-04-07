//! 长沙麻将规则
//!
//! 长沙麻将特点是"跑得快"式计分，讲究"将"牌

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 长沙麻将规则
pub struct ChangshaMahjongRules {
    metadata: RuleMetadata,
}

impl ChangshaMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "长沙麻将规则",
                "长沙地区流行麻将规则"
            )
            .with_origin("长沙")
            .with_tags(vec!["游戏".into(), "麻将".into(), "长沙".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌(无花牌)",
            "每人起手13张",
            "可以吃碰杠",
            "只能自摸胡牌",
            "讲究将牌",
        ]
    }

    /// 将牌规则
    pub fn jiang_rules(&self) -> Vec<&'static str> {
        vec![
            "将牌: 二五八作为将牌",
            "胡牌必须有一对将牌",
            "将牌决定胡牌条件",
            "不同将牌番数不同",
            "二五八是长沙麻将核心",
        ]
    }

    /// 番型规则
    pub fn fan_types(&self) -> Vec<(&'static str, u8)> {
        vec![
            // 基本番型
            ("平胡", 1),
            ("对对胡", 2),
            ("混一色", 2),
            ("清一色", 4),
            ("七对子", 2),
            // 将牌番型
            ("二将", 2),
            ("五将", 3),
            ("八将", 4),
            // 高级番型
            ("清一色对对", 6),
            ("将一色", 8),
            ("十三幺", 10),
            ("天胡", 10),
            ("地胡", 8),
            ("杠开", 2),
        ]
    }

    /// 血战规则
    pub fn xuezhan_rules(&self) -> Vec<&'static str> {
        vec![
            "一人胡牌后继续打",
            "可多人胡牌",
            "胡牌者不再参与",
            "最后结算总分",
            "增加刺激性",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "底分 × 番数",
            "自摸三家付",
            "不能点炮",
            "将牌加番",
            "血战累计计分",
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "必须自摸",
            "必须将牌",
            "无将不能胡",
            "荒牌流局",
            "可以报听",
        ]
    }

    /// 吃碰规则
    pub fn chi_peng_rules(&self) -> Vec<&'static str> {
        vec![
            "可以吃上家牌",
            "可以碰任意家牌",
            "可以明杠暗杠",
            "吃碰后必须保持将牌",
            "听牌后不能再吃碰",
        ]
    }
}

impl Default for ChangshaMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ChangshaMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_changsha")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        let fan_list: String = self.fan_types()
            .iter()
            .map(|(name, fan)| format!("  • {}: {}番", name, fan))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "【长沙麻将规则】\n\n\
            基本设置:\n{}\n\n\
            将牌规则:\n{}\n\n\
            番型规则:\n{}\n\n\
            血战规则:\n{}\n",
            self.basic_settings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.jiang_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            fan_list,
            self.xuezhan_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_changsha_mahjong_rules() {
        let rules = ChangshaMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
        assert!(!rules.jiang_rules().is_empty());
    }
}