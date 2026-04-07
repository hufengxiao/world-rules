//! 南京麻将规则
//!
//! 南京麻将特点是"成牌"制度，计分方式独特

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 南京麻将规则
pub struct NanjingMahjongRules {
    metadata: RuleMetadata,
}

impl NanjingMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "南京麻将规则",
                "南京地区流行麻将规则"
            )
            .with_origin("南京")
            .with_tags(vec!["游戏".into(), "麻将".into(), "南京".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌(无花牌)",
            "每人起手13张",
            "可以吃碰杠",
            "讲究成牌制度",
            "只能自摸胡牌",
        ]
    }

    /// 成牌规则
    pub fn chengpai_rules(&self) -> Vec<&'static str> {
        vec![
            "成牌: 达到特定条件才能胡",
            "必须有番型才能成牌",
            "无番型不能胡牌",
            "成牌条件限制较严",
            "成牌增加策略性",
        ]
    }

    /// 番型规则
    pub fn fan_types(&self) -> Vec<(&'static str, u8)> {
        vec![
            // 基本成牌番型
            ("平胡", 1),
            ("对对胡", 2),
            ("混一色", 2),
            ("清一色", 4),
            ("七对子", 2),
            // 特殊番型
            ("成牌", 1),
            ("大车", 6),
            ("小车", 4),
            // 高级番型
            ("清对", 6),
            ("混对", 4),
            ("十三幺", 10),
            ("天胡", 10),
            ("地胡", 8),
            ("杠开", 2),
        ]
    }

    /// 大车小车
    pub fn dache_xiaoche_rules(&self) -> Vec<&'static str> {
        vec![
            "大车: 高番数胡牌",
            "小车: 低番数胡牌",
            "大车番数要求更高",
            "大车赔付更高",
            "小车相对容易胡",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "底分 × 番数",
            "自摸三家付",
            "不能点炮",
            "成牌加番",
            "大车翻倍",
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "必须自摸",
            "必须成牌",
            "无番不能胡",
            "荒牌流局",
            "讲究策略",
        ]
    }

    /// 吃碰规则
    pub fn chi_peng_rules(&self) -> Vec<&'static str> {
        vec![
            "可以吃上家牌",
            "可以碰任意家牌",
            "可以明杠暗杠",
            "吃碰影响成牌条件",
            "听牌后不能再吃碰",
        ]
    }
}

impl Default for NanjingMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NanjingMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_nanjing")
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
            "【南京麻将规则】\n\n\
            基本设置:\n{}\n\n\
            成牌规则:\n{}\n\n\
            番型规则:\n{}\n\n\
            计分规则:\n{}\n",
            self.basic_settings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.chengpai_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            fan_list,
            self.scoring_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nanjing_mahjong_rules() {
        let rules = NanjingMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
        assert!(!rules.chengpai_rules().is_empty());
    }
}