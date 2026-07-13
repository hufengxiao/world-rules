//! IMP桥牌规则 (IMP Bridge)
//!
//! IMP桥牌使用国际比赛分(International Match Points)计分制。
//! 将原始分差换算为IMP分，用于队式比赛和在线桥牌平台。
//!
//! # Examples
//!
//! ```rust
//! use world_rules::rules::games::bridge_imp::BridgeImpRules;
//! use world_rules::rules::core::Rule;
//!
//! let rules = BridgeImpRules::new();
//! assert_eq!(rules.metadata().name, "IMP桥牌规则");
//! assert!(!rules.explain().is_empty());
//! ```

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: BridgeImpRules,
    name: "IMP桥牌规则",
    desc: "IMP桥牌（IMP Bridge）国际比赛分计分规则",
    origin: "国际",
    tags: ["游戏", "卡牌", "桥牌", "竞技"],
}

impl BridgeImpRules {
    /// 概述
    pub fn overview(&self) -> Vec<&'static str> {
        vec![
            "IMP(International Match Points)是国际比赛分制",
            "将原始分差换算为标准化的IMP分",
            "用于队式比赛和在线桥牌平台",
            "平滑运气波动，强调技术水平",
        ]
    }

    /// IMP换算表
    pub fn imp_conversion_table(&self) -> Vec<&'static str> {
        vec![
            "0-10分 → 0 IMP",
            "20-40分 → 1 IMP",
            "50-80分 → 2 IMP",
            "90-120分 → 3 IMP",
            "130-160分 → 4 IMP",
            "170-210分 → 5 IMP",
            "220-260分 → 6 IMP",
            "270-310分 → 7 IMP",
            "320-360分 → 8 IMP",
            "370-420分 → 9 IMP",
            "430-490分 → 10 IMP",
            "500-590分 → 11 IMP",
            "600-740分 → 12 IMP",
            "750-890分 → 13 IMP",
            "900-1090分 → 14 IMP",
            "1100-1290分 → 15 IMP",
            "1300-1490分 → 16 IMP",
            "1500-1740分 → 17 IMP",
            "1750-1990分 → 18 IMP",
            "2000-2240分 → 19 IMP",
            "2250-2490分 → 20 IMP",
            "2500-2990分 → 21 IMP",
            "3000-3490分 → 22 IMP",
            "3500-3990分 → 23 IMP",
            "4000+分 → 24 IMP",
        ]
    }

    /// 比赛形式
    pub fn tournament_formats(&self) -> Vec<&'static str> {
        vec![
            "队式赛: 开闭室成绩对比计算IMP",
            "瑞士队式赛: 多队轮转，IMP累计",
            "KO淘汰赛: IMP累计决胜负",
            "在线桥牌: BBO、桥友圈等平台",
        ]
    }

    /// 策略特点
    pub fn strategy_characteristics(&self) -> Vec<&'static str> {
        vec![
            "激进叫牌可能带来大赢或大输",
            "安全打法比比赛分制更重要",
            "部分定约价值提高",
            "满贯叫牌需要精确判断",
        ]
    }

    /// VP换算
    pub fn vp_conversion(&self) -> Vec<&'static str> {
        vec![
            "VP(Victory Points)用于队式赛排名",
            "IMP差值换算为VP(通常20分制)",
            "0 IMP差 → 10-10 VP",
            "大比分差距有VP封顶",
            "VP用于排名而非直接胜负",
        ]
    }
}

impl Rule for BridgeImpRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("bridge_imp")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "IMP桥牌规则 (IMP Bridge)",
            &[
                ("概述", &self.overview()),
                ("IMP换算表", &self.imp_conversion_table()),
                ("比赛形式", &self.tournament_formats()),
                ("策略特点", &self.strategy_characteristics()),
                ("VP换算", &self.vp_conversion()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge_imp_rules() {
        let rules = BridgeImpRules::new();
        assert_eq!(rules.metadata().name, "IMP桥牌规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.imp_conversion_table().is_empty());
    }
}
