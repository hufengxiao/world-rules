//! 钢架雪车规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 钢架雪车规则
pub struct SkeletonRules {
    metadata: RuleMetadata,
}

impl SkeletonRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "钢架雪车规则",
                "钢架雪车比赛基本规则"
            )
            .with_origin("瑞士")
            .with_tags(vec!["体育".into(), "冬季".into()]),
        }
    }

    /// 比赛项目
    pub fn competition_events(&self) -> Vec<&'static str> {
        vec![
            "男子单人",
            "女子单人",
            "团体赛",
            "世界杯系列",
            "奥运会项目",
        ]
    }

    /// 雪车规格
    pub fn sled_specifications(&self) -> Vec<&'static str> {
        vec![
            "长度: 80-120厘米",
            "高度: 8-20厘米",
            "重量: 男子43公斤，女子35公斤",
            "材质: 钢和塑料",
            "无转向装置",
        ]
    }

    /// 比赛姿势
    pub fn riding_position(&self) -> Vec<&'static str> {
        vec![
            "俯卧姿势",
            "头朝前",
            "下巴贴近雪车",
            "身体紧贴雪车",
            "手脚控制方向",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "滑行4次",
            "总时间最短获胜",
            "起跑助跑",
            "单人操控",
            "身体控制方向",
        ]
    }

    /// 赛道规格
    pub fn track_specifications(&self) -> Vec<&'static str> {
        vec![
            "赛道长度: 1200-1650米",
            "弯道数量: 12-18个",
            "落差: 100-150米",
            "冰面温度控制",
            "安全设施",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "头盔必须佩戴",
            "护具要求",
            "赛道安全检查",
            "天气条件监控",
            "医疗支持",
        ]
    }

    /// 起跑规则
    pub fn start_rules(&self) -> Vec<&'static str> {
        vec![
            "助跑起跑",
            "快速跃上雪车",
            "姿势调整",
            "起跑计时",
            "违规起跑处罚",
        ]
    }

    /// 技术要点
    pub fn technique(&self) -> Vec<&'static str> {
        vec![
            "起跑速度",
            "身体位置控制",
            "弯道技术",
            "脚部微调",
            "终点刹车",
        ]
    }
}

impl Default for SkeletonRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SkeletonRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("skeleton")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【钢架雪车规则】\n\n\
            比赛项目:\n{}\n\n\
            比赛姿势:\n{}\n\n\
            安全规则:\n{}\n\n\
            技术要点:\n{}\n",
            self.competition_events().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.riding_position().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.technique().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skeleton_rules() {
        let rules = SkeletonRules::new();
        assert!(!rules.competition_events().is_empty());
    }
}