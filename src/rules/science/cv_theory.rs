//! 计算机视觉理论

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: CvTheoryRules,
    name: "计算机视觉理论",
    desc: "计算机视觉理论定律",
    origin: "国际",
    tags: ["科学", "计算机"]
}

impl CvTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["卷积特征提取", "目标检测", "图像分割"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["人脸识别", "自动驾驶视觉", "医学影像"]
    }
}

impl Rule for CvTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("cv_theory")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "计算机视觉理论",
            &[("基础", &self.section_0()), ("应用", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cv_theory_rules() {
        let r = CvTheoryRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
