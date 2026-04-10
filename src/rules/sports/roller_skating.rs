//! 轮滑规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 轮滑规则
pub struct RollerSkatingRules {
    metadata: RuleMetadata,
}

impl RollerSkatingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "轮滑规则",
                "轮滑比赛基本规则"
            )
            .with_origin("欧洲")
            .with_tags(vec!["体育".into(), "极限".into()]),
        }
    }

    /// 比赛项目
    pub fn disciplines(&self) -> Vec<&'static str> {
        vec![
            "速度轮滑: 竞速比赛",
            "花样轮滑: 技术表演",
            "轮滑球: 团队比赛",
            "自由式轮滑: 技术挑战",
            "轮滑越野: 户外比赛",
        ]
    }

    /// 速度轮滑规则
    pub fn speed_rules(&self) -> Vec<&'static str> {
        vec![
            "场地赛: 200米跑道",
            "公路赛: 长距离",
            "起跑方式: 集体出发",
            "计时制",
            "分组比赛",
        ]
    }

    /// 花样轮滑规则
    pub fn figure_rules(&self) -> Vec<&'static str> {
        vec![
            "规定动作",
            "自由滑行",
            "评分系统0-10分",
            "音乐伴奏",
            "技术难度评分",
        ]
    }

    /// 轮滑球规则
    pub fn hockey_rules(&self) -> Vec<&'static str> {
        vec![
            "每队5人(含守门员)",
            "比赛时间: 两节各20分钟",
            "球门: 1.05米×1.70米",
            "击球入网得分",
            "犯规处罚",
        ]
    }

    /// 自由式轮滑规则
    pub fn freestyle_rules(&self) -> Vec<&'static str> {
        vec![
            "绕桩比赛",
            "速度过桩",
            "花式绕桩",
            "跳高比赛",
            "技术评分",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "轮滑鞋: 不同类型",
            "轮子: 不同硬度",
            "头盔: 安全保护",
            "护具: 膝盖肘部",
            "手套: 保护手部",
        ]
    }

    /// 技术等级
    pub fn skill_levels(&self) -> Vec<&'static str> {
        vec![
            "初级: 基本滑行",
            "中级: 技术动作",
            "高级: 高难度动作",
            "专业: 竞技水平",
            "分级考试",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "护具必须佩戴",
            "场地安全检查",
            "控制速度",
            "避免碰撞",
            "医疗支持",
        ]
    }
}

impl Default for RollerSkatingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for RollerSkatingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("roller_skating")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【轮滑规则】\n\n\
            比赛项目:\n{}\n\n\
            速度轮滑规则:\n{}\n\n\
            装备要求:\n{}\n\n\
            安全规则:\n{}\n",
            self.disciplines().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.speed_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roller_skating_rules() {
        let rules = RollerSkatingRules::new();
        assert!(!rules.disciplines().is_empty());
    }
}