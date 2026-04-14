//! 定向射箭规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 定向射箭规则 (Mounted Archery)
pub struct MountedArcheryRules {
    metadata: RuleMetadata,
}

impl MountedArcheryRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "定向射箭规则",
                "骑马射箭运动规则"
            )
            .with_origin("蒙古")
            .with_tags(vec!["体育".into(), "马术".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "传统骑射",
            "现代骑射",
            "速度骑射",
            "精准骑射",
            "综合比赛",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "赛道长度规定",
            "靶标数量",
            "射箭时机",
            "时间限制",
            "安全规则",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "骑马技术",
            "射箭技术",
            "协调配合",
            "节奏控制",
            "稳定技术",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "射箭弓",
            "箭矢",
            "骑马装备",
            "防护装备",
            "靶标系统",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "马匹安全",
            "射箭安全",
            "防护装备",
            "医疗支持",
            "场地安全",
        ]
    }

    /// 评分标准
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "命中率评分",
            "速度评分",
            "综合评分",
            "排名规则",
            "分数计算",
        ]
    }

    /// 级别体系
    pub fn skill_levels(&self) -> Vec<&'static str> {
        vec![
            "初级骑射",
            "中级水平",
            "高级水平",
            "专业级别",
            "大师级别",
        ]
    }
}

impl Default for MountedArcheryRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MountedArcheryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("mounted_archery")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【定向射箭规则】\n\n\
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
    fn test_mounted_archery_rules() {
        let rules = MountedArcheryRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}