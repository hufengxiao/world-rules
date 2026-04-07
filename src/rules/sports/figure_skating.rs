//! 花样滑冰规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 花样滑冰规则
pub struct FigureSkatingRules {
    metadata: RuleMetadata,
}

impl FigureSkatingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "花样滑冰规则",
                "花样滑冰比赛基本规则"
            )
            .with_origin("国际滑冰联盟")
            .with_tags(vec!["体育".into(), "冰上".into()]),
        }
    }

    /// 比赛项目
    pub fn competition_categories(&self) -> Vec<&'static str> {
        vec![
            "男子单人滑",
            "女子单人滑",
            "双人滑",
            "冰舞",
            "团体赛",
        ]
    }

    /// 评分系统
    pub fn scoring_system(&self) -> Vec<&'static str> {
        vec![
            "技术分(TES): 动作难度和执行",
            "艺术分(PCS): 表现和编排",
            "总分 = 技术分 + 艺术分",
            "扣分项: 摔倒、时间违规等",
        ]
    }

    /// 跳跃类型
    pub fn jump_types(&self) -> Vec<&'static str> {
        vec![
            "后内结环跳",
            "后外结环跳",
            "后外点冰跳",
            "勾手跳",
            "后内点冰跳",
            "阿克塞尔跳(前起跳)",
            "四周跳: 最高难度",
        ]
    }

    /// 旋转类型
    pub fn spin_types(&self) -> Vec<&'static str> {
        vec![
            "直立旋转",
            "蹲转",
            "弓身旋转",
            "换脚旋转",
            "联合旋转",
            "飞行旋转",
        ]
    }

    /// 步法要求
    pub fn step_sequence(&self) -> Vec<&'static str> {
        vec![
            "必须包含多种转体步法",
            "直线步法序列",
            "圆形步法序列",
            "蛇形步法序列",
            "利用整个冰面",
        ]
    }

    /// 短节目要求
    pub fn short_program(&self) -> Vec<&'static str> {
        vec![
            "时间: 2分40秒上下浮动10秒",
            "必须包含规定动作",
            "动作顺序可调整",
            "单人: 3跳跃、3旋转、步法",
            "双人: 附加抛跳、螺旋线等",
        ]
    }

    /// 自由滑要求
    pub fn free_skating(&self) -> Vec<&'static str> {
        vec![
            "男子: 4分30秒上下浮动10秒",
            "女子: 4分上下浮动10秒",
            "双人: 4分30秒上下浮动10秒",
            "动作数量有限制",
            "后半段跳跃加分10%",
        ]
    }

    /// 扣分规则
    pub fn deductions(&self) -> Vec<&'static str> {
        vec![
            "摔倒: 扣1分(每次)",
            "时间违规: 扣1-2分",
            "服装违规: 扣1分",
            "音乐违规: 扣2分",
            "动作重复: 降级评分",
        ]
    }
}

impl Default for FigureSkatingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FigureSkatingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("figure_skating")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【花样滑冰规则】\n\n\
            比赛项目:\n{}\n\n\
            跳跃类型:\n{}\n\n\
            评分系统:\n{}\n\n\
            短节目要求:\n{}\n",
            self.competition_categories().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.jump_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_system().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.short_program().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_figure_skating_rules() {
        let rules = FigureSkatingRules::new();
        assert!(!rules.competition_categories().is_empty());
        assert!(!rules.jump_types().is_empty());
    }
}