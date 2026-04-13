//! 热气球比赛规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 热气球比赛规则
pub struct HotAirBalloonRules {
    metadata: RuleMetadata,
}

impl HotAirBalloonRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "热气球比赛规则",
                "热气球比赛规则"
            )
            .with_origin("法国")
            .with_tags(vec!["体育".into(), "航空".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "精准着陆比赛",
            "距离比赛",
            "高度比赛",
            "速度比赛",
            "综合比赛",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间限制",
            "目标设定",
            "出发规则",
            "终点判定",
            "安全规则",
        ]
    }

    /// 技术操作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "气球操控",
            "高度控制",
            "方向控制",
            "着陆技术",
            "安全操作",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "热气球",
            "燃烧器",
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
            "精准度评分",
            "距离评分",
            "高度评分",
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

impl Default for HotAirBalloonRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for HotAirBalloonRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("hot_air_balloon")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【热气球比赛规则】\n\n\
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
    fn test_hot_air_balloon_rules() {
        let rules = HotAirBalloonRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}