//! 无人机竞速规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 无人机竞速规则
pub struct DroneRacingRules {
    metadata: RuleMetadata,
}

impl DroneRacingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "无人机竞速规则",
                "无人机竞速比赛规则"
            )
            .with_origin("国际")
            .with_tags(vec!["体育".into(), "科技".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "竞速比赛",
            "花样比赛",
            "FPV竞速",
            "障碍比赛",
            "综合比赛",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间限制",
            "赛道规定",
            "出发规则",
            "终点判定",
            "安全规则",
        ]
    }

    /// 技术操作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "遥控操控",
            "FPV技术",
            "速度控制",
            "转弯技术",
            "花样动作",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "竞速无人机",
            "FPV眼镜",
            "遥控设备",
            "比赛服装",
            "安全装备",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "飞行区域限制",
            "安全距离",
            " spectators安全",
            "救援准备",
            "场地安全",
        ]
    }

    /// 评分标准
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "完成时间",
            "花样评分",
            "障碍评分",
            "技术评分",
            "综合评分",
        ]
    }

    /// 级别体系
    pub fn skill_levels(&self) -> Vec<&'static str> {
        vec![
            "初学者级别",
            "中级水平",
            "高级水平",
            "专业级别",
            "教练认证",
        ]
    }
}

impl Default for DroneRacingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for DroneRacingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("drone_racing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【无人机竞速规则】\n\n\
            比赛类型:\n{}\n\n\
            技术操作:\n{}\n\n\
            安全规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.competition_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drone_racing_rules() {
        let rules = DroneRacingRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}