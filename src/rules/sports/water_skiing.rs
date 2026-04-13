//! 滑水规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 滑水规则
pub struct WaterSkiingRules {
    metadata: RuleMetadata,
}

impl WaterSkiingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "滑水规则",
                "滑水比赛基本规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "水上".into()]),
        }
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "回旋滑水",
            "跳跃滑水",
            "花样滑水",
            "三项全能",
            "赤脚滑水",
        ]
    }

    /// 回旋规则
    pub fn slalom_rules(&self) -> Vec<&'static str> {
        vec![
            "穿越浮标",
            "速度递增",
            "绳索缩短",
            "浮标数量计分",
            "裁判评分",
        ]
    }

    /// 跳跃规则
    pub fn jumping_rules(&self) -> Vec<&'static str> {
        vec![
            "跳跃斜坡",
            "距离测量",
            "速度控制",
            "三次跳跃机会",
            "最好成绩",
        ]
    }

    /// 花样规则
    pub fn trick_rules(&self) -> Vec<&'static str> {
        vec![
            "花样动作表演",
            "动作难度系数",
            "两轮比赛",
            "计时限制",
            "总分计算",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "滑水板",
            "滑水绳索",
            "滑水靴",
            "救生衣",
            "头盔可选",
        ]
    }

    /// 赛道规格
    pub fn course_specifications(&self) -> Vec<&'static str> {
        vec![
            "回旋赛道: 浮标排列",
            "跳跃斜坡规格",
            "水域宽阔",
            "船只速度控制",
            "安全区域",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "必须佩戴救生衣",
            "船只安全检查",
            "驾驶员培训",
            "医疗支持",
            "天气条件监控",
        ]
    }

    /// 比赛等级
    pub fn competition_levels(&self) -> Vec<&'static str> {
        vec![
            "业余级别",
            "专业级别",
            "世界锦标赛",
            "世界杯",
            "奥运会表演项目",
        ]
    }
}

impl Default for WaterSkiingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WaterSkiingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("water_skiing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【滑水规则】\n\n\
            比赛形式:\n{}\n\n\
            回旋规则:\n{}\n\n\
            安全规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.competition_formats().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.slalom_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_water_skiing_rules() {
        let rules = WaterSkiingRules::new();
        assert!(!rules.competition_formats().is_empty());
    }
}