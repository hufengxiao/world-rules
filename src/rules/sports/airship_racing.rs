//! 飞艇比赛规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 飞艇比赛规则
pub struct AirshipRacingRules {
    metadata: RuleMetadata,
}

impl AirshipRacingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "飞艇比赛规则",
                "飞艇比赛规则"
            )
            .with_origin("法国")
            .with_tags(vec!["体育".into(), "航空".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "竞速比赛",
            "精准比赛",
            "耐力比赛",
            "花样比赛",
            "综合比赛",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛路线",
            "时间限制",
            "出发规则",
            "终点判定",
            "安全规则",
        ]
    }

    /// 技术操作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "飞艇操控",
            "速度控制",
            "方向控制",
            "着陆技术",
            "花样操作",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "飞艇",
            "动力系统",
            "安全装备",
            "导航设备",
            "应急装备",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "天气条件限制",
            "高度限制",
            "安全区域",
            "救援准备",
            "医疗支持",
        ]
    }

    /// 评分标准
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "完成时间",
            "精准度评分",
            "花样评分",
            "综合评分",
            "排名规则",
        ]
    }

    /// 级别体系
    pub fn skill_levels(&self) -> Vec<&'static str> {
        vec![
            "初级驾驶员",
            "中级水平",
            "高级水平",
            "专业级别",
            "教练认证",
        ]
    }
}

impl Default for AirshipRacingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AirshipRacingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("airship_racing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【飞艇比赛规则】\n\n\
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
    fn test_airship_racing_rules() {
        let rules = AirshipRacingRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}