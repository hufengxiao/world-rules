//! 花样游泳规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 花样游泳规则
pub struct SynchronizedSwimmingRules {
    metadata: RuleMetadata,
}

impl SynchronizedSwimmingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "花样游泳规则",
                "FINA 花样游泳标准规则"
            )
            .with_origin("FINA")
            .with_tags(vec!["体育".into(), "花样游泳".into()]),
        }
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "单人技术自选",
            "单人自由自选",
            "双人技术自选",
            "双人自由自选",
            "集体技术自选",
            "集体自由自选",
            "自由组合",
        ]
    }

    /// 评分系统
    pub fn scoring_system(&self) -> Vec<&'static str> {
        vec![
            "技术分: 动作执行的准确性",
            "艺术分: 编排、音乐表达",
            "难度分: 动作难度",
            "总分 = 各项分数之和",
        ]
    }

    /// 技术要求
    pub fn technical_requirements(&self) -> Vec<&'static str> {
        vec![
            "规定动作: 必须包含特定动作",
            "时间限制: 根据项目不同",
            "人数要求: 集体项目8人",
            "动作流畅衔接",
        ]
    }

    /// 评分要素
    pub fn scoring_elements(&self) -> Vec<&'static str> {
        vec![
            "队形变化",
            "同步性",
            "难度表现",
            "艺术表现力",
            "水中的位置",
        ]
    }

    /// 扣分因素
    pub fn deductions(&self) -> Vec<&'static str> {
        vec![
            "动作不同步",
            "出界",
            "时间超时/不足",
            "触碰池底",
        ]
    }
}

impl Default for SynchronizedSwimmingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SynchronizedSwimmingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("synchronized_swimming")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【花样游泳规则】\n\n\
            比赛项目:\n{}\n\n\
            评分系统:\n{}\n\n\
            评分要素:\n{}\n",
            self.events().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_system().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_elements().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synchronized_swimming_rules() {
        let rules = SynchronizedSwimmingRules::new();
        assert!(!rules.events().is_empty());
    }
}