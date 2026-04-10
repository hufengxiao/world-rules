//! 水球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 水球规则
pub struct WaterPoloRules {
    metadata: RuleMetadata,
}

impl WaterPoloRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "水球规则",
                "水球比赛基本规则"
            )
            .with_origin("英国")
            .with_tags(vec!["体育".into(), "水上".into()]),
        }
    }

    /// 场地规格
    pub fn pool_dimensions(&self) -> Vec<&'static str> {
        vec![
            "泳池: 长25-30米，宽20米",
            "水深: 至少1.8米",
            "球门: 高90厘米，宽3米",
            "球门线到池边至少30厘米",
            "标记线: 2米、5米线",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "场上队员: 7人(含守门员)",
            "替补队员: 6人",
            "换人次数不限",
            "可在比赛进行中换人",
            "守门员戴红色帽子",
        ]
    }

    /// 比赛时间
    pub fn match_duration(&self) -> Vec<&'static str> {
        vec![
            "四节比赛，每节8分钟",
            "节间休息2分钟",
            "中场休息3分钟",
            "有效时间制(停表)",
            "平局后点球决胜",
        ]
    }

    /// 基本规则
    pub fn basic_rules(&self) -> Vec<&'static str> {
        vec![
            "只能单手控球(守门员除外)",
            "不得持球潜水",
            "不得拉扯对方",
            "不得阻挡对方无球队员",
            "必须在水中游泳移动",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "球完全越过球门线得1分",
            "射门前最多持球35秒",
            "30秒进攻计时",
            "可在任何位置射门",
            "得分多者获胜",
        ]
    }

    /// 犯规与处罚
    pub fn fouls_penalties(&self) -> Vec<&'static str> {
        vec![
            "普通犯规: 自由球",
            "严重犯规: 罚出场20秒",
            "暴力犯规: 红牌驱逐",
            "5米内犯规: 点球",
            "累计3次严重犯规罚出场",
        ]
    }

    /// 守门员规则
    pub fn goalkeeper_rules(&self) -> Vec<&'static str> {
        vec![
            "可在2米区域内用双手触球",
            "可在水中站立",
            "可扑球至任何位置",
            "不得越过半场",
            "红帽子区分身份",
        ]
    }
}

impl Default for WaterPoloRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WaterPoloRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("water_polo")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【水球规则】\n\n\
            场地规格:\n{}\n\n\
            基本规则:\n{}\n\n\
            得分规则:\n{}\n\n\
            犯规与处罚:\n{}\n",
            self.pool_dimensions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.basic_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls_penalties().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_water_polo_rules() {
        let rules = WaterPoloRules::new();
        assert!(!rules.pool_dimensions().is_empty());
    }
}