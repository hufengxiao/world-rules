//! 网球戴维斯杯规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

/// 戴维斯杯赛制阶段
#[derive(Debug, Clone, Copy)]
pub enum DavisCupStage {
    /// 世界组决赛
    WorldGroupFinal,
    /// 世界组半决赛
    WorldGroupSemifinal,
    /// 世界组首轮
    WorldGroupFirstRound,
    /// 区域组
    RegionalGroup,
}

simple_rule! {
    struct: TennisDavisCupRules,
    name: "网球戴维斯杯规则",
    desc: "戴维斯杯网球团体赛规则",
    origin: "ITF",
    tags: ["体育", "网球", "团体赛"]
}

impl TennisDavisCupRules {
    /// 赛事名称
    pub fn tournament_name(&self) -> &'static str {
        "戴维斯杯"
    }

    /// 对阵形式
    pub fn match_format(&self) -> Vec<&'static str> {
        vec![
            "每场对决包含5场比赛",
            "第1场: 单打",
            "第2场: 单打",
            "第3场: 双打",
            "第4场: 单打",
            "第5场: 单打",
            "先赢3场获胜",
        ]
    }

    /// 比赛赛制
    pub fn match_sets(&self) -> Vec<&'static str> {
        vec![
            "世界组: 五盘三胜制",
            "区域组: 三盘两胜制",
            "每盘6-6时抢七",
            "决胜盘无抢十",
        ]
    }

    /// 主场选择规则
    pub fn home_away_rules(&self) -> Vec<&'static str> {
        vec![
            "主场球队选择场地类型",
            "主场球队选择比赛地点",
            "客场球队适应场地",
            "轮换主场客场",
        ]
    }

    /// 世界组规则
    pub fn world_group_rules(&self) -> Vec<&'static str> {
        vec![
            "16支国家队参赛",
            "首轮淘汰赛",
            "胜者晋级下一轮",
            "败者参加附加赛",
            "决赛决出冠军",
        ]
    }

    /// 资格获取方式
    pub fn qualification_method(&self) -> Vec<&'static str> {
        vec![
            "世界组前一年成绩",
            "区域组晋级",
            "附加赛胜者晋级",
            "败者降级到区域组",
        ]
    }

    /// 队员规则
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队4-6名选手",
            "必须本国国籍",
            "赛前确定出场名单",
            "可临时更换替补",
        ]
    }

    /// 检查对决是否获胜
    pub fn check_match_win(&self, wins: u8) -> bool {
        wins >= 3
    }

    /// 获取阶段描述
    pub fn stage_description(&self, stage: DavisCupStage) -> &'static str {
        match stage {
            DavisCupStage::WorldGroupFinal => "世界组决赛",
            DavisCupStage::WorldGroupSemifinal => "世界组半决赛",
            DavisCupStage::WorldGroupFirstRound => "世界组首轮",
            DavisCupStage::RegionalGroup => "区域组比赛",
        }
    }
}

impl Rule for TennisDavisCupRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("tennis_davis_cup")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "网球戴维斯杯规则",
            &[
                ("对阵形式", &self.match_format()),
                ("比赛赛制", &self.match_sets()),
                ("主客场规则", &self.home_away_rules()),
                ("世界组规则", &self.world_group_rules()),
                ("资格获取", &self.qualification_method()),
                ("队员规则", &self.team_composition()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let rules = TennisDavisCupRules::new();
        assert_eq!(rules.metadata().name, "网球戴维斯杯规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_match_win() {
        let rules = TennisDavisCupRules::new();
        assert!(rules.check_match_win(3));
        assert!(rules.check_match_win(4));
        assert!(!rules.check_match_win(2));
    }

    #[test]
    fn test_stage_description() {
        let rules = TennisDavisCupRules::new();
        assert!(rules
            .stage_description(DavisCupStage::WorldGroupFinal)
            .contains("决赛"));
    }
}
