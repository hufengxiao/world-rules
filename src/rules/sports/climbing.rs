//! 攀岩规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 攀岩规则
pub struct ClimbingRules {
    metadata: RuleMetadata,
}

impl ClimbingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "攀岩规则",
                "攀岩比赛基本规则"
            )
            .with_origin("欧洲")
            .with_tags(vec!["体育".into(), "极限".into()]),
        }
    }

    /// 比赛项目
    pub fn disciplines(&self) -> Vec<&'static str> {
        vec![
            "速度攀岩: 快速登顶",
            "难度攀岩: 技术攀爬",
            "攀石: 短路线挑战",
            "综合攀岩: 三项综合",
            "自然岩壁攀岩",
        ]
    }

    /// 速度攀岩规则
    pub fn speed_rules(&self) -> Vec<&'static str> {
        vec![
            "标准路线: 15米高度",
            "固定路线",
            "计时制",
            "两人同时比赛",
            "最快登顶获胜",
        ]
    }

    /// 难度攀岩规则
    pub fn lead_rules(&self) -> Vec<&'static str> {
        vec![
            "路线高度: 15米以上",
            "限时6分钟",
            "高度决定成绩",
            "使用保护绳",
            "观察时间",
        ]
    }

    /// 攀石规则
    pub fn bouldering_rules(&self) -> Vec<&'static str> {
        vec![
            "短路线: 4-5米",
            "无绳保护",
            "安全垫",
            "多条路线挑战",
            "尝试次数计入成绩",
        ]
    }

    /// 得分系统
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "速度: 登顶时间",
            "难度: 最高点高度",
            "攀石: 完成路线数",
            "尝试次数影响得分",
            "综合成绩相加",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "攀岩鞋: 特殊鞋底",
            "安全带: 保护装备",
            "快挂: 固定绳索",
            "绳索: 安全保护",
            "攀石垫: 安全保护",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "检查保护系统",
            "正确使用装备",
            "场地安全检查",
            "医疗支持在场",
            "比赛监督",
        ]
    }

    /// 技术等级
    pub fn difficulty_grades(&self) -> Vec<&'static str> {
        vec![
            "国际等级: 1-9级",
            "法国等级: 3a-9c",
            "美国等级: 5.0-5.15",
            "难度递增",
            "选手分级",
        ]
    }
}

impl Default for ClimbingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ClimbingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("climbing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【攀岩规则】\n\n\
            比赛项目:\n{}\n\n\
            得分系统:\n{}\n\n\
            装备要求:\n{}\n\n\
            安全规则:\n{}\n",
            self.disciplines().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climbing_rules() {
        let rules = ClimbingRules::new();
        assert!(!rules.disciplines().is_empty());
    }
}