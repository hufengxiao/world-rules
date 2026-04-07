//! 广西麻将规则
//!
//! 广西麻将讲究"转转"玩法，特点鲜明

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 广西麻将规则
pub struct GuangxiMahjongRules {
    metadata: RuleMetadata,
}

impl GuangxiMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "广西麻将规则",
                "广西地区流行麻将规则"
            )
            .with_origin("广西")
            .with_tags(vec!["游戏".into(), "麻将".into(), "广西".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌",
            "每人起手13张",
            "可以吃碰杠",
            "讲究转转玩法",
            "计分方式独特",
        ]
    }

    /// 转转规则
    pub fn zhuanzhuan_rules(&self) -> Vec<&'static str> {
        vec![
            "转转: 换位玩法",
            "每局结束后换位",
            "增加公平性",
            "减少运气因素",
            "广西特色玩法",
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
            // 广西特色
            ("转转胡", 2),
            ("清对", 6),
            ("混对", 4),
            // 高级番型
            ("十三幺", 10),
            ("大三元", 8),
            ("大四喜", 8),
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
            "转转额外计分",
            "庄家翻倍",
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "可以吃碰杠",
            "可以点炮",
            "转转换位",
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

impl Default for GuangxiMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for GuangxiMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_guangxi")
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
            "【广西麻将规则】\n\n\
            基本设置:\n{}\n\n\
            转转规则:\n{}\n\n\
            番型规则:\n{}\n\n\
            计分规则:\n{}\n",
            self.basic_settings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.zhuanzhuan_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            fan_list,
            self.scoring_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guangxi_mahjong_rules() {
        let rules = GuangxiMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
    }
}