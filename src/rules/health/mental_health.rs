//! 心理健康规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 心理健康规则
pub struct MentalHealthRules {
    metadata: RuleMetadata,
}

impl MentalHealthRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "心理健康规则",
                "心理健康基础知识"
            )
            .with_origin("心理学")
            .with_tags(vec!["健康".into(), "心理".into()]),
        }
    }

    /// 心理健康标准
    pub fn mental_health_standards(&self) -> Vec<&'static str> {
        vec![
            "能够适应环境变化",
            "能够应对生活压力",
            "能够建立和维持人际关系",
            "能够认识自我和接纳自我",
            "情绪相对稳定",
            "行为符合社会规范",
        ]
    }

    /// 压力管理
    pub fn stress_management(&self) -> Vec<&'static str> {
        vec![
            "识别压力源",
            "学会时间管理",
            "培养兴趣爱好",
            "寻求社会支持",
            "保持规律运动",
            "学习放松技巧",
            "必要时寻求专业帮助",
        ]
    }

    /// 情绪调节
    pub fn emotion_regulation(&self) -> Vec<&'static str> {
        vec![
            "认识和接受情绪",
            "表达而不是压抑情绪",
            "转移注意力",
            "积极自我对话",
            "深呼吸放松",
            "写日记整理情绪",
        ]
    }

    /// 焦虑症状
    pub fn anxiety_symptoms(&self) -> Vec<&'static str> {
        vec![
            "过度担忧",
            "坐立不安",
            "疲劳乏力",
            "注意力难以集中",
            "肌肉紧张",
            "睡眠障碍",
            "心悸出汗",
        ]
    }

    /// 抑郁症状
    pub fn depression_symptoms(&self) -> Vec<&'static str> {
        vec![
            "持续悲伤或空虚感",
            "对活动失去兴趣",
            "食欲和体重变化",
            "睡眠障碍",
            "疲劳或精力不足",
            "自我否定或内疚",
            "注意力下降",
        ]
    }

    /// 保持心理健康
    pub fn maintain_mental_health(&self) -> Vec<&'static str> {
        vec![
            "保持积极心态",
            "培养良好人际关系",
            "设定现实目标",
            "学会说\"不\"",
            "保持工作生活平衡",
            "定期自我反思",
            "寻求帮助是勇敢的表现",
        ]
    }

    /// 心理援助资源
    pub fn mental_health_resources(&self) -> Vec<&'static str> {
        vec![
            "心理咨询热线",
            "社区心理服务中心",
            "医院心理科/精神科",
            "学校心理咨询室",
            "员工心理援助计划(EAP)",
            "线上心理咨询平台",
        ]
    }
}

impl Default for MentalHealthRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MentalHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("mental_health")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【心理健康规则】\n\n\
            心理健康标准:\n{}\n\n\
            压力管理:\n{}\n\n\
            保持心理健康:\n{}\n",
            self.mental_health_standards().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.stress_management().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.maintain_mental_health().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mental_health_rules() {
        let rules = MentalHealthRules::new();
        assert!(!rules.mental_health_standards().is_empty());
    }
}