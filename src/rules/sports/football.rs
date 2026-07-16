//! 足球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 足球比赛状态
///
/// 记录比赛当前的比分、牌数和时间信息。
///
/// # 示例
/// ```
/// use world_rules::rules::sports::football::FootballMatchState;
///
/// let state = FootballMatchState {
///     score_home: 2,
///     score_away: 1,
///     yellow_cards_home: 1,
///     yellow_cards_away: 2,
///     red_cards_home: 0,
///     red_cards_away: 0,
///     half: 2,
///     minutes: 75,
/// };
/// assert_eq!(state.score_home, 2);
/// ```
#[derive(Debug, Clone)]
pub struct FootballMatchState {
    /// 主队进球数
    pub score_home: u8,
    /// 客队进球数
    pub score_away: u8,
    /// 主队黄牌数
    pub yellow_cards_home: u8,
    /// 客队黄牌数
    pub yellow_cards_away: u8,
    /// 主队红牌数
    pub red_cards_home: u8,
    /// 客队红牌数
    pub red_cards_away: u8,
    /// 当前半场 (1 或 2)
    pub half: u8,
    /// 当前分钟数
    pub minutes: u8,
}

/// 足球规则
///
/// 实现国际足联（FIFA）标准足球规则。
///
/// # 示例
/// ```
/// use world_rules::rules::sports::football::FootballRules;
///
/// let rules = FootballRules::new();
/// assert_eq!(rules.match_duration(), 90);
/// assert_eq!(rules.team_size(), 11);
/// ```
pub struct FootballRules {
    metadata: RuleMetadata,
}

impl FootballRules {
    /// 创建新的足球规则实例
    ///
    /// # 示例
    /// ```
    /// use world_rules::rules::sports::football::FootballRules;
    ///
    /// let rules = FootballRules::new();
    /// assert_eq!(rules.match_duration(), 90);
    /// ```
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("足球规则", "国际足联 (FIFA) 标准足球规则")
                .with_origin("FIFA")
                .with_tags(vec!["体育".into(), "足球".into()]),
        }
    }

    /// 比赛时长
    pub fn match_duration(&self) -> u16 {
        90 // 标准比赛90分钟
    }

    /// 半场时长
    pub fn half_duration(&self) -> u16 {
        45
    }

    /// 球场尺寸
    pub fn field_dimensions(&self) -> (u16, u16) {
        // 长度: 90-120米, 宽度: 45-90米
        // 国际标准: 105×68米
        (105, 68)
    }

    /// 球队人数
    pub fn team_size(&self) -> u8 {
        11
    }

    /// 检查越位 (简化版)
    pub fn check_offside(&self, attacker_position: u8, last_defender_position: u8) -> bool {
        attacker_position > last_defender_position
    }

    /// 获取犯规处罚
    pub fn get_foul_penalty(&self, foul_type: FoulType) -> Penalty {
        match foul_type {
            FoulType::Minor => Penalty::FreeKick,
            FoulType::Serious => Penalty::YellowCard,
            FoulType::Violent => Penalty::RedCard,
            FoulType::PenaltyArea => Penalty::PenaltyKick,
        }
    }
}

/// 犯规类型
///
/// 足球比赛中不同类型的犯规。
///
/// # 示例
/// ```
/// use world_rules::rules::sports::football::{FootballRules, FoulType, Penalty};
///
/// let rules = FootballRules::new();
/// let penalty = rules.get_foul_penalty(FoulType::Violent);
/// assert!(matches!(penalty, Penalty::RedCard));
/// ```
#[derive(Debug, Clone)]
pub enum FoulType {
    /// 轻微犯规
    Minor,
    /// 严重犯规
    Serious,
    /// 暴力犯规
    Violent,
    /// 禁区内犯规
    PenaltyArea,
}

/// 处罚类型
///
/// 足球比赛中针对犯规的不同处罚。
#[derive(Debug, Clone)]
pub enum Penalty {
    /// 任意球
    FreeKick,
    /// 黄牌
    YellowCard,
    /// 红牌
    RedCard,
    /// 点球
    PenaltyKick,
}

impl Default for FootballRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FootballRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football")
    }

    fn explain(&self) -> String {
        let (length, width) = self.field_dimensions();
        format!(
            "【足球规则】\n\n\
            比赛时长: {}分钟 (上下半场各{}分钟)\n\
            球场尺寸: {}×{}米\n\
            每队人数: {}人 (含门将)\n\n\
            基本规则:\n\
            1. 除门将外，其他球员不能用手触球\n\
            2. 越位规则: 传球时进攻球员不能比最后防守球员更靠近球门\n\
            3. 犯规处罚:\n\
               - 一般犯规: 任意球\n\
               - 严重犯规: 黄牌警告\n\
               - 暴力犯规: 红牌罚下\n\
               - 禁区内犯规: 点球\n\
            4. 两黄变一红\n\
            5. 比赛结束比分高者获胜",
            self.match_duration(),
            self.half_duration(),
            length,
            width,
            self.team_size()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_football_rules() {
        let rules = FootballRules::new();
        assert_eq!(rules.team_size(), 11);
        assert_eq!(rules.match_duration(), 90);
    }
}

#[cfg(test)]
mod extra_tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_field_dimensions() {
        let rules = FootballRules::new();
        let (w, h) = rules.field_dimensions();
        assert!((100..=110).contains(&w));
        assert!((64..=75).contains(&h));
    }

    #[test]
    fn test_offside_detection() {
        let rules = FootballRules::new();
        // 攻击者在最后一名防守球员之后 → 越位
        assert!(rules.check_offside(30, 25));
        // 攻击者与最后一名防守球员齐平或在前 → 不越位
        assert!(!rules.check_offside(25, 25));
        assert!(!rules.check_offside(20, 25));
    }

    #[test]
    fn test_foul_penalties() {
        let rules = FootballRules::new();
        assert!(matches!(
            rules.get_foul_penalty(FoulType::Minor),
            Penalty::FreeKick
        ));
        assert!(matches!(
            rules.get_foul_penalty(FoulType::Serious),
            Penalty::YellowCard
        ));
        assert!(matches!(
            rules.get_foul_penalty(FoulType::Violent),
            Penalty::RedCard
        ));
        assert!(matches!(
            rules.get_foul_penalty(FoulType::PenaltyArea),
            Penalty::PenaltyKick
        ));
    }

    #[test]
    fn test_half_duration() {
        let rules = FootballRules::new();
        assert_eq!(rules.half_duration(), 45);
    }

    #[test]
    fn test_rule_trait() {
        let rules = FootballRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("match".to_string()))
            .is_ok());
        assert!(!rules.explain().is_empty());
        assert_eq!(rules.category(), RuleCategory::sports("football"));
    }
}
