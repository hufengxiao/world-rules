//! 手球欧冠联赛规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

/// EHF欧冠阶段
#[derive(Debug, Clone, Copy)]
pub enum HandballChampionsLeagueStage {
    /// 小组赛
    GroupStage,
    /// 淘汰赛
    KnockoutRound,
    /// 四分之一决赛
    QuarterFinal,
    /// 半决赛
    SemiFinal,
    /// 决赛
    Final,
}

simple_rule! {
    struct: HandballEhfChampionsLeagueRules,
    name: "手球欧冠联赛规则",
    desc: "EHF手球欧洲冠军联赛规则",
    origin: "EHF",
    tags: ["体育", "手球", "欧冠"]
}

impl HandballEhfChampionsLeagueRules {
    /// 参赛队伍数量
    pub fn team_count(&self) -> u8 {
        16 // 16支队伍
    }

    /// 小组数量
    pub fn group_count(&self) -> u8 {
        2 // 2个小组
    }

    /// 每组队伍数量
    pub fn teams_per_group(&self) -> u8 {
        8
    }

    /// 小组赛规则
    pub fn group_stage_rules(&self) -> Vec<&'static str> {
        vec![
            "每组8队循环赛",
            "每组前6名晋级",
            "每组第7-8名淘汰",
            "积分决定排名",
            "胜2分，平1分，负0分",
        ]
    }

    /// 比赛赛制
    pub fn match_format(&self) -> Vec<&'static str> {
        vec![
            "正规时间60分钟",
            "上下半场各30分钟",
            "平局后延长赛",
            "延长赛上下半场各5分钟",
            "仍平局后7米罚球决胜",
        ]
    }

    /// 淘汰赛规则
    pub fn knockout_rules(&self) -> Vec<&'static str> {
        vec![
            "16强淘汰赛",
            "主客场两回合制",
            "总比分胜者晋级",
            "四分之一决赛",
            "半决赛主客场",
            "决赛单场决胜",
        ]
    }

    /// 决赛规则
    pub fn final_rules(&self) -> Vec<&'static str> {
        vec![
            "决赛在中立场地",
            "单场决胜制",
            "平局后延长赛",
            "延长赛平局后罚球",
            "无三四名决赛",
        ]
    }

    /// 资格获取方式
    pub fn qualification_method(&self) -> Vec<&'static str> {
        vec!["各国联赛冠军", "各国联赛亚军", "EHF排名", "外卡邀请"]
    }

    /// 获取阶段描述
    pub fn stage_description(&self, stage: HandballChampionsLeagueStage) -> &'static str {
        match stage {
            HandballChampionsLeagueStage::GroupStage => "小组赛",
            HandballChampionsLeagueStage::KnockoutRound => "淘汰赛",
            HandballChampionsLeagueStage::QuarterFinal => "四分之一决赛",
            HandballChampionsLeagueStage::SemiFinal => "半决赛",
            HandballChampionsLeagueStage::Final => "决赛",
        }
    }

    /// 检查小组是否晋级
    pub fn check_group_qualification(&self, group_position: u8) -> bool {
        (1..=6).contains(&group_position)
    }

    /// 计算积分
    pub fn calculate_points(&self, wins: u8, draws: u8) -> u16 {
        wins as u16 * 2 + draws as u16
    }
}

impl Rule for HandballEhfChampionsLeagueRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("handball_ehf_champions_league")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        format!(
            "【手球欧冠联赛规则】\n\n\
            参赛队伍: {} 支\n\
            小组数量: {} 个\n\
            每组队伍: {} 支\n\n\
            小组赛规则:\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}\n\n\
            比赛赛制:\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}\n\n\
            淘汰赛规则:\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}\n\n\
            决赛规则:\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}\n\n\
            资格获取:\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}",
            self.team_count(),
            self.group_count(),
            self.teams_per_group(),
            self.group_stage_rules()[0],
            self.group_stage_rules()[1],
            self.group_stage_rules()[2],
            self.group_stage_rules()[3],
            self.group_stage_rules()[4],
            self.match_format()[0],
            self.match_format()[1],
            self.match_format()[2],
            self.match_format()[3],
            self.match_format()[4],
            self.knockout_rules()[0],
            self.knockout_rules()[1],
            self.knockout_rules()[2],
            self.knockout_rules()[3],
            self.knockout_rules()[4],
            self.knockout_rules()[5],
            self.final_rules()[0],
            self.final_rules()[1],
            self.final_rules()[2],
            self.final_rules()[3],
            self.final_rules()[4],
            self.qualification_method()[0],
            self.qualification_method()[1],
            self.qualification_method()[2],
            self.qualification_method()[3]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let rules = HandballEhfChampionsLeagueRules::new();
        assert_eq!(rules.metadata().name, "手球欧冠联赛规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_team_count() {
        let rules = HandballEhfChampionsLeagueRules::new();
        assert_eq!(rules.team_count(), 16);
        assert_eq!(rules.teams_per_group(), 8);
    }

    #[test]
    fn test_group_qualification() {
        let rules = HandballEhfChampionsLeagueRules::new();
        assert!(rules.check_group_qualification(1));
        assert!(rules.check_group_qualification(6));
        assert!(!rules.check_group_qualification(7));
    }

    #[test]
    fn test_points_calculation() {
        let rules = HandballEhfChampionsLeagueRules::new();
        assert_eq!(rules.calculate_points(5, 2), 12);
        assert_eq!(rules.calculate_points(10, 0), 20);
    }
}
