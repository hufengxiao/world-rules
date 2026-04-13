//! 模型飞机比赛规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 模型飞机比赛规则
pub struct ModelAirplaneRules {
    metadata: RuleMetadata,
}

impl ModelAirplaneRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "模型飞机比赛规则",
                "模型飞机比赛规则"
            )
            .with_origin("国际")
            .with_tags(vec!["体育".into(), "航空".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "竞速比赛",
            "花样比赛",
            "精准比赛",
            "耐力比赛",
            "综合比赛",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间限制",
            "飞行路线",
            "出发规则",
            "终点判定",
            "安全规则",
        ]
    }

    /// 技术操作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "遥控操控",
            "飞行技术",
            "花样动作",
            "着陆技术",
            "维修技术",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "模型飞机",
            "遥控设备",
            "动力系统",
            "维修装备",
            "安全装备",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "飞行区域限制",
            "天气条件",
            "安全距离",
            "救援准备",
            "场地安全",
        ]
    }

    /// 评分标准
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "完成时间",
            "花样评分",
            "精准度评分",
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

impl Default for ModelAirplaneRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ModelAirplaneRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("model_airplane")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【模型飞机比赛规则】\n\n\
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
    fn test_model_airplane_rules() {
        let rules = ModelAirplaneRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}