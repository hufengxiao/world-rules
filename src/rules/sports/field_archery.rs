//! 野外射箭规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 野外射箭规则
pub struct FieldArcheryRules {
    metadata: RuleMetadata,
}

impl FieldArcheryRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "野外射箭规则",
                "野外射箭比赛基本规则"
            )
            .with_origin("国际")
            .with_tags(vec!["体育".into(), "射箭".into()]),
        }
    }

    /// 比赛距离
    pub fn distances(&self) -> Vec<&'static str> {
        vec![
            "距离变化: 5-60米",
            "标记靶",
            "未标记靶",
            "野外环境",
            "地形变化",
        ]
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "标记靶比赛",
            "未标记靶比赛",
            "个人赛",
            "团体赛",
            "野外锦标赛",
        ]
    }

    /// 靶面规格
    pub fn target_specifications(&self) -> Vec<&'static str> {
        vec![
            "野外靶设计",
            "颜色区域",
            "得分区",
            "靶面更换",
            "角度变化",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "最高6分",
            "区域得分",
            "总成绩计算",
            "同分判定",
            "比赛轮次",
        ]
    }

    /// 环境规则
    pub fn environment_rules(&self) -> Vec<&'static str> {
        vec![
            "地形适应",
            "光线变化",
            "风力影响",
            "天气条件",
            "场地安全",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "弓具标准",
            "箭具标准",
            "野外适用",
            "服装适应",
            "安全装备",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "安全行走",
            "靶后区域",
            "统一指挥",
            "医疗支持",
            "天气预警",
        ]
    }

    /// 比赛路线
    pub fn course_design(&self) -> Vec<&'static str> {
        vec![
            "24靶标",
            "路线设计",
            "上坡下坡",
            "距离变化",
            "安全路径",
        ]
    }
}

impl Default for FieldArcheryRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FieldArcheryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("field_archery")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【野外射箭规则】\n\n\
            比赛距离:\n{}\n\n\
            得分规则:\n{}\n\n\
            安全规则:\n{}\n\n\
            比赛路线:\n{}\n",
            self.distances().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.course_design().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_archery_rules() {
        let rules = FieldArcheryRules::new();
        assert!(!rules.distances().is_empty());
    }
}