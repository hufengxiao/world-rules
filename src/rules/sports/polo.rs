//! 马球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 马球规则 (Polo)
pub struct PoloRules {
    metadata: RuleMetadata,
}

impl PoloRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "马球规则",
                "马球比赛规则"
            )
            .with_origin("波斯")
            .with_tags(vec!["体育".into(), "马术".into()]),
        }
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间: 4-6节",
            "每节7分钟",
            "中场休息3分钟",
            "加时规则",
            "比赛结束",
        ]
    }

    /// 场地规格
    pub fn field_specifications(&self) -> Vec<&'static str> {
        vec![
            "场地尺寸: 274×146米",
            "球门宽度: 8米",
            "场地边界",
            "安全区域",
            "场地布置",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队4人上场",
            "场上位置",
            "马匹配置",
            "替补队员",
            "换人规则",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "击球技术",
            "骑马技术",
            "传球技术",
            "防守技术",
            "战术运用",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "危险骑乘",
            "非法阻挡",
            "犯规处罚",
            "罚球规则",
            "安全犯规",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "球穿过球门得分",
            "有效进球",
            "得分统计",
            "比分记录",
            "胜负判定",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "马球杆",
            "马球",
            "骑马装备",
            "防护装备",
            "比赛服装",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "马匹安全",
            "骑手安全",
            "防护装备",
            "医疗支持",
            "场地安全",
        ]
    }
}

impl Default for PoloRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PoloRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("polo")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【马球规则】\n\n\
            场地规格:\n{}\n\n\
            技术动作:\n{}\n\n\
            得分规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.field_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
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
    fn test_polo_rules() {
        let rules = PoloRules::new();
        assert!(!rules.competition_rules().is_empty());
    }
}