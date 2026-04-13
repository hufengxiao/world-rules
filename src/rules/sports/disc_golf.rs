//! 地板高尔夫规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 地板高尔夫规则 (Disc Golf)
pub struct DiscGolfRules {
    metadata: RuleMetadata,
}

impl DiscGolfRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "地板高尔夫规则",
                "飞盘高尔夫比赛规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "休闲".into()]),
        }
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛采用杆数制",
            "18洞比赛",
            "最少杆数获胜",
            "比赛时间",
            "比赛结束",
        ]
    }

    /// 场地规格
    pub fn course_specifications(&self) -> Vec<&'static str> {
        vec![
            "洞数: 9洞或18洞",
            "目标筐尺寸",
            "场地布局",
            "障碍设置",
            "场地类型",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "投掷技术",
            "瞄准技术",
            "距离控制",
            "精准投掷",
            "策略运用",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "每洞杆数",
            "总杆数计算",
            "标准杆数",
            "罚杆规则",
            "比分记录",
        ]
    }

    /// 飞盘类型
    pub fn disc_types(&self) -> Vec<&'static str> {
        vec![
            "距离盘",
            "精准盘",
            "中等距离盘",
            "飞盘选择",
            "规格要求",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "飞盘",
            "目标筐",
            "场地装备",
            "比赛服装",
            "计分板",
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

impl Default for DiscGolfRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for DiscGolfRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("disc_golf")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【地板高尔夫规则】\n\n\
            场地规格:\n{}\n\n\
            技术动作:\n{}\n\n\
            得分规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.course_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disc_golf_rules() {
        let rules = DiscGolfRules::new();
        assert!(!rules.competition_rules().is_empty());
    }
}