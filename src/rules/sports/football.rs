//! 足球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 足球比赛状态
#[derive(Debug, Clone)]
pub struct FootballMatchState {
    pub score_home: u8,
    pub score_away: u8,
    pub yellow_cards_home: u8,
    pub yellow_cards_away: u8,
    pub red_cards_home: u8,
    pub red_cards_away: u8,
    pub half: u8,
    pub minutes: u8,
}

/// 足球规则
pub struct FootballRules {
    metadata: RuleMetadata,
}

impl FootballRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "足球规则",
                "国际足联 (FIFA) 标准足球规则"
            )
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
#[derive(Debug, Clone)]
pub enum FoulType {
    Minor,
    Serious,
    Violent,
    PenaltyArea,
}

/// 处罚类型
#[derive(Debug, Clone)]
pub enum Penalty {
    FreeKick,
    YellowCard,
    RedCard,
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

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
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