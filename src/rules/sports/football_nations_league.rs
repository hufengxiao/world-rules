//! 国家联赛规则 - UEFA Nations League

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 国家联赛级别
#[derive(Debug, Clone, PartialEq)]
pub enum NationsLeagueLevel {
    /// A级（顶级）
    LevelA,
    /// B级
    LevelB,
    /// C级
    LevelC,
    /// D级（最低）
    LevelD,
}

/// 国家联赛比赛阶段
#[derive(Debug, Clone, PartialEq)]
pub enum NationsLeagueStage {
    /// 小组赛
    GroupStage,
    /// 半决赛
    SemiFinal,
    /// 决赛
    Final,
    /// 三四名决赛
    ThirdPlace,
}

/// 国家联赛规则
pub struct FootballNationsLeagueRules {
    metadata: RuleMetadata,
}

impl FootballNationsLeagueRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("国家联赛规则", "UEFA 欧洲国家联赛规则")
                .with_origin("UEFA")
                .with_tags(vec![
                    "体育".into(),
                    "足球".into(),
                    "国家队".into(),
                    "欧洲".into(),
                ]),
        }
    }

    /// A级球队数量
    pub fn level_a_teams(&self) -> u8 {
        16 // A级16队
    }

    /// 各级别小组数量
    pub fn group_count_per_level(&self) -> u8 {
        4 // 每级别4个小组
    }

    /// 小组赛比赛场次
    pub fn group_matches_per_team(&self) -> u8 {
        6 // 每队小组赛6场（主客场）
    }

    /// A级晋级规则
    pub fn level_a_knockout_rule(&self) -> String {
        "A级各组第1名进入半决赛，争夺国家联赛冠军".to_string()
    }

    ///升降级规则
    pub fn promotion_relegation_rule(&self) -> String {
        "各组第1名升级，各组最后1名降级（C级有附加赛）".to_string()
    }

    /// 与欧洲杯关联
    pub fn euro_cup_qualification_link(&self) -> String {
        "国家联赛成绩可提供欧洲杯预选赛附加赛资格".to_string()
    }

    /// 加时赛规则
    pub fn extra_time_rule(&self) -> String {
        "半决赛、决赛加时赛30分钟，若仍平局进入点球大战".to_string()
    }

    /// 获取级别球队数量
    pub fn teams_per_level(&self, level: NationsLeagueLevel) -> u8 {
        match level {
            NationsLeagueLevel::LevelA => 16,
            NationsLeagueLevel::LevelB => 16,
            NationsLeagueLevel::LevelC => 16,
            NationsLeagueLevel::LevelD => 7,
        }
    }

    /// 获取淘汰赛对阵
    pub fn knockout_pairing(&self, stage: NationsLeagueStage) -> String {
        match stage {
            NationsLeagueStage::SemiFinal => "4个小组第1名交叉对阵".to_string(),
            NationsLeagueStage::Final => "半决赛胜者对决".to_string(),
            NationsLeagueStage::ThirdPlace => "半决赛负者对决".to_string(),
            NationsLeagueStage::GroupStage => "小组赛主客场循环".to_string(),
        }
    }

    /// 赛事举办周期
    pub fn tournament_cycle(&self) -> u8 {
        2 // 每2年举办一次
    }

    /// 比赛时间窗口
    pub fn match_windows(&self) -> String {
        "在9月、10月、11月国际比赛窗口进行".to_string()
    }
}

impl Default for FootballNationsLeagueRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballNationsLeagueRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_nations_league")
    }

    fn explain(&self) -> String {
        format!(
            "【国家联赛规则】\n\n\
            分级制度: A、B、C、D四个级别\n\
            A级球队: {} 支顶级国家队\n\
            小组数量: 每级别{}个小组\n\
            每队场次: {}场小组赛（主客场）\n\n\
            A级淘汰赛: {}\n\
            升降级: {}\n\n\
            与欧洲杯关联: {}\n\n\
            赛事特色:\n\
            1. 每{}年举办一次\n\
            2. {}\n\
            3. 替代传统友谊赛，增加竞技性",
            self.level_a_teams(),
            self.group_count_per_level(),
            self.group_matches_per_team(),
            self.level_a_knockout_rule(),
            self.promotion_relegation_rule(),
            self.euro_cup_qualification_link(),
            self.tournament_cycle(),
            self.match_windows()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_nations_league_basic() {
        let rules = FootballNationsLeagueRules::new();
        assert_eq!(rules.level_a_teams(), 16);
        assert_eq!(rules.group_count_per_level(), 4);
        assert_eq!(rules.group_matches_per_team(), 6);
    }

    #[test]
    fn test_teams_per_level() {
        let rules = FootballNationsLeagueRules::new();
        assert_eq!(rules.teams_per_level(NationsLeagueLevel::LevelA), 16);
        assert_eq!(rules.teams_per_level(NationsLeagueLevel::LevelD), 7);
    }

    #[test]
    fn test_promotion_relegation() {
        let rules = FootballNationsLeagueRules::new();
        assert!(rules.promotion_relegation_rule().contains("升级"));
        assert!(rules.promotion_relegation_rule().contains("降级"));
    }

    #[test]
    fn test_euro_cup_link() {
        let rules = FootballNationsLeagueRules::new();
        assert!(rules.euro_cup_qualification_link().contains("欧洲杯"));
    }

    #[test]
    fn test_rule_trait() {
        let rules = FootballNationsLeagueRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("match".to_string()))
            .is_ok());
        assert!(!rules.explain().is_empty());
        assert_eq!(
            rules.category(),
            RuleCategory::sports("football_nations_league")
        );
    }
}
