//! 婚姻法规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 婚姻法规则
pub struct MarriageLawRules {
    metadata: RuleMetadata,
}

impl MarriageLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "婚姻法规则",
                "中国婚姻法基本规则"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "婚姻".into()]),
        }
    }

    /// 结婚条件
    pub fn marriage_conditions(&self) -> Vec<&'static str> {
        vec![
            "男不得早于22周岁",
            "女不得早于20周岁",
            "双方完全自愿",
            "无配偶",
            "非直系血亲和三代以内旁系血亲",
        ]
    }

    /// 禁止结婚的情形
    pub fn prohibited_marriages(&self) -> Vec<&'static str> {
        vec![
            "直系血亲",
            "三代以内旁系血亲",
            "患有医学上认为不应当结婚的疾病",
        ]
    }

    /// 夫妻财产
    pub fn property_rules(&self) -> Vec<&'static str> {
        vec![
            "共同财产: 工资、奖金、经营收益等",
            "个人财产: 婚前财产、人身损害赔偿等",
            "约定财产制: 可书面约定财产归属",
        ]
    }

    /// 离婚方式
    pub fn divorce_methods(&self) -> Vec<&'static str> {
        vec![
            "协议离婚: 双方自愿，签订离婚协议",
            "诉讼离婚: 一方要求，法院判决",
            "冷静期: 协议离婚需30天冷静期",
        ]
    }

    /// 离婚条件(诉讼)
    pub fn divorce_conditions(&self) -> Vec<&'static str> {
        vec![
            "感情确已破裂",
            "分居满两年",
            "一方有重大过错",
            "调解无效",
        ]
    }

    /// 子女抚养
    pub fn child_custody(&self) -> Vec<&'static str> {
        vec![
            "两岁以下: 原则上归母亲",
            "两岁以上: 有利于子女成长原则",
            "八岁以上: 尊重子女意愿",
            "非抚养方有探望权",
        ]
    }

    /// 离婚财产分割
    pub fn property_division(&self) -> Vec<&'static str> {
        vec![
            "共同财产: 协议分割或法院判决",
            "照顾子女和女方权益",
            "过错方可能少分财产",
            "个人财产归个人",
        ]
    }
}

impl Default for MarriageLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MarriageLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("marriage")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【婚姻法规则】\n\n\
            结婚条件:\n{}\n\n\
            夫妻财产:\n{}\n\n\
            离婚方式:\n{}\n\n\
            子女抚养:\n{}\n",
            self.marriage_conditions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.property_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.divorce_methods().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.child_custody().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marriage_law_rules() {
        let rules = MarriageLawRules::new();
        assert!(!rules.marriage_conditions().is_empty());
    }
}