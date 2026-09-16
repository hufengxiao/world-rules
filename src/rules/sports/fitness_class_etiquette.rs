//! 团操课程礼仪
//!
//! 健身房团课、跳操等集体课程的空间与礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FitnessClassEtiquetteRules,
    name: "团操课程礼仪",
    desc: "健身房团课、跳操等集体课程的空间与礼仪",
    origin: "大众",
    tags: ["体育", "健身", "团课", "操课", "礼仪"]
}

impl FitnessClassEtiquetteRules {
    /// 课前准备
    pub fn preparation(&self) -> Vec<&'static str> {
        vec![
            "提前到场找好位置",
            "自带毛巾与水杯",
            "短时热身后进课",
            "穿着合体的运动装备",
        ]
    }

    /// 课中行为
    pub fn during(&self) -> Vec<&'static str> {
        vec![
            "紧跟教练节奏练习",
            "难度不适时量力调整",
            "不多说话打扰邻座",
            "注意与相邻学员保持间距",
        ]
    }

    /// 空间与器具
    pub fn space(&self) -> Vec<&'static str> {
        vec![
            "器材轮流使用不独占",
            "用完归位不挡通道",
            "舞蹈区避免碰撞他人",
            "共同维护教室整洁",
        ]
    }

    /// 课后
    pub fn after(&self) -> Vec<&'static str> {
        vec![
            "放松拉伸缓解酸痛",
            "放回器材与毛巾",
            "向教练道谢",
            "合理补水与休息",
        ]
    }
}

impl Rule for FitnessClassEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("fitness_class")
    }

    fn explain(&self) -> String {
        format!(
            "【团操课程礼仪】\n{}",
            [
                format!(
                    "课前准备：\\n{}",
                    self.preparation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "课中行为：\\n{}",
                    self.during()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "空间与器具：\\n{}",
                    self.space()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "课后：\\n{}",
                    self.after()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
            ]
            .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_fitnessclassetiquetterules_basic() {
        let rules = FitnessClassEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "团操课程礼仪");
        assert!(!rules.preparation().is_empty());
        assert!(!rules.during().is_empty());
        assert!(!rules.space().is_empty());
        assert!(!rules.after().is_empty());
    }

    #[test]
    fn test_fitnessclassetiquetterules_validation() {
        let rules = FitnessClassEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("fitness_class"));
    }

    #[test]
    fn test_fitnessclassetiquetterules_explain() {
        let rules = FitnessClassEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("课前准备"));
        assert!(e.contains("课中行为"));
        assert!(e.contains("空间与器具"));
    }
}
