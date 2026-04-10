//! 空手道规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 空手道规则
pub struct KarateRules {
    metadata: RuleMetadata,
}

impl KarateRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "空手道规则",
                "空手道比赛基本规则"
            )
            .with_origin("日本")
            .with_tags(vec!["体育".into(), "格斗".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "组手: 对打比赛",
            "型: 套路表演",
            "个人赛和团体赛",
            "奥运会正式项目",
            "按体重分级",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "3分: 有效头部踢击或摔技",
            "2分: 中等踢击或中段技法",
            "1分: 基本击打",
            "必须控制力度",
            "击中有效部位得分",
        ]
    }

    /// 有效部位
    pub fn target_areas(&self) -> Vec<&'static str> {
        vec![
            "头部: 面部(有护具)",
            "颈部: 喉部禁止攻击",
            "胸部: 胸腹部",
            "背部: 禁止攻击",
            "下段: 禁止攻击",
        ]
    }

    /// 犯规行为
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "过度接触",
            "攻击禁止部位",
            "危险动作",
            "拖延时间",
            "出界",
            "不尊重对手或裁判",
        ]
    }

    /// 比赛时间
    pub fn match_duration(&self) -> Vec<&'static str> {
        vec![
            "组手比赛: 3分钟",
            "先得8分者获胜",
            "时间结束分高者胜",
            "平局后判定决胜负",
            "决赛可延长比赛时间",
        ]
    }

    /// 体重级别
    pub fn weight_classes(&self) -> Vec<&'static str> {
        vec![
            "男子: -67kg, -75kg, +75kg",
            "女子: -55kg, -61kg, +61kg",
            "赛前称重",
            "体重合格才能参赛",
            "各级别分别比赛",
        ]
    }

    /// 段位制度
    pub fn belt_system(&self) -> Vec<&'static str> {
        vec![
            "白带: 初学者",
            "黄带-黑带: 各级段位",
            "段位由考试晋升",
            "黑带分初段至十段",
            "代表技术和修养水平",
        ]
    }
}

impl Default for KarateRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for KarateRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("karate")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【空手道规则】\n\n\
            比赛类型:\n{}\n\n\
            得分规则:\n{}\n\n\
            犯规行为:\n{}\n\n\
            段位制度:\n{}\n",
            self.competition_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.belt_system().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_karate_rules() {
        let rules = KarateRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}