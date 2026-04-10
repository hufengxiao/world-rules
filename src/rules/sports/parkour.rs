//! 跑酷规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 跑酷规则
pub struct ParkourRules {
    metadata: RuleMetadata,
}

impl ParkourRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "跑酷规则",
                "跑酷比赛基本规则"
            )
            .with_origin("法国")
            .with_tags(vec!["体育".into(), "极限".into()]),
        }
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "速度赛: 最快完成路线",
            "自由式: 技术表演",
            "技巧赛: 难度挑战",
            "个人赛",
            "团队接力赛",
        ]
    }

    /// 基本动作
    pub fn basic_movements(&self) -> Vec<&'static str> {
        vec![
            "落地滚翻: 安全着陆",
            "精准跳跃: 精确落地",
            "猫跳: 双手支撑",
            "墙跑: 垂直墙面",
            "翻越障碍: 快速过障",
        ]
    }

    /// 高级技巧
    pub fn advanced_techniques(&self) -> Vec<&'static str> {
        vec![
            "后空翻",
            "前空翻",
            "侧翻",
            "360度旋转",
            "组合动作",
        ]
    }

    /// 得分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "速度: 完成时间",
            "技术: 动作难度",
            "流畅度: 连续性",
            "创新: 新动作",
            "安全: 无受伤",
        ]
    }

    /// 路线设计
    pub fn course_design(&self) -> Vec<&'static str> {
        vec![
            "多种障碍物",
            "不同高度",
            "技术挑战",
            "安全设计",
            "观赏性考量",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "熟悉场地",
            "热身准备",
            "能力范围内动作",
            "防护装备",
            "医疗支持",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "运动鞋: 抓地力强",
            "舒适服装",
            "护膝护肘",
            "手套可选",
            "轻便装备",
        ]
    }

    /// 禁止行为
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "危险动作",
            "破坏场地",
            "干扰他人",
            "未经许可进入",
            "无保护高难度动作",
        ]
    }
}

impl Default for ParkourRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ParkourRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("parkour")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【跑酷规则】\n\n\
            基本动作:\n{}\n\n\
            得分标准:\n{}\n\n\
            安全规则:\n{}\n\n\
            禁止行为:\n{}\n",
            self.basic_movements().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_criteria().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.prohibited_actions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parkour_rules() {
        let rules = ParkourRules::new();
        assert!(!rules.basic_movements().is_empty());
    }
}