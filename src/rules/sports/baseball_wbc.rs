//! 世界棒球经典赛规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

/// WBC赛制阶段
#[derive(Debug, Clone, Copy)]
pub enum WbcStage {
    /// 小组赛
    PoolRound,
    /// 淘汰赛
    QuarterFinal,
    /// 半决赛
    SemiFinal,
    /// 决赛
    Final,
}

simple_rule! {
    struct: BaseballWbcRules,
    name: "世界棒球经典赛规则",
    desc: "世界棒球经典赛(WBC)国际棒球锦标赛规则",
    origin: "WBCI",
    tags: ["体育", "棒球", "国际赛"]
}

impl BaseballWbcRules {
    /// 参赛队伍数量
    pub fn team_count(&self) -> u8 {
        20 // 20支国家队
    }

    /// 小组数量
    pub fn pool_count(&self) -> u8 {
        4 // 4个小组
    }

    /// 每组队伍数量
    pub fn teams_per_pool(&self) -> u8 {
        5
    }

    /// 小组赛规则
    pub fn pool_round_rules(&self) -> Vec<&'static str> {
        vec![
            "每组5队循环赛",
            "每组前2名晋级淘汰赛",
            "每组后3名淘汰",
            "排名依据胜负关系",
        ]
    }

    /// 比赛赛制
    pub fn game_rules(&self) -> Vec<&'static str> {
        vec![
            "小组赛: 限制投球数",
            "淘汰赛: 正规9局",
            "延长赛规则",
            " mercy rule适用",
            "投球数保护规则",
        ]
    }

    /// 投球数限制
    pub fn pitch_limit_rules(&self) -> Vec<&'static str> {
        vec![
            "小组赛首轮: 65球",
            "小组赛次轮: 80球",
            "淘汰赛: 95球",
            "决赛: 100球",
            "休息天数限制",
        ]
    }

    /// 淘汰赛规则
    pub fn knockout_rules(&self) -> Vec<&'static str> {
        vec!["八强淘汰赛", "半决赛", "决赛", "单场淘汰制", "无三四名决赛"]
    }

    /// 队员资格
    pub fn player_eligibility(&self) -> Vec<&'static str> {
        vec!["本国国籍或血统", "永久居留权", "父母国籍", "可选择代表国家"]
    }

    /// 赛事周期
    pub fn tournament_cycle(&self) -> Vec<&'static str> {
        vec!["每4年举办一次", "春季举办", "MLB赛季前", "国际棒球最高水平"]
    }

    /// 获取阶段描述
    pub fn stage_description(&self, stage: WbcStage) -> &'static str {
        match stage {
            WbcStage::PoolRound => "小组赛",
            WbcStage::QuarterFinal => "八强淘汰赛",
            WbcStage::SemiFinal => "半决赛",
            WbcStage::Final => "决赛",
        }
    }

    /// 检查小组是否晋级
    pub fn check_pool_qualification(&self, pool_position: u8) -> bool {
        (1..=2).contains(&pool_position)
    }
}

impl Rule for BaseballWbcRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("baseball_wbc")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        format!(
            "【世界棒球经典赛规则】\n\n\
            参赛队伍: {} 支\n\
            小组数量: {} 个\n\
            每组队伍: {} 支\n\n\
            小组赛规则:\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}\n\n\
            比赛规则:\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}\n\n\
            投球数限制:\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}\n\n\
            淘汰赛:\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}\n\n\
            队员资格:\n\
            - {}\n\
            - {}\n\
            - {}\n\
            - {}",
            self.team_count(),
            self.pool_count(),
            self.teams_per_pool(),
            self.pool_round_rules()[0],
            self.pool_round_rules()[1],
            self.pool_round_rules()[2],
            self.pool_round_rules()[3],
            self.game_rules()[0],
            self.game_rules()[1],
            self.game_rules()[2],
            self.game_rules()[3],
            self.game_rules()[4],
            self.pitch_limit_rules()[0],
            self.pitch_limit_rules()[1],
            self.pitch_limit_rules()[2],
            self.pitch_limit_rules()[3],
            self.pitch_limit_rules()[4],
            self.knockout_rules()[0],
            self.knockout_rules()[1],
            self.knockout_rules()[2],
            self.knockout_rules()[3],
            self.knockout_rules()[4],
            self.player_eligibility()[0],
            self.player_eligibility()[1],
            self.player_eligibility()[2],
            self.player_eligibility()[3]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let rules = BaseballWbcRules::new();
        assert_eq!(rules.metadata().name, "世界棒球经典赛规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_team_count() {
        let rules = BaseballWbcRules::new();
        assert_eq!(rules.team_count(), 20);
        assert_eq!(rules.pool_count(), 4);
        assert_eq!(rules.teams_per_pool(), 5);
    }

    #[test]
    fn test_pool_qualification() {
        let rules = BaseballWbcRules::new();
        assert!(rules.check_pool_qualification(1));
        assert!(rules.check_pool_qualification(2));
        assert!(!rules.check_pool_qualification(3));
    }
}
