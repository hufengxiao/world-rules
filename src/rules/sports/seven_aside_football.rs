//! 七人制足球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 七人制足球规则
pub struct SevenAsideFootballRules {
    metadata: RuleMetadata,
}

impl SevenAsideFootballRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "七人制足球规则",
                "七人制足球比赛基本规则"
            )
            .with_origin("欧洲")
            .with_tags(vec!["体育".into(), "足球".into()]),
        }
    }

    /// 球场规格
    pub fn field_specifications(&self) -> Vec<&'static str> {
        vec![
            "长度: 50-70米",
            "宽度: 35-50米",
            "禁区半径9米",
            "罚球点距门9米",
            "门宽: 5米×高2米",
        ]
    }

    /// 队伍构成
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队7人上场",
            "1名守门员",
            "6名场上球员",
            "替补不限",
            "换人次数不限",
        ]
    }

    /// 比赛时间
    pub fn game_duration(&self) -> Vec<&'static str> {
        vec![
            "两半场各25-30分钟",
            "半场休息10分钟",
            "加时赛可选",
            "时间灵活调整",
            "青少年缩短时间",
        ]
    }

    /// 球规格
    pub fn ball_specifications(&self) -> Vec<&'static str> {
        vec![
            "4号或5号球",
            "重量标准",
            "材质要求",
            "气压规定",
            "比赛专用球",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "越位规则简化",
            "禁区犯规罚球",
            "犯规处罚",
            "黄牌红牌",
            "累积犯规",
        ]
    }

    /// 换人规则
    pub fn substitution_rules(&self) -> Vec<&'static str> {
        vec![
            "换人次数不限",
            "死球时换人",
            "换下可换上",
            "裁判允许",
            "边线换人",
        ]
    }

    /// 裁判职责
    pub fn referee_duties(&self) -> Vec<&'static str> {
        vec![
            "主裁判1人",
            "边裁1-2人可选",
            "比赛控制",
            "犯规判定",
            "时间管理",
        ]
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "业余比赛",
            "青少年比赛",
            "友谊赛",
            "小型联赛",
            "企业比赛",
        ]
    }
}

impl Default for SevenAsideFootballRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SevenAsideFootballRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("seven_aside_football")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【七人制足球规则】\n\n\
            球场规格:\n{}\n\n\
            队伍构成:\n{}\n\n\
            比赛时间:\n{}\n\n\
            犯规规则:\n{}\n",
            self.field_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.team_composition().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.game_duration().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seven_aside_football_rules() {
        let rules = SevenAsideFootballRules::new();
        assert!(!rules.field_specifications().is_empty());
    }
}