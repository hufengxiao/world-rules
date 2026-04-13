//! 冰球比赛规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 冰球比赛规则 (详细版)
pub struct IceHockeyDetailedRules {
    metadata: RuleMetadata,
}

impl IceHockeyDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "冰球比赛规则",
                "冰球比赛详细规则"
            )
            .with_origin("加拿大")
            .with_tags(vec!["体育".into(), "冬季".into()]),
        }
    }

    /// 比赛时间
    pub fn game_duration(&self) -> Vec<&'static str> {
        vec![
            "三节比赛",
            "每节20分钟",
            "有效时间制",
            "节间休息15分钟",
            "加时赛规则",
        ]
    }

    /// 场地规格
    pub fn rink_specifications(&self) -> Vec<&'static str> {
        vec![
            "冰场尺寸: 60×30米(NHL)",
            "国际尺寸: 60×26米",
            "球门尺寸: 1.83×1.22米",
            "蓝线和红线",
            "进攻区域划分",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队6人上场",
            "1名守门员",
            "5名场上球员",
            "三前锋两后卫",
            "替补轮换",
        ]
    }

    /// 犯规规则
    pub fn penalties(&self) -> Vec<&'static str> {
        vec![
            "小罚: 2分钟",
            "大罚: 5分钟",
            " misconduct: 10分钟",
            "比赛罚: 取消资格",
            "罚时区执行",
        ]
    }

    /// 犯规行为
    pub fn foul_actions(&self) -> Vec<&'static str> {
        vec![
            "非法冲撞",
            "用杆击人",
            "绊人",
            "干扰",
            "危险动作",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "冰球越位",
            "传球越位",
            " icing规则",
            "争球规则",
            "换人规则",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "冰球杆",
            "冰球鞋",
            "头盔必须",
            "护具齐全",
            "守门员装备",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "球完全越过球门线",
            "有效进球判定",
            "得分统计",
            "助攻统计",
            "比分记录",
        ]
    }
}

impl Default for IceHockeyDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for IceHockeyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("ice_hockey_detailed")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【冰球比赛规则】\n\n\
            犯规规则:\n{}\n\n\
            比赛规则:\n{}\n\n\
            装备要求:\n{}\n\n\
            得分规则:\n{}\n",
            self.penalties().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.competition_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ice_hockey_detailed_rules() {
        let rules = IceHockeyDetailedRules::new();
        assert!(!rules.game_duration().is_empty());
    }
}