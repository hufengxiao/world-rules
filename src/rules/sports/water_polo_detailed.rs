//! 水球详细规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 水球详细规则
pub struct WaterPoloDetailedRules {
    metadata: RuleMetadata,
}

impl WaterPoloDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "水球详细规则",
                "水球比赛详细规则"
            )
            .with_origin("英国")
            .with_tags(vec!["体育".into(), "水上".into()]),
        }
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间: 4节",
            "每节8分钟",
            "有效时间制",
            "节间休息2分钟",
            "加时规则",
        ]
    }

    /// 场地规格
    pub fn pool_specifications(&self) -> Vec<&'static str> {
        vec![
            "泳池尺寸: 30×20米",
            "水深: 最浅1.8米",
            "球门尺寸: 3×0.9米",
            "场地边界",
            "安全区域",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队7人上场",
            "1名守门员",
            "6名场上球员",
            "替补队员",
            "换人规则",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "游泳技术",
            "持球技术",
            "传球技术",
            "射门技术",
            "防守技术",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "普通犯规",
            "严重犯规",
            "暴力犯规",
            "犯规处罚",
            "罚球规则",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "球进入球门得分",
            "有效进球",
            "得分统计",
            "比分记录",
            "胜负判定",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "水球",
            "泳帽",
            "比赛泳衣",
            "防水装备",
            "场地装备",
        ]
    }
}

impl Default for WaterPoloDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WaterPoloDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("water_polo_detailed")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【水球详细规则】\n\n\
            场地规格:\n{}\n\n\
            技术动作:\n{}\n\n\
            得分规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.pool_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_water_polo_detailed_rules() {
        let rules = WaterPoloDetailedRules::new();
        assert!(!rules.competition_rules().is_empty());
    }
}