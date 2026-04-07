//! 台湾麻将规则
//!
//! 台湾麻将特点是16张起手，有台数计分系统

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 台湾麻将规则
pub struct TaiwanMahjongRules {
    metadata: RuleMetadata,
}

impl TaiwanMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "台湾麻将规则",
                "台湾16张麻将规则"
            )
            .with_origin("台湾")
            .with_tags(vec!["游戏".into(), "麻将".into(), "台湾".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用144张牌",
            "每人起手16张(不是13张)",
            "胡牌需要17张(5组+1对)",
            "4人对战",
            "庄家起手17张",
        ]
    }

    /// 台数系统
    pub fn tai_system(&self) -> Vec<(&'static str, u8)> {
        vec![
            // 一台
            ("门清", 1),
            ("自摸", 1),
            ("花牌", 1),
            ("圈风", 1),
            ("门风", 1),
            // 二台
            ("三元牌", 2),
            ("小三元", 2),
            ("混一色", 2),
            ("三暗刻", 2),
            // 三台
            ("清一色", 3),
            ("对对胡", 3),
            // 五台
            ("七对子", 5),
            ("清对", 5),
            // 八台
            ("大三元", 8),
            ("小四喜", 8),
            // 十台
            ("清七对", 10),
            ("大四喜", 10),
            // 十三台(满贯)
            ("天胡", 13),
            ("地胡", 13),
            ("十三幺", 13),
            ("字一色", 13),
        ]
    }

    /// 花牌规则
    pub fn flower_rules(&self) -> Vec<&'static str> {
        vec![
            "花牌: 春夏秋冬、梅兰竹菊",
            "每张花牌一台",
            "座位对应: 春梅-东、夏兰-南、秋竹-西、冬菊-北",
            "起手花牌亮出补牌",
            "花牌越多台数越高",
        ]
    }

    /// 台数计算
    pub fn tai_calculation(&self) -> Vec<&'static str> {
        vec![
            "台数 = 各台型累加",
            "底台: 基础台数",
            "满贯: 13台封顶",
            "自摸加一台",
            "门清加一台",
        ]
    }

    /// 连庄规则
    pub fn lianzhuang_rules(&self) -> Vec<&'static str> {
        vec![
            "庄家胡牌连庄",
            "流局时庄家听牌连庄",
            "闲家胡牌下家做庄",
            "最多连庄四局",
            "连庄次数影响台数",
        ]
    }

    /// 拉庄规则
    pub fn la_zhuang_rules(&self) -> Vec<&'static str> {
        vec![
            "拉庄: 闲家可选择拉庄",
            "拉庄后台数翻倍",
            "输赢金额也翻倍",
            "可多次拉庄",
            "拉庄增加刺激度",
        ]
    }

    /// 计分公式
    pub fn scoring_formula(&self) -> Vec<&'static str> {
        vec![
            "分数 = 底分 × 台数",
            "自摸: 三家付",
            "点炮: 一家付",
            "拉庄后翻倍",
            "满贯按13台计算",
        ]
    }

    /// 特殊规则
    pub fn special_rules(&self) -> Vec<&'static str> {
        vec![
            "可以吃碰杠",
            "可以听牌",
            "听牌后翻倍",
            "相公(牌数不对)判负",
            "荒牌流局",
        ]
    }
}

impl Default for TaiwanMahjongRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for TaiwanMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_taiwan")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        let tai_list: String = self.tai_system()
            .iter()
            .map(|(name, tai)| format!("  • {}: {}台", name, tai))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "【台湾麻将规则】\n\n\
            基本设置:\n{}\n\n\
            台数系统:\n{}\n\n\
            花牌规则:\n{}\n\n\
            计分公式:\n{}\n",
            self.basic_settings().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            tai_list,
            self.flower_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_formula().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_taiwan_mahjong_rules() {
        let rules = TaiwanMahjongRules::new();
        assert!(!rules.basic_settings().is_empty());
        assert!(!rules.tai_system().is_empty());
    }
}