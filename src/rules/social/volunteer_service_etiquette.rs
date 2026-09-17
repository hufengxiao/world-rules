//! 志愿服务礼仪
//!
//! 公益活动志愿服务中的守时、周到与协作礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: VolunteerServiceEtiquetteRules,
    name: "志愿服务礼仪",
    desc: "公益活动志愿服务中的守时、周到与协作礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "志愿", "服务", "公益"]
}

impl VolunteerServiceEtiquetteRules {
    /// 守时尽责
    pub fn punctuality(&self) -> Vec<&'static str> {
        vec![
            "按约定时间到岗",
            "请假提前告知负责人",
            "忠实履行交代的任务",
            "不半途随意退出",
        ]
    }

    /// 服务周到
    pub fn service(&self) -> Vec<&'static str> {
        vec![
            "以受助者需求为重",
            "态度温和耐心引导",
            "说明清楚所提供帮助",
            "不越权承诺无法兑现",
        ]
    }

    /// 团队协作
    pub fn team(&self) -> Vec<&'static str> {
        vec![
            "服从分工互相补位",
            "主动沟通协调安排",
            "尊重工作人员与其他志愿者",
            "合理承担不过度揽活",
        ]
    }

    /// 职业操守
    pub fn ethics(&self) -> Vec<&'static str> {
        vec![
            "保护受助者隐私",
            "不收取或索取报酬",
            "如实反馈志愿服务情况",
            "着装得体树立良好形象",
        ]
    }
}

impl Rule for VolunteerServiceEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("volunteer_service")
    }

    fn explain(&self) -> String {
        format!(
            "【志愿服务礼仪】\n{}",
            [
                format!(
                    "守时尽责：\\n{}",
                    self.punctuality()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "服务周到：\\n{}",
                    self.service()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "团队协作：\\n{}",
                    self.team()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "职业操守：\\n{}",
                    self.ethics()
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
    fn test_volunteerserviceetiquetterules_basic() {
        let rules = VolunteerServiceEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "志愿服务礼仪");
        assert!(!rules.punctuality().is_empty());
        assert!(!rules.service().is_empty());
        assert!(!rules.team().is_empty());
        assert!(!rules.ethics().is_empty());
    }

    #[test]
    fn test_volunteerserviceetiquetterules_validation() {
        let rules = VolunteerServiceEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("volunteer_service"));
    }

    #[test]
    fn test_volunteerserviceetiquetterules_explain() {
        let rules = VolunteerServiceEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("守时尽责"));
        assert!(e.contains("服务周到"));
        assert!(e.contains("团队协作"));
    }
}
