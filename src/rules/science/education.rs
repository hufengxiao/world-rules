//! 教育学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 教育学定律集合
pub struct EducationLaws {
    metadata: RuleMetadata,
}

impl EducationLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "教育学定律",
                "教育学基本定律"
            )
            .with_origin("社会科学")
            .with_tags(vec!["科学".into(), "教育".into()]),
        }
    }

    /// 教育规律定律
    pub fn education_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("教育适应定律", "适应发展", "教育适应社会发展"),
            ("教育促进定律", "促进发展", "教育促进社会发展"),
            ("教育规律定律", "内在规律", "教育内在规律"),
            ("教育目的定律", "目标导向", "教育目的引导"),
            ("教育内容定律", "内容选择", "教育内容选择"),
            ("教育方法定律", "方法多样", "教育方法多样性"),
            ("教育评价定律", "评价多元", "教育评价多元"),
        ]
    }

    /// 教学定律
    pub fn teaching_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("教学规律定律", "双边活动", "教学双边规律"),
            ("间接经验定律", "间接学习", "间接经验学习"),
            ("直接经验定律", "实践体验", "直接经验重要性"),
            ("掌握知识定律", "知识技能", "知识与技能统一"),
            ("智力发展定律", "智力培养", "智力发展规律"),
            ("教学原则定律", "原则指导", "教学原则系统"),
            ("启发定律", "启发引导", "启发式教学"),
        ]
    }

    /// 学习定律
    pub fn learning_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("学习动机定律", "动机驱动", "学习动机重要性"),
            ("学习兴趣定律", "兴趣引导", "兴趣促进学习"),
            ("学习迁移定律", "迁移应用", "学习迁移规律"),
            ("学习记忆定律", "记忆保持", "记忆保持规律"),
            ("学习遗忘定律", "遗忘曲线", "遗忘规律"),
            ("学习练习定律", "练习巩固", "练习巩固规律"),
            ("学习反馈定律", "反馈强化", "反馈强化学习"),
        ]
    }

    /// 德育定律
    pub fn moral_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("德育规律定律", "品德形成", "品德形成规律"),
            ("知行统一定律", "言行一致", "知行统一要求"),
            ("榜样定律", "榜样示范", "榜样示范作用"),
            ("环境定律", "环境影响", "环境熏陶作用"),
            ("活动定律", "活动育人", "活动育人规律"),
            ("自我教育定律", "自我发展", "自我教育能力"),
        ]
    }

    /// 教育制度
    pub fn systems(&self) -> Vec<&'static str> {
        vec![
            "学校教育",
            "家庭教育",
            "社会教育",
            "终身教育",
            "职业教育",
            "高等教育",
            "基础教育",
            "特殊教育",
        ]
    }

    /// 教育方法
    pub fn methods(&self) -> Vec<&'static str> {
        vec![
            "讲授法",
            "讨论法",
            "实验法",
            "练习法",
            "参观法",
            "自学法",
            "合作学习",
            "探究学习",
        ]
    }
}

impl Default for EducationLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for EducationLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("education")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【教育学定律】\n\n教育规律:\n{}\n\n教学定律:\n{}\n\n学习定律:\n{}\n",
            self.education_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.teaching_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.learning_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_education_laws() {
        let laws = EducationLaws::new();
        assert!(!laws.education_laws().is_empty());
        assert!(!laws.teaching_laws().is_empty());
    }
}