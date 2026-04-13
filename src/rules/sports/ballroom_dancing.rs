//! 交际舞规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 交际舞规则
pub struct BallroomDancingRules {
    metadata: RuleMetadata,
}

impl BallroomDancingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "交际舞规则",
                "交际舞比赛规则"
            )
            .with_origin("欧洲")
            .with_tags(vec!["体育".into(), "舞蹈".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "标准舞比赛",
            "拉丁舞比赛",
            "十项全能比赛",
            "团体比赛",
            "表演比赛",
        ]
    }

    /// 标准舞项目
    pub fn standard_dances(&self) -> Vec<&'static str> {
        vec![
            "华尔兹",
            "探戈",
            "维也纳华尔兹",
            "狐步舞",
            "快步舞",
        ]
    }

    /// 拉丁舞项目
    pub fn latin_dances(&self) -> Vec<&'static str> {
        vec![
            "桑巴",
            "恰恰恰",
            "伦巴",
            "帕索多ble",
            "牛仔舞",
        ]
    }

    /// 评分标准
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "技术质量",
            "音乐诠释",
            "表演能力",
            "编舞构成",
            "整体表现",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛轮次",
            "服装规定",
            "音乐规则",
            "场地要求",
            "裁判评分",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "比赛服装",
            "舞鞋",
            "音乐选择",
            "场地装备",
            "附属配件",
        ]
    }

    /// 级别体系
    pub fn skill_levels(&self) -> Vec<&'static str> {
        vec![
            "初级级别",
            "中级水平",
            "高级水平",
            "专业级别",
            "教练认证",
        ]
    }
}

impl Default for BallroomDancingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BallroomDancingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("ballroom_dancing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【交际舞规则】\n\n\
            标准舞项目:\n{}\n\n\
            拉丁舞项目:\n{}\n\n\
            评分标准:\n{}\n\n\
            装备要求:\n{}\n",
            self.standard_dances().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.latin_dances().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ballroom_dancing_rules() {
        let rules = BallroomDancingRules::new();
        assert!(!rules.standard_dances().is_empty());
    }
}