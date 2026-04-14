//! 自由潜水规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 自由潜水规则
pub struct FreeDivingRules {
    metadata: RuleMetadata,
}

impl FreeDivingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "自由潜水规则",
                "自由潜水比赛规则"
            )
            .with_origin("法国")
            .with_tags(vec!["体育".into(), "水上".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "恒重潜水",
            "无限制潜水",
            "静态闭气",
            "动态潜水",
            "变重潜水",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "深度记录",
            "时间记录",
            "安全规则",
            "裁判监督",
            "比赛结束",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "闭气技术",
            "潜水技术",
            "压力控制",
            "呼吸控制",
            "安全技术",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "安全潜水员必须",
            "医疗支持",
            "减压规则",
            "救援准备",
            "健康检查",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "潜水装备",
            "潜水服",
            "安全装备",
            "计时设备",
            "深度计",
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

    /// 记录规则
    pub fn records(&self) -> Vec<&'static str> {
        vec![
            "世界记录",
            "国家记录",
            "个人记录",
            "认证规则",
            "记录标准",
        ]
    }
}

impl Default for FreeDivingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FreeDivingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("free_diving")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【自由潜水规则】\n\n\
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
    fn test_free_diving_rules() {
        let rules = FreeDivingRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}