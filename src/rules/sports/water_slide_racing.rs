//! 水上滑梯规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 水上滑梯规则 (Water Slide Racing)
pub struct WaterSlideRacingRules {
    metadata: RuleMetadata,
}

impl WaterSlideRacingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "水上滑梯规则",
                "水上滑梯比赛规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "水上".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "竞速比赛",
            "花样比赛",
            "团体比赛",
            "表演比赛",
            "综合比赛",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间限制",
            "出发规则",
            "终点判定",
            "安全规则",
            "比赛结束",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "滑行技术",
            "姿势控制",
            "花样动作",
            "速度控制",
            "安全技术",
        ]
    }

    /// 评分标准
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "完成时间",
            "花样评分",
            "技术评分",
            "综合评分",
            "排名规则",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "安全检查",
            "防护装备",
            "医疗支持",
            "场地安全",
            "应急处理",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "泳衣",
            "安全装备",
            "场地装备",
            "计时设备",
            "附属配件",
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

impl Default for WaterSlideRacingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WaterSlideRacingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("water_slide_racing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【水上滑梯规则】\n\n\
            比赛类型:\n{}\n\n\
            技术动作:\n{}\n\n\
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
    fn test_water_slide_racing_rules() {
        let rules = WaterSlideRacingRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}