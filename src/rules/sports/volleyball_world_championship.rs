//! 排球世锦赛规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: VolleyballWorldChampionshipRules,
    name: "排球世锦赛规则",
    desc: "FIVB排球世界锦标赛规则",
    origin: "FIVB",
    tags: ["体育", "排球", "世锦赛"]
}

impl VolleyballWorldChampionshipRules {
    /// 参赛队伍数量
    pub fn team_count(&self) -> u8 {
        24 // 24支队伍参赛
    }

    /// 小组数量
    pub fn group_count(&self) -> u8 {
        4 // 4个小组，每组6队
    }

    /// 小组晋级规则
    pub fn group_qualification(&self) -> Vec<&'static str> {
        vec![
            "每组前4名晋级淘汰赛",
            "小组赛采用循环赛制",
            "小组赛排名依据积分、胜场、净胜局",
            "第5-6名淘汰",
        ]
    }

    /// 比赛赛制
    pub fn match_format(&self) -> Vec<&'static str> {
        vec![
            "小组赛: 5局3胜制",
            "淘汰赛: 5局3胜制",
            "决赛: 5局3胜制",
            "前4局25分，决胜局15分",
            "必须领先2分获胜",
        ]
    }

    /// 淘汰赛结构
    pub fn knockout_structure(&self) -> Vec<&'static str> {
        vec!["16强淘汰赛", "8强淘汰赛", "半决赛", "决赛", "三四名决赛"]
    }

    /// 赛事周期
    pub fn tournament_cycle(&self) -> Vec<&'static str> {
        vec![
            "每4年举办一次",
            "与奥运会错开2年",
            "男女分开举办",
            "主办国自动参赛",
        ]
    }

    /// 资格获取方式
    pub fn qualification_method(&self) -> Vec<&'static str> {
        vec![
            "洲际资格赛",
            "世界排名",
            "主办国直接晋级",
            "卫冕冠军自动晋级",
        ]
    }

    /// 检查小组是否晋级
    pub fn check_group_qualification(&self, group_position: u8) -> bool {
        (1..=4).contains(&group_position)
    }
}

impl Rule for VolleyballWorldChampionshipRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("volleyball_world_championship")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        format!(
            "【排球世锦赛规则】\n\n\
            参赛队伍: {} 支\n\
            小组数量: {} 个\n\
            每组队伍: {} 支\n\n\
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
            淘汰赛结构:\n\
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
            6,
            self.group_qualification()[0],
            self.group_qualification()[1],
            self.group_qualification()[2],
            self.group_qualification()[3],
            self.match_format()[0],
            self.match_format()[1],
            self.match_format()[2],
            self.match_format()[3],
            self.match_format()[4],
            self.knockout_structure()[0],
            self.knockout_structure()[1],
            self.knockout_structure()[2],
            self.knockout_structure()[3],
            self.knockout_structure()[4],
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
        let rules = VolleyballWorldChampionshipRules::new();
        assert_eq!(rules.metadata().name, "排球世锦赛规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_team_count() {
        let rules = VolleyballWorldChampionshipRules::new();
        assert_eq!(rules.team_count(), 24);
        assert_eq!(rules.group_count(), 4);
    }

    #[test]
    fn test_qualification() {
        let rules = VolleyballWorldChampionshipRules::new();
        assert!(rules.check_group_qualification(1));
        assert!(rules.check_group_qualification(4));
        assert!(!rules.check_group_qualification(5));
    }
}
