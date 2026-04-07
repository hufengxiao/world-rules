//! 武汉麻将规则
//!
//! 武汉麻将又称"红中赖子杠"，是湖北地区最流行的麻将玩法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 武汉麻将规则
pub struct WuhanMahjongRules {
    metadata: RuleMetadata,
}

impl WuhanMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "武汉麻将规则",
                "武汉红中赖子杠麻将规则"
            )
            .with_origin("武汉")
            .with_tags(vec!["游戏".into(), "麻将".into(), "武汉".into()]),
        }
    }

    /// 牌组构成
    pub fn tile_composition(&self) -> Vec<&'static str> {
        vec![
            "使用112张牌",
            "万、条、筒各36张",
            "红中4张作为赖子",
            "无其他字牌",
            "无花牌",
        ]
    }

    /// 赖子规则
    pub fn laizi_rules(&self) -> Vec<&'static str> {
        vec![
            "红中为赖子(百搭牌)",
            "赖子可以代替任何牌",
            "赖子不能打出去",
            "手中无赖子时可杠红中",
            "赖子只能用于胡牌",
            "多个赖子组合有限制",
        ]
    }

    /// 杠牌规则
    pub fn kong_rules(&self) -> Vec<&'static str> {
        vec![
            "明杠: 他人打出的牌杠",
            "暗杠: 自己摸到的牌杠",
            "补杠: 已碰牌后摸到第四张",
            "赖子杠: 四张红中杠",
            "杠后必须摸牌打牌",
            "杠牌计分翻倍",
        ]
    }

    /// 胡牌规则
    pub fn winning_rules(&self) -> Vec<&'static str> {
        vec![
            "必须自摸胡牌",
            "不能点炮",
            "胡牌后继续打(血战)",
            "赖子必须用于胡牌",
            "胡牌型必须完整",
        ]
    }

    /// 番型规则
    pub fn fan_types(&self) -> Vec<(&'static str, u8)> {
        vec![
            ("屁胡", 1),
            ("硬胡", 2),
            ("软胡", 1),
            ("清一色", 4),
            ("对对胡", 3),
            ("七对", 3),
            ("豪华七对", 6),
            ("清七对", 6),
            ("清豪华七对", 12),
            // 赖子相关
            ("赖子胡", 1),
            ("无赖子", 2),
        ]
    }

    /// 特殊胡牌
    pub fn special_wins(&self) -> Vec<&'static str> {
        vec![
            "天胡: 庄家起手14张直接胡",
            "地胡: 闲家第一轮自摸胡",
            "杠开: 杠后自摸胡",
            "抢杠: 抢他人补杠胡(武汉规则较少用)",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "基础分: 每番1分起",
            "自摸: 三家付",
            "杠牌: 另计杠分",
            "番数相乘",
            "有赖子时番数减半",
            "无赖子时番数翻倍",
        ]
    }

    /// 血战规则
    pub fn xuezhan_rules(&self) -> Vec<&'static str> {
        vec![
            "一人胡牌后游戏继续",
            "已胡者不再参与",
            "最多三家胡牌",
            "最后一人未胡则流局",
            "胡牌者坐下一局庄",
        ]
    }

    /// 禁忌规则
    pub fn forbidden_rules(&self) -> Vec<&'static str> {
        vec![
            "不能打赖子",
            "不能吃牌",
            "不能点炮",
            "必须自摸",
            "胡牌后不能继续操作",
        ]
    }
}

impl Default for WuhanMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WuhanMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_wuhan")
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
            "【武汉麻将规则】\n\n\
            牌组构成:\n{}\n\n\
            赖子规则:\n{}\n\n\
            番型规则:\n{}\n\n\
            血战规则:\n{}\n",
            self.tile_composition().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.laizi_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            fan_list,
            self.xuezhan_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wuhan_mahjong_rules() {
        let rules = WuhanMahjongRules::new();
        assert!(!rules.tile_composition().is_empty());
        assert!(!rules.laizi_rules().is_empty());
    }
}