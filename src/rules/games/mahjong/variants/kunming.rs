//! 昆明麻将规则
//!
//! 昆明麻将是云南地区的代表玩法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 昆明麻将规则
pub struct KunmingMahjongRules {
    metadata: RuleMetadata,
}

impl KunmingMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "昆明麻将规则",
                "昆明地区流行麻将规则"
            )
            .with_origin("昆明")
            .with_tags(vec!["游戏".into(), "麻将".into(), "昆明".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌",
            "每人起手13张",
            "可以吃碰杠",
            "有特殊番型",
            "计分方式独特",
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
            // 云南特色
            ("夹心五", 3),
            ("飘胡", 2),
            ("大对子", 2),
            // 高级番型
            ("清对", 6),
            ("混对", 4),
            ("十三幺", 10),
            ("天胡", 10),
            ("地胡", 8),
            ("杠开", 2),
        ]
    }

    /// 夹心五
    pub fn jiaxinwu_rules(&self) -> Vec<&'static str> {
        vec![
            "夹心五: 5万/5条/5筒被夹在中间",
            "如3-4-5或5-6-7",
            "夹心五是云南特色",
            "番数加成",
            "增加趣味性",
        ]
    }

    /// 飘胡规则
    pub fn piaohu_rules(&self) -> Vec<&'static str> {
        vec![
            "飘胡: 单张胡牌",
            "可以是自摸或点炮",
            "飘胡番数较低",
            "容易胡牌",
            "增加胡牌机会",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "底分 × 番数",
            "自摸三家付",
            "点炮一家付",
            "夹心五加番",
            "庄家翻倍",
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "可以吃碰杠",
            "可以点炮",
            "一家胡牌结束",
            "荒牌流局",
            "连庄规则",
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

impl Default for KunmingMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for KunmingMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_kunming")
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
            "【昆明麻将规则】\n\n\
            基本设置:\n{}\n\n\
            番型规则:\n{}\n\n\
            夹心五规则:\n{}\n\n\
            计分规则:\n{}\n",
            self.basic_settings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            fan_list,
            self.jiaxinwu_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kunming_mahjong_rules() {
        let rules = KunmingMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
        assert!(!rules.fan_types().is_empty());
    }
}