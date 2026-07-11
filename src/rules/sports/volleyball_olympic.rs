//! 排球奥运会规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: VolleyballOlympicRules,
    name: "排球奥运会规则",
    desc: "奥运会排球比赛规则",
    origin: "IOC/FIVB",
    tags: ["体育", "排球", "奥运会"]
}

impl VolleyballOlympicRules {
    /// 参赛队伍数量
    pub fn team_count(&self) -> u8 {
        12 // 12支队伍参赛
    }

    /// 小组数量
    pub fn group_count(&self) -> u8 {
        2 // 2个小组，每组6队
    }

    /// 小组晋级规则
    pub fn group_qualification(&self) -> Vec<&'static str> {
        vec![
            "每组前4名晋级淘汰赛",
            "小组赛采用循环赛制",
            "排名依据积分胜负关系",
            "第5-6名淘汰",
        ]
    }

    /// 比赛赛制
    pub fn match_format(&self) -> Vec<&'static str> {
        vec![
            "所有比赛5局3胜制",
            "前4局25分，决胜局15分",
            "必须领先2分获胜",
            "每局无上限分",
        ]
    }

    /// 淘汰赛结构
    pub fn knockout_structure(&self) -> Vec<&'static str> {
        vec!["八强淘汰赛", "半决赛", "决赛", "三四名决赛", "五六名决赛"]
    }

    /// 资格获取方式
    pub fn qualification_method(&self) -> Vec<&'static str> {
        vec!["洲际资格赛冠军", "世界排名", "主办国直接晋级", "奥运资格赛"]
    }

    /// 奖牌设置
    pub fn medal_allocation(&self) -> Vec<&'static str> {
        vec!["冠军: 金牌", "亚军: 银牌", "第三名: 铜牌", "第四名: 无奖牌"]
    }

    /// 检查小组是否晋级
    pub fn check_group_qualification(&self, group_position: u8) -> bool {
        (1..=4).contains(&group_position)
    }

    /// 检查是否有奖牌
    pub fn check_medal(&self, final_position: u8) -> Option<&'static str> {
        match final_position {
            1 => Some("金牌"),
            2 => Some("银牌"),
            3 => Some("铜牌"),
            _ => None,
        }
    }
}

impl Rule for VolleyballOlympicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("volleyball_olympic")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        format!(
            "【排球奥运会规则】\n\n\
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
            - {}\n\n\
            奖牌设置:\n\
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
            self.knockout_structure()[0],
            self.knockout_structure()[1],
            self.knockout_structure()[2],
            self.knockout_structure()[3],
            self.knockout_structure()[4],
            self.qualification_method()[0],
            self.qualification_method()[1],
            self.qualification_method()[2],
            self.qualification_method()[3],
            self.medal_allocation()[0],
            self.medal_allocation()[1],
            self.medal_allocation()[2],
            self.medal_allocation()[3]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let rules = VolleyballOlympicRules::new();
        assert_eq!(rules.metadata().name, "排球奥运会规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_team_count() {
        let rules = VolleyballOlympicRules::new();
        assert_eq!(rules.team_count(), 12);
        assert_eq!(rules.group_count(), 2);
    }

    #[test]
    fn test_qualification() {
        let rules = VolleyballOlympicRules::new();
        assert!(rules.check_group_qualification(1));
        assert!(rules.check_group_qualification(4));
        assert!(!rules.check_group_qualification(5));
    }

    #[test]
    fn test_medal() {
        let rules = VolleyballOlympicRules::new();
        assert_eq!(rules.check_medal(1), Some("金牌"));
        assert_eq!(rules.check_medal(2), Some("银牌"));
        assert_eq!(rules.check_medal(3), Some("铜牌"));
        assert_eq!(rules.check_medal(4), None);
    }
}
