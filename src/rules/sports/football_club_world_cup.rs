//! 俱乐部世界杯规则 - FIFA Club World Cup

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 俱乐部世界杯比赛阶段
#[derive(Debug, Clone, PartialEq)]
pub enum ClubWorldCupStage {
    /// 第一轮
    FirstRound,
    /// 第二轮
    SecondRound,
    /// 半决赛
    SemiFinal,
    /// 决赛
    Final,
    /// 三四名决赛
    ThirdPlace,
}

/// 俱乐部世界杯规则
pub struct FootballClubWorldCupRules {
    metadata: RuleMetadata,
}

impl FootballClubWorldCupRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("俱乐部世界杯规则", "FIFA 俱乐部世界杯规则")
                .with_origin("FIFA")
                .with_tags(vec![
                    "体育".into(),
                    "足球".into(),
                    "俱乐部".into(),
                    "国际".into(),
                ]),
        }
    }

    /// 参赛球队数量（传统版本）
    pub fn team_count(&self) -> u8 {
        7 // 6大洲冠军 + 东道主联赛冠军
    }

    /// 参赛球队数量（新版2025起）
    pub fn expanded_team_count(&self) -> u8 {
        32 // 2025起扩军至32队
    }

    /// 参赛资格
    pub fn qualification_rule(&self) -> String {
        "欧冠冠军、南美解放者杯冠军等六大洲俱乐部冠军 + 东道主".to_string()
    }

    /// 比赛轮次结构
    pub fn match_structure(&self) -> String {
        "东道主 vs 大洋洲冠军，胜者进入第二轮对阵其他洲冠军，半决赛对阵欧冠和南美冠军".to_string()
    }

    /// 加时赛规则
    pub fn extra_time_rule(&self) -> String {
        "淘汰赛加时赛30分钟，若仍平局进入点球大战".to_string()
    }

    /// 获取比赛对阵
    pub fn knockout_pairing(&self, stage: ClubWorldCupStage) -> String {
        match stage {
            ClubWorldCupStage::FirstRound => "东道主联赛冠军 vs 大洋洲冠军".to_string(),
            ClubWorldCupStage::SecondRound => "第一轮胜者 vs 亚非北美冠军之一".to_string(),
            ClubWorldCupStage::SemiFinal => {
                "第二轮胜者 vs 南美冠军；欧冠冠军 vs 第二轮另一胜者".to_string()
            }
            ClubWorldCupStage::Final => "半决赛胜者对决".to_string(),
            ClubWorldCupStage::ThirdPlace => "半决赛负者对决".to_string(),
        }
    }

    /// 赛事举办周期（传统）
    pub fn tournament_cycle(&self) -> u8 {
        1 // 每年举办一次
    }

    /// 新版举办周期
    pub fn expanded_cycle(&self) -> u8 {
        4 // 2025起每4年举办一次
    }

    /// 历史冠军
    pub fn historical_champions(&self) -> String {
        "皇马、巴萨、拜仁、利物浦等欧洲豪门多次夺冠".to_string()
    }

    /// 赛事价值
    pub fn tournament_value(&self) -> String {
        "检验各洲俱乐部实力的最高舞台".to_string()
    }
}

impl Default for FootballClubWorldCupRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballClubWorldCupRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_club_world_cup")
    }

    fn explain(&self) -> String {
        format!(
            "【俱乐部世界杯规则】\n\n\
            传统参赛: {} 支俱乐部（6大洲冠军+东道主）\n\
            新版参赛: {} 支俱乐部（2025起）\n\
            参赛资格: {}\n\n\
            比赛结构:\n\
            {}\n\n\
            淘汰赛规则:\n\
            - {}\n\
            - 三四名决赛由半决赛负者进行\n\n\
            赛事特色:\n\
            1. 传统版每年{}举办，新版每{}年举办\n\
            2. {}\n\
            3. {}",
            self.team_count(),
            self.expanded_team_count(),
            self.qualification_rule(),
            self.match_structure(),
            self.extra_time_rule(),
            self.tournament_cycle(),
            self.expanded_cycle(),
            self.historical_champions(),
            self.tournament_value()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_club_world_cup_basic() {
        let rules = FootballClubWorldCupRules::new();
        assert_eq!(rules.team_count(), 7);
        assert_eq!(rules.expanded_team_count(), 32);
    }

    #[test]
    fn test_match_structure() {
        let rules = FootballClubWorldCupRules::new();
        assert!(rules.match_structure().contains("东道主"));
        assert!(rules.match_structure().contains("大洋洲"));
    }

    #[test]
    fn test_knockout_pairing() {
        let rules = FootballClubWorldCupRules::new();
        assert!(rules
            .knockout_pairing(ClubWorldCupStage::FirstRound)
            .contains("东道主"));
        assert!(rules
            .knockout_pairing(ClubWorldCupStage::Final)
            .contains("决赛"));
    }

    #[test]
    fn test_historical_champions() {
        let rules = FootballClubWorldCupRules::new();
        assert!(rules.historical_champions().contains("皇马"));
    }

    #[test]
    fn test_rule_trait() {
        let rules = FootballClubWorldCupRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("match".to_string()))
            .is_ok());
        assert!(!rules.explain().is_empty());
        assert_eq!(
            rules.category(),
            RuleCategory::sports("football_club_world_cup")
        );
    }
}
