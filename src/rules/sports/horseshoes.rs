//! 马蹄铁规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 马蹄铁规则 (Horseshoes)
pub struct HorseshoesRules {
    metadata: RuleMetadata,
}

impl HorseshoesRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "马蹄铁规则",
                "马蹄铁投掷运动规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "休闲".into()]),
        }
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛采用局数制",
            "每局两个马蹄铁",
            "轮流投掷",
            "得分规则",
            "比赛结束",
        ]
    }

    /// 场地规格
    pub fn court_specifications(&self) -> Vec<&'static str> {
        vec![
            "场地距离: 40英尺",
            "铁柱高度: 15英寸",
            "铁柱直径: 1英寸",
            "投掷区域",
            "场地布置",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "投掷技术",
            "瞄准技术",
            "旋转技术",
            "控制技术",
            "节奏控制",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "环绕铁柱: 3分",
            "靠近铁柱: 1分",
            "最近者得分",
            "取消规则",
            "比分记录",
        ]
    }

    /// 马蹄铁规格
    pub fn horseshoe_specifications(&self) -> Vec<&'static str> {
        vec![
            "重量限制: 2.5磅",
            "尺寸规定",
            "开口尺寸",
            "材质要求",
            "规格认证",
        ]
    }

    /// 团队配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "单人比赛",
            "双人比赛",
            "队员轮换",
            "比赛顺序",
            "投掷顺序",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "马蹄铁",
            "铁柱",
            "场地装备",
            "计分板",
            "比赛服装",
        ]
    }
}

impl Default for HorseshoesRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for HorseshoesRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("horseshoes")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【马蹄铁规则】\n\n\
            场地规格:\n{}\n\n\
            技术动作:\n{}\n\n\
            得分规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.court_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
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
    fn test_horseshoes_rules() {
        let rules = HorseshoesRules::new();
        assert!(!rules.competition_rules().is_empty());
    }
}