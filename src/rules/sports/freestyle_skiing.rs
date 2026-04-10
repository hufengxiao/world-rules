//! 自由式滑雪规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 自由式滑雪规则
pub struct FreestyleSkiingRules {
    metadata: RuleMetadata,
}

impl FreestyleSkiingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "自由式滑雪规则",
                "自由式滑雪比赛基本规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "冬季".into()]),
        }
    }

    /// 比赛项目
    pub fn competition_events(&self) -> Vec<&'static str> {
        vec![
            "雪上技巧: 障碍滑行",
            "空中技巧: 跳跃表演",
            "障碍追逐: 竞速比赛",
            "U型场地: 技术表演",
            "坡面障碍: 障碍技术",
        ]
    }

    /// 雪上技巧规则
    pub fn moguls_rules(&self) -> Vec<&'static str> {
        vec![
            "斜坡障碍滑行",
            "跳跃动作",
            "速度评分",
            "技术评分",
            "综合得分",
        ]
    }

    /// 空中技巧规则
    pub fn aerials_rules(&self) -> Vec<&'static str> {
        vec![
            "跳台跳跃",
            "空中动作",
            "着陆质量",
            "难度系数",
            "姿态评分",
        ]
    }

    /// 评分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "难度: 动作难度",
            "执行: 完成质量",
            "幅度: 飞行高度",
            "流畅度: 连续性",
            "创新: 新技术",
        ]
    }

    /// 技术动作
    pub fn tricks(&self) -> Vec<&'static str> {
        vec![
            "旋转: 多周旋转",
            "翻转: 空中翻转",
            "抓板: 各种抓板",
            "组合动作",
            "着陆技术",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "滑雪板: 双板",
            "滑雪靴: 固定靴",
            "头盔: 安全保护",
            "护具: 保护装备",
            "滑雪服: 气动设计",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "头盔必须佩戴",
            "场地安全检查",
            "天气条件监控",
            "医疗支持",
            "训练要求",
        ]
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "预赛和决赛",
            "淘汰赛制",
            "世界杯系列",
            "世界锦标赛",
            "奥运会项目",
        ]
    }
}

impl Default for FreestyleSkiingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FreestyleSkiingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("freestyle_skiing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【自由式滑雪规则】\n\n\
            比赛项目:\n{}\n\n\
            评分标准:\n{}\n\n\
            技术动作:\n{}\n\n\
            安全规则:\n{}\n",
            self.competition_events().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_criteria().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.tricks().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freestyle_skiing_rules() {
        let rules = FreestyleSkiingRules::new();
        assert!(!rules.competition_events().is_empty());
    }
}