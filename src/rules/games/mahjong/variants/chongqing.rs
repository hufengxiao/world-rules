//! 重庆麻将规则
//!
//! 重庆麻将与四川麻将类似，但有独特特点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 重庆麻将规则
pub struct ChongqingMahjongRules {
    metadata: RuleMetadata,
}

impl ChongqingMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "重庆麻将规则",
                "重庆地区流行麻将规则"
            )
            .with_origin("重庆")
            .with_tags(vec!["游戏".into(), "麻将".into(), "重庆".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用108张牌(只有万条筒)",
            "每人起手13张",
            "必须缺一门才能胡",
            "只能自摸胡牌",
            "血战到底规则",
        ]
    }

    /// 缺门规则
    pub fn quemen_rules(&self) -> Vec<&'static str> {
        vec![
            "必须缺一门: 只保留两种花色",
            "开局前选择缺哪一门",
            "缺万、缺条、缺筒三选一",
            "不能有缺门的牌",
            "缺门是重庆麻将核心",
        ]
    }

    /// 血战规则
    pub fn xuezhan_rules(&self) -> Vec<&'static str> {
        vec![
            "血战到底: 一人胡牌后继续打",
            "可多人胡牌",
            "已胡者退出游戏",
            "最后结算总分",
            "增加刺激性",
        ]
    }

    /// 番型规则
    pub fn fan_types(&self) -> Vec<(&'static str, u8)> {
        vec![
            // 基本番型
            ("平胡", 1),
            ("对对胡", 2),
            ("清一色", 4),
            // 重庆特色
            ("金钩钓", 2),
            ("清对", 4),
            ("将对", 4),
            ("杠上花", 2),
            // 高级番型
            ("七对", 2),
            ("清七对", 4),
            ("龙七对", 8),
            ("天胡", 10),
            ("地胡", 8),
        ]
    }

    /// 金钩钓
    pub fn jingoudiao_rules(&self) -> Vec<&'static str> {
        vec![
            "金钩钓: 单吊一张牌胡",
            "必须是自摸",
            "番数加成",
            "是重庆特色番型",
            "增加技巧性",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "底分 × 番数",
            "自摸三家付",
            "不能点炮",
            "血战累计计分",
            "清一色番数最高",
        ]
    }

    /// 杠牌规则
    pub fn kong_rules(&self) -> Vec<&'static str> {
        vec![
            "明杠: 暴露杠牌",
            "暗杠: 隐藏杠牌",
            "杠后继续摸打",
            "杠上花: 杠后自摸胡",
            "杠牌计分另算",
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "只有万条筒三种花色",
            "必须缺一门",
            "只能自摸",
            "血战到底",
            "无字牌无花牌",
        ]
    }
}

impl Default for ChongqingMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ChongqingMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_chongqing")
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
            "【重庆麻将规则】\n\n\
            基本设置:\n{}\n\n\
            缺门规则:\n{}\n\n\
            番型规则:\n{}\n\n\
            血战规则:\n{}\n",
            self.basic_settings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.quemen_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            fan_list,
            self.xuezhan_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chongqing_mahjong_rules() {
        let rules = ChongqingMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
        assert!(!rules.quemen_rules().is_empty());
    }
}