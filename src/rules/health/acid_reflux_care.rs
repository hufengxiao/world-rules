//! 胃酸反流管理
//!
//! 烧心反酸的诱因、缓解与生活调理规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: AcidRefluxCareRules,
    name: "胃酸反流管理",
    desc: "烧心反酸的诱因、缓解与生活调理规则",
    origin: "医学",
    tags: ["健康", "胃酸", "反流", "烧心"]
}

impl AcidRefluxCareRules {
    /// 认识症状
    pub fn symptoms(&self) -> Vec<&'static str> {
        vec![
            "烧心胸骨后灼热",
            "反酸口泛酸",
            "进食或躺下加重",
            "咽异物感需留意",
        ]
    }

    /// 饮食调理
    pub fn diet(&self) -> Vec<&'static str> {
        vec![
            "少食多餐不暴食",
            "少油腻辛辣咖啡",
            "饭后不立即躺卧",
            "睡前避免进食",
        ]
    }

    /// 生活习惯
    pub fn lifestyle(&self) -> Vec<&'static str> {
        vec![
            "保持体重不肥胖",
            "减少过量饮酒",
            "戒烟少刺激",
            "衣着宽松减压",
        ]
    }

    /// 就医提示
    pub fn seek(&self) -> Vec<&'static str> {
        vec![
            "频繁反酸就医",
            "伴吞咽困难胸痛重视",
            "长期反流检查",
            "勿自行长期滥用胃药",
        ]
    }
}

impl Rule for AcidRefluxCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("acid_reflux")
    }

    fn explain(&self) -> String {
        format!(
            "【胃酸反流管理】\n{}",
            [
                format!(
                    "认识症状：\\n{}",
                    self.symptoms()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "饮食调理：\\n{}",
                    self.diet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活习惯：\\n{}",
                    self.lifestyle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医提示：\\n{}",
                    self.seek()
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
    fn test_acidrefluxcarerules_basic() {
        let rules = AcidRefluxCareRules::new();
        assert_eq!(rules.metadata().name, "胃酸反流管理");
        assert!(!rules.symptoms().is_empty());
        assert!(!rules.diet().is_empty());
        assert!(!rules.lifestyle().is_empty());
        assert!(!rules.seek().is_empty());
    }

    #[test]
    fn test_acidrefluxcarerules_validation() {
        let rules = AcidRefluxCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("acid_reflux"));
    }

    #[test]
    fn test_acidrefluxcarerules_explain() {
        let rules = AcidRefluxCareRules::new();
        let e = rules.explain();
        assert!(e.contains("认识症状"));
        assert!(e.contains("饮食调理"));
        assert!(e.contains("生活习惯"));
    }
}
