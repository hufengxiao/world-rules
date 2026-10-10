//! 白内障术后护理
//!
//! 白内障手术后的眼部护理与康复规则，包括用眼卫生、滴眼药规范、避免外力与复查安排，保障术后视力恢复。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CataractPostOpCareRules,
    name: "白内障术后护理",
    desc: "白内障手术后的眼部护理与康复规则，包括用眼卫生、滴眼药规范、避免外力与复查安排，保障术后视力恢复。",
    origin: "眼科术后护理与康复指导",
    tags: ["健康", "眼科", "术后", "护理"]
}

impl CataractPostOpCareRules {
    /// 用眼卫生
    pub fn hygiene(&self) -> Vec<&'static str> {
        vec![
            "术后勿揉眼按压眼球",
            "洗头洗脸避免水入眼",
            "外出戴防护镜遮光",
            "保持眼部清洁",
        ]
    }

    /// 用药规范
    pub fn medication(&self) -> Vec<&'static str> {
        vec![
            "按时按量滴眼药水",
            "滴药前洗净双手",
            "多种眼药间隔数分钟",
            "遵医嘱不自行停药",
        ]
    }

    /// 活动限制
    pub fn activity(&self) -> Vec<&'static str> {
        vec![
            "避免剧烈运动与弯腰用力",
            "不提重物防眼压升高",
            "睡觉勿压患侧眼",
            "咳嗽便秘须及时缓解",
        ]
    }

    /// 复查与预警
    pub fn medical(&self) -> Vec<&'static str> {
        vec![
            "按医嘱定期复查视力",
            "眼痛视力骤降立即就医",
            "眼红分泌物增多须复诊",
            "按时取出或调整眼罩",
        ]
    }
}

impl Rule for CataractPostOpCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("cataract")
    }

    fn explain(&self) -> String {
        format!(
            "【白内障术后护理】\n{}",
            [
                format!(
                    "用眼卫生：\\n{}",
                    self.hygiene()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "用药规范：\\n{}",
                    self.medication()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "活动限制：\\n{}",
                    self.activity()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "复查与预警：\\n{}",
                    self.medical()
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
    fn test_cataractpostopcarerules_basic() {
        let rules = CataractPostOpCareRules::new();
        assert_eq!(rules.metadata().name, "白内障术后护理");
        assert!(!rules.hygiene().is_empty());
        assert!(!rules.medication().is_empty());
        assert!(!rules.activity().is_empty());
        assert!(!rules.medical().is_empty());
    }

    #[test]
    fn test_cataractpostopcarerules_validation() {
        let rules = CataractPostOpCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("cataract"));
    }

    #[test]
    fn test_cataractpostopcarerules_explain() {
        let rules = CataractPostOpCareRules::new();
        let e = rules.explain();
        assert!(e.contains("用眼卫生"));
        assert!(e.contains("用药规范"));
        assert!(e.contains("活动限制"));
    }
}
