//! 刑法基础知识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 刑法规则
pub struct CriminalLawRules {
    metadata: RuleMetadata,
}

impl CriminalLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "刑法规则",
                "中国刑法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "刑法".into()]),
        }
    }

    /// 犯罪构成要件
    pub fn crime_elements(&self) -> Vec<&'static str> {
        vec![
            "犯罪客体: 被侵害的合法权益",
            "犯罪客观方面: 危害行为和结果",
            "犯罪主体: 实施犯罪的人",
            "犯罪主观方面: 故意或过失",
            "四要件必须同时具备",
        ]
    }

    /// 刑罚种类
    pub fn punishment_types(&self) -> Vec<&'static str> {
        vec![
            "主刑: 管制、拘役、有期徒刑、无期徒刑、死刑",
            "附加刑:罚金、剥夺政治权利、没收财产、驱逐出境",
            "主刑只能独立适用",
            "附加刑可独立或附加适用",
        ]
    }

    /// 刑期规定
    pub fn sentence_limits(&self) -> Vec<&'static str> {
        vec![
            "管制: 3个月至2年",
            "拘役: 1个月至6个月",
            "有期徒刑: 6个月至15年",
            "数罪并罚最高25年",
            "无期徒刑: 终身剥夺自由",
        ]
    }

    /// 刑事责任年龄
    pub fn age_of_responsibility(&self) -> Vec<&'static str> {
        vec![
            "完全不负刑事责任: 不满12周岁",
            "相对负刑事责任: 12-14周岁(特定重罪)",
            "相对负刑事责任: 14-16周岁(八种重罪)",
            "完全负刑事责任: 16周岁以上",
            "未成年人犯罪从轻或减轻处罚",
        ]
    }

    /// 正当防卫
    pub fn self_defense(&self) -> Vec<&'static str> {
        vec![
            "为保护合法权益免受不法侵害",
            "针对不法侵害人",
            "在侵害进行时",
            "不超过必要限度",
            "特殊防卫: 针对严重暴力犯罪可无限防卫",
            "防卫过当应负刑事责任但可减轻",
        ]
    }

    /// 紧急避险
    pub fn emergency_evasion(&self) -> Vec<&'static str> {
        vec![
            "为保护较大利益牺牲较小利益",
            "必须在紧急情况下",
            "别无其他选择",
            "不得超过必要限度",
            "避险过当应负刑事责任",
        ]
    }

    /// 自首和立功
    pub fn confession_merit(&self) -> Vec<&'static str> {
        vec![
            "自首: 自动投案如实供述",
            "准自首: 被采取强制措施后供述其他罪行",
            "立功: 提供侦破其他案件线索",
            "重大立功: 提供重大案件线索",
            "自首可从轻或减轻处罚",
            "立功可从轻或减轻处罚",
        ]
    }
}

impl Default for CriminalLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CriminalLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("criminal")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【刑法规则】\n\n\
            犯罪构成要件:\n{}\n\n\
            刑罚种类:\n{}\n\n\
            刑事责任年龄:\n{}\n\n\
            正当防卫:\n{}\n",
            self.crime_elements().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.punishment_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.age_of_responsibility().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.self_defense().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_criminal_law_rules() {
        let rules = CriminalLawRules::new();
        assert!(!rules.crime_elements().is_empty());
    }
}