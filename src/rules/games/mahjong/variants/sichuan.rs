//! 四川麻将规则(血战到底) - 详细版
//!
//! 四川麻将是西南地区最流行的玩法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 四川麻将规则(详细版)
pub struct SichuanDetailedMahjongRules {
    metadata: RuleMetadata,
}

impl SichuanDetailedMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "四川麻将规则",
                "四川血战到底详细规则"
            )
            .with_origin("四川")
            .with_tags(vec!["游戏".into(), "麻将".into(), "四川".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用108张牌(只有万条筒)",
            "每人起手13张",
            "必须缺一门才能胡",
            "只能自摸胡牌",
            "血战到底: 胡牌后继续打",
        ]
    }

    /// 缺一门规则
    pub fn quemen_rules(&self) -> Vec<&'static str> {
        vec![
            "开局必须选择缺哪一门",
            "缺万、缺条、缺筒三选一",
            "选择后不能更改",
            "胡牌时不能有缺门的牌",
            "缺一门是四川麻将核心规则",
            "增加策略性和公平性",
        ]
    }

    /// 血战到底规则
    pub fn xuezhan_rules(&self) -> Vec<&'static str> {
        vec![
            "一人胡牌后游戏继续",
            "已胡者不再参与",
            "其他人继续打牌",
            "最多三家胡牌",
            "最后结算总分",
            "血战到底增加刺激性",
        ]
    }

    /// 番型规则
    pub fn fan_types(&self) -> Vec<(&'static str, u8)> {
        vec![
            // 基本番型
            ("平胡", 1),
            ("对对胡", 2),
            ("清一色", 4),
            // 四川特色
            ("金钩钓", 2),
            ("清对", 4),
            ("将对", 4),
            ("杠上花", 2),
            ("海底捞", 2),
            // 高级番型
            ("七对", 2),
            ("清七对", 4),
            ("龙七对", 8),
            ("双龙七对", 16),
            ("天胡", 10),
            ("地胡", 8),
        ]
    }

    /// 金钩钓规则
    pub fn jingoudiao_rules(&self) -> Vec<&'static str> {
        vec![
            "金钩钓: 单张胡牌",
            "必须是自摸",
            "最后只剩一张牌",
            "番数加成",
            "四川特色番型",
        ]
    }

    /// 查大叫规则
    pub fn chadajiao_rules(&self) -> Vec<&'static str> {
        vec![
            "查大叫: 流局时检查听牌",
            "未听牌者赔听牌者",
            "听牌者不互赔",
            "增加防守策略",
            "是四川麻将重要规则",
        ]
    }

    /// 查花猪规则
    pub fn chahuazhu_rules(&self) -> Vec<&'static str> {
        vec![
            "花猪: 手中有三种花色",
            "流局时查花猪",
            "花猪者赔偿其他三家",
            "惩罚不缺门者",
            "保证规则执行",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "底分 × 番数",
            "自摸三家付",
            "不能点炮",
            "血战累计计分",
            "查大叫另算",
            "查花猪另算",
        ]
    }

    /// 杠牌规则
    pub fn kong_rules(&self) -> Vec<&'static str> {
        vec![
            "明杠: 暴露杠牌",
            "暗杠: 隐藏杠牌",
            "杠后摸打",
            "杠上花: 杠后自摸胡",
            "杠牌影响番数",
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "只有万条筒三种花色",
            "无字牌无花牌",
            "必须缺一门",
            "只能自摸",
            "血战到底",
            "查大叫查花猪",
        ]
    }
}

impl Default for SichuanDetailedMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SichuanDetailedMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_sichuan")
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
            "【四川麻将规则】\n\n\
            基本设置:\n{}\n\n\
            缺一门规则:\n{}\n\n\
            血战到底:\n{}\n\n\
            番型规则:\n{}\n\n\
            计分规则:\n{}\n",
            self.basic_settings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.quemen_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.xuezhan_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            fan_list,
            self.scoring_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sichuan_detailed_mahjong_rules() {
        let rules = SichuanDetailedMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
        assert!(!rules.quemen_rules().is_empty());
        assert!(!rules.xuezhan_rules().is_empty());
    }
}