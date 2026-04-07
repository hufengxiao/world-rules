//! 新疆麻将规则
//!
//! 新疆麻将特点鲜明，规则独特

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 新疆麻将规则
pub struct XinjiangMahjongRules {
    metadata: RuleMetadata,
}

impl XinjiangMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "新疆麻将规则",
                "新疆地区流行麻将规则"
            )
            .with_origin("新疆")
            .with_tags(vec!["游戏".into(), "麻将".into(), "新疆".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌",
            "每人起手13张",
            "可以吃碰杠",
            "讲究七对胡牌",
            "计分独特",
        ]
    }

    /// 七对规则
    pub fn qidui_rules(&self) -> Vec<&'static str> {
        vec![
            "七对: 七个对子胡牌",
            "是新疆麻将常见胡法",
            "番数较高",
            "容易组成",
            "新疆特色",
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
            ("七对子", 3),
            // 新疆特色
            ("豪华七对", 6),
            ("清七对", 6),
            ("龙七对", 8),
            // 高级番型
            ("清对", 6),
            ("混对", 4),
            ("十三幺", 10),
            ("天胡", 10),
            ("地胡", 8),
            ("杠开", 2),
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "底分 × 番数",
            "自摸三家付",
            "点炮一家付",
            "七对加番",
            "庄家翻倍",
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "可以吃碰杠",
            "可以点炮",
            "七对常见",
            "一家胡牌结束",
            "荒牌流局",
        ]
    }

    /// 吃碰规则
    pub fn chi_peng_rules(&self) -> Vec<&'static str> {
        vec![
            "可以吃上家牌",
            "可以碰任意家牌",
            "可以明杠暗杠",
            "吃碰影响番型",
            "听牌后不能再操作",
        ]
    }
}

impl Default for XinjiangMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for XinjiangMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_xinjiang")
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
            "【新疆麻将规则】\n\n\
            基本设置:\n{}\n\n\
            七对规则:\n{}\n\n\
            番型规则:\n{}\n\n\
            计分规则:\n{}\n",
            self.basic_settings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.qidui_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            fan_list,
            self.scoring_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xinjiang_mahjong_rules() {
        let rules = XinjiangMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
    }
}