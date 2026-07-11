//! 网球ATP总决赛规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: TennisAtpFinalsRules,
    name: "网球ATP总决赛规则",
    desc: "ATP年终总决赛规则",
    origin: "ATP",
    tags: ["体育", "网球", "ATP"]
}

impl TennisAtpFinalsRules {
    /// 参赛选手数量
    pub fn player_count(&self) -> u8 {
        8 // 8名单打选手
    }

    /// 赛事名称
    pub fn tournament_name(&self) -> &'static str {
        "ATP年终总决赛"
    }

    /// 小组赛规则
    pub fn group_stage_rules(&self) -> Vec<&'static str> {
        vec![
            "8名选手分成2组，每组4人",
            "小组赛采用循环赛制",
            "每组前2名晋级半决赛",
            "小组第3-4名淘汰",
        ]
    }

    /// 比赛赛制
    pub fn match_format(&self) -> Vec<&'static str> {
        vec![
            "小组赛: 三盘两胜制",
            "半决赛: 三盘两胜制",
            "决赛: 三盘两胜制",
            "每盘6-6时抢七",
            "无决胜盘抢十",
        ]
    }

    /// 淘汰赛规则
    pub fn knockout_rules(&self) -> Vec<&'static str> {
        vec![
            "半决赛: A组第1 vs B组第2",
            "半决赛: B组第1 vs A组第2",
            "决赛: 半决赛胜者对决",
            "无三四名决赛",
        ]
    }

    /// 资格获取方式
    pub fn qualification_method(&self) -> Vec<&'static str> {
        vec![
            "ATP年度积分排名前8",
            "必须参加至少8项赛事",
            "大满贯和大师赛积分优先",
            "替补选手: 排名第9-10位",
        ]
    }

    /// 积分分配
    pub fn points_allocation(&self) -> Vec<&'static str> {
        vec![
            "小组赛每胜: 200分",
            "半决赛胜: 400分",
            "决赛胜: 500分",
            "全胜夺冠: 1500分",
            "替补参赛: 0分",
        ]
    }

    /// 奖金分配
    pub fn prize_money(&self) -> Vec<&'static str> {
        vec![
            "小组赛出场费",
            "小组赛每胜奖金",
            "半决赛奖金",
            "决赛奖金",
            "冠军总奖金最高",
        ]
    }

    /// 检查是否晋级半决赛
    pub fn check_semifinal_qualification(&self, group_position: u8) -> bool {
        (1..=2).contains(&group_position)
    }

    /// 计算小组赛积分
    pub fn calculate_group_points(&self, wins: u8) -> u16 {
        wins as u16 * 200
    }
}

impl Rule for TennisAtpFinalsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("tennis_atp_finals")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        format!(
            "【网球ATP总决赛规则】\n\n\
            参赛选手: {} 名\n\n\
            小组赛规则:\n\
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
            - {}\n\n\
            资格获取:\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}\n\n\
            积分分配:\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}",
            self.player_count(),
            self.group_stage_rules()[0],
            self.group_stage_rules()[1],
            self.group_stage_rules()[2],
            self.group_stage_rules()[3],
            self.match_format()[0],
            self.match_format()[1],
            self.match_format()[2],
            self.match_format()[3],
            self.match_format()[4],
            self.knockout_rules()[0],
            self.knockout_rules()[1],
            self.knockout_rules()[2],
            self.knockout_rules()[3],
            self.qualification_method()[0],
            self.qualification_method()[1],
            self.qualification_method()[2],
            self.qualification_method()[3],
            self.points_allocation()[0],
            self.points_allocation()[1],
            self.points_allocation()[2],
            self.points_allocation()[3],
            self.points_allocation()[4]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let rules = TennisAtpFinalsRules::new();
        assert_eq!(rules.metadata().name, "网球ATP总决赛规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_player_count() {
        let rules = TennisAtpFinalsRules::new();
        assert_eq!(rules.player_count(), 8);
    }

    #[test]
    fn test_qualification() {
        let rules = TennisAtpFinalsRules::new();
        assert!(rules.check_semifinal_qualification(1));
        assert!(rules.check_semifinal_qualification(2));
        assert!(!rules.check_semifinal_qualification(3));
    }

    #[test]
    fn test_points_calculation() {
        let rules = TennisAtpFinalsRules::new();
        assert_eq!(rules.calculate_group_points(0), 0);
        assert_eq!(rules.calculate_group_points(1), 200);
        assert_eq!(rules.calculate_group_points(3), 600);
    }
}
