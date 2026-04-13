//! 卡巴迪规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 卡巴迪规则
pub struct KabaddiRules {
    metadata: RuleMetadata,
}

impl KabaddiRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "卡巴迪规则",
                "印度传统团队运动规则"
            )
            .with_origin("印度")
            .with_tags(vec!["体育".into(), "团队".into()]),
        }
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间: 2半场",
            "每半场20分钟",
            "有效时间制",
            "中场休息5分钟",
            "加时赛规则",
        ]
    }

    /// 场地规格
    pub fn court_specifications(&self) -> Vec<&'static str> {
        vec![
            "场地尺寸: 12.5×10米",
            "中线划分",
            "进攻区域",
            "防守区域",
            "边界线",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队7人上场",
            "场上位置",
            "替补队员",
            "换人规则",
            "队员轮换",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "进攻技术",
            "防守技术",
            "触碰技术",
            "呼吸控制",
            "逃脱技术",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "触碰防守队员得分",
            "成功返回加分",
            "防守成功得分",
            "全队出局加分",
            "比分记录",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "超时犯规",
            "边界犯规",
            "非法触碰",
            "违规动作",
            "犯规处罚",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "比赛服装",
            "场地装备",
            "无护具要求",
            "运动鞋",
            "防护装备",
        ]
    }
}

impl Default for KabaddiRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for KabaddiRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("kabaddi")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【卡巴迪规则】\n\n\
            场地规格:\n{}\n\n\
            技术动作:\n{}\n\n\
            得分规则:\n{}\n\n\
            犯规规则:\n{}\n",
            self.court_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kabaddi_rules() {
        let rules = KabaddiRules::new();
        assert!(!rules.competition_rules().is_empty());
    }
}