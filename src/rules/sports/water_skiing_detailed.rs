//! 花样滑水规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 花样滑水规则
pub struct WaterSkiingDetailedRules {
    metadata: RuleMetadata,
}

impl WaterSkiingDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "花样滑水规则",
                "花样滑水比赛规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "水上".into()]),
        }
    }

    /// 比赛项目
    pub fn competition_events(&self) -> Vec<&'static str> {
        vec![
            "花样滑水",
            "回旋滑水",
            "跳跃滑水",
            "综合比赛",
            "团体比赛",
        ]
    }

    /// 花样动作
    pub fn tricks(&self) -> Vec<&'static str> {
        vec![
            "旋转动作",
            "跳跃动作",
            "翻转动作",
            "花样动作分类",
            "动作难度",
        ]
    }

    /// 评分标准
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "动作难度评分",
            "执行质量",
            "动作数量",
            "时间限制",
            "综合评分",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "滑行技术",
            "控制技术",
            "花样技术",
            "跳跃技术",
            "回旋技术",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "救生衣佩戴",
            "安全区域",
            "船只安全",
            "救援准备",
            "医疗支持",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "滑水板",
            "牵引绳",
            "救生衣",
            "防护装备",
            "船只装备",
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

impl Default for WaterSkiingDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WaterSkiingDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("water_skiing_detailed")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【花样滑水规则】\n\n\
            比赛项目:\n{}\n\n\
            花样动作:\n{}\n\n\
            评分标准:\n{}\n\n\
            装备要求:\n{}\n",
            self.competition_events().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.tricks().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_water_skiing_detailed_rules() {
        let rules = WaterSkiingDetailedRules::new();
        assert!(!rules.competition_events().is_empty());
    }
}