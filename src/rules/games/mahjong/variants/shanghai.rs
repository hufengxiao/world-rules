//! 上海麻将规则
//!
//! 上海麻将特点是有花牌，计分复杂，讲究"门清"

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 上海麻将规则
pub struct ShanghaiMahjongRules {
    metadata: RuleMetadata,
}

impl ShanghaiMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "上海麻将规则",
                "上海地区流行麻将规则"
            )
            .with_origin("上海")
            .with_tags(vec!["游戏".into(), "麻将".into(), "上海".into()]),
        }
    }

    /// 牌组构成
    pub fn tile_composition(&self) -> Vec<&'static str> {
        vec![
            "使用144张牌",
            "万、条、筒各36张",
            "风牌16张(东南西北)",
            "箭牌12张(中发白)",
            "花牌8张(春夏秋冬、梅兰竹菊)",
            "每人起手13张",
        ]
    }

    /// 花牌规则
    pub fn flower_rules(&self) -> Vec<&'static str> {
        vec![
            "春夏秋冬为花牌",
            "梅兰竹菊为花牌",
            "起手摸到花牌亮出补牌",
            "花牌对应座位: 春-东、夏-南、秋-西、冬-北",
            "花牌计番",
            "花牌越多番数越高",
        ]
    }

    /// 门清规则
    pub fn menqing_rules(&self) -> Vec<&'static str> {
        vec![
            "门清: 未吃碰杠牌",
            "门清胡牌番数翻倍",
            "门前清可宣布听牌",
            "听牌后不能换牌",
            "听牌后自摸番数更高",
        ]
    }

    /// 番型规则
    pub fn fan_types(&self) -> Vec<(&'static str, u8)> {
        vec![
            // 基本番型
            ("门清", 1),
            ("自摸", 1),
            ("花牌", 1),
            // 花色番型
            ("混一色", 2),
            ("清一色", 4),
            ("对对胡", 2),
            ("七对子", 2),
            // 高级番型
            ("清对", 6),
            ("混对", 4),
            ("清七对", 8),
            ("混七对", 4),
            ("十三幺", 10),
            // 特殊番型
            ("天胡", 20),
            ("地胡", 15),
            ("杠开", 2),
            ("抢杠", 2),
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "勒子: 计分封顶",
            "相公: 牌数不对判负",
            "荒牌: 流局",
            "连庄: 庄家胡牌继续坐庄",
            "承包: 碰杠某人牌后需负责",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "底分: 基础分数",
            "番数: 各番型相加",
            "门清翻倍",
            "自摸三家付",
            "点炮一家付",
            "封顶: 勒子限制",
        ]
    }

    /// 承包规则
    pub fn chengbao_rules(&self) -> Vec<&'static str> {
        vec![
            "碰某人牌三次以上",
            "被碰者需承担胡牌分",
            "明杠同理",
            "承包者付全部分数",
            "其他两家不付",
        ]
    }

    /// 吃碰规则
    pub fn chi_peng_rules(&self) -> Vec<&'static str> {
        vec![
            "可以吃上家牌",
            "可以碰任意家牌",
            "可以明杠",
            "吃碰后不再门清",
            "听牌后不能再吃碰",
        ]
    }
}

impl Default for ShanghaiMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ShanghaiMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_shanghai")
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
            "【上海麻将规则】\n\n\
            牌组构成:\n{}\n\n\
            门清规则:\n{}\n\n\
            番型规则:\n{}\n\n\
            计分规则:\n{}\n",
            self.tile_composition().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.menqing_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            fan_list,
            self.scoring_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shanghai_mahjong_rules() {
        let rules = ShanghaiMahjongRules::new();
        assert!(!rules.tile_composition().is_empty());
        assert!(!rules.flower_rules().is_empty());
    }
}