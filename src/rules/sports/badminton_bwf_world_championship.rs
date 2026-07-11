//! 羽毛球世锦赛规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: BadmintonBwfWorldChampionshipRules,
    name: "羽毛球世锦赛规则",
    desc: "BWF羽毛球世界锦标赛规则",
    origin: "BWF",
    tags: ["体育", "羽毛球", "世锦赛"]
}

impl BadmintonBwfWorldChampionshipRules {
    /// 参赛项目
    pub fn event_types(&self) -> Vec<&'static str> {
        vec!["男子单打", "女子单打", "男子双打", "女子双打", "混合双打"]
    }

    /// 参赛人数
    pub fn player_count_per_event(&self) -> Vec<&'static str> {
        vec![
            "单打: 64名选手",
            "双打: 32对组合",
            "种子选手: 前8位",
            "主办国选手名额",
        ]
    }

    /// 比赛赛制
    pub fn match_format(&self) -> Vec<&'static str> {
        vec![
            "单打: 三局两胜制",
            "双打: 三局两胜制",
            "每局21分",
            "必须领先2分获胜",
            "29-29时先到30分获胜",
        ]
    }

    /// 淘汰赛结构
    pub fn knockout_structure(&self) -> Vec<&'static str> {
        vec![
            "首轮: 64进32",
            "第二轮: 32进16",
            "第三轮: 16进8",
            "半决赛: 8进4",
            "决赛: 决出冠军",
        ]
    }

    /// 资格获取方式
    pub fn qualification_method(&self) -> Vec<&'static str> {
        vec!["BWF世界排名", "洲际资格赛", "主办国名额", "邀请名额"]
    }

    /// 赛事周期
    pub fn tournament_cycle(&self) -> Vec<&'static str> {
        vec![
            "每年举办一次",
            "奥运年不举办",
            "与奥运会错开",
            "全年积分累计",
        ]
    }

    /// 种子选手规则
    pub fn seeding_rules(&self) -> Vec<&'static str> {
        vec![
            "前8名设为种子",
            "种子分散在不同区",
            "1号种子在上半区",
            "2号种子在下半区",
        ]
    }

    /// 检查局分是否获胜
    pub fn check_game_win(&self, score: u8, opponent_score: u8) -> bool {
        if score >= 21 {
            if score >= 30 {
                true // 30分上限
            } else {
                score - opponent_score >= 2
            }
        } else {
            false
        }
    }
}

impl Rule for BadmintonBwfWorldChampionshipRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("badminton_bwf_world_championship")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "羽毛球世锦赛规则",
            &[
                ("比赛项目", &self.event_types()),
                ("参赛规模", &self.player_count_per_event()),
                ("比赛赛制", &self.match_format()),
                ("淘汰赛", &self.knockout_structure()),
                ("资格获取", &self.qualification_method()),
                ("种子规则", &self.seeding_rules()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let rules = BadmintonBwfWorldChampionshipRules::new();
        assert_eq!(rules.metadata().name, "羽毛球世锦赛规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_events() {
        let rules = BadmintonBwfWorldChampionshipRules::new();
        assert_eq!(rules.event_types().len(), 5);
    }

    #[test]
    fn test_game_win() {
        let rules = BadmintonBwfWorldChampionshipRules::new();
        assert!(rules.check_game_win(21, 19));
        assert!(rules.check_game_win(30, 29));
        assert!(!rules.check_game_win(21, 20));
        assert!(!rules.check_game_win(20, 19));
    }
}
