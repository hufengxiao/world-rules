//! 烧心反酸护理
//!
//! 胃酸反流引起烧心反酸的日常缓解与防护

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HeartburnRefluxRules,
    name: "烧心反酸护理",
    desc: "胃酸反流引起烧心反酸的日常缓解与防护",
    origin: "中国",
    tags: ["消化", "胃酸", "护理"]
}

impl HeartburnRefluxRules {
    /// 饮食规避
    pub fn avoid(&self) -> Vec<&'static str> {
        vec![
            "少食巧克力咖啡浓茶",
            "避免过酸过辣食物",
            "减少高脂油炸摄入",
            "忌饱餐后立即平卧",
        ]
    }

    /// 餐后护理
    pub fn after_meal(&self) -> Vec<&'static str> {
        vec![
            "餐后保持直立两小时",
            "睡前两三小时不进食",
            "抬高床头十至十五厘米",
            "左侧卧有助减少反流",
        ]
    }

    /// 生活调整
    pub fn lifestyle(&self) -> Vec<&'static str> {
        vec![
            "控制体重减轻腹压",
            "松解过紧的腰带衣物",
            "戒烟限酒保护贲门",
            "避免弯腰用力动作",
        ]
    }

    /// 就医警示
    pub fn warning(&self) -> Vec<&'static str> {
        vec![
            "频繁发作须就医评估",
            "吞咽困难体重下降就诊",
            "胸痛需先排除心脏问题",
            "长期反流防食管病变",
        ]
    }
}

impl Rule for HeartburnRefluxRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("heartburn_reflux")
    }

    fn explain(&self) -> String {
        format!(
            "【烧心反酸护理】\n{}",
            [
                format!(
                    "饮食规避：\\n{}",
                    self.avoid()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "餐后护理：\\n{}",
                    self.after_meal()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活调整：\\n{}",
                    self.lifestyle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医警示：\\n{}",
                    self.warning()
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
    fn test_heartburnrefluxrules_basic() {
        let rules = HeartburnRefluxRules::new();
        assert_eq!(rules.metadata().name, "烧心反酸护理");
        assert!(!rules.avoid().is_empty());
        assert!(!rules.after_meal().is_empty());
        assert!(!rules.lifestyle().is_empty());
        assert!(!rules.warning().is_empty());
    }

    #[test]
    fn test_heartburnrefluxrules_validation() {
        let rules = HeartburnRefluxRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("heartburn_reflux"));
    }

    #[test]
    fn test_heartburnrefluxrules_explain() {
        let rules = HeartburnRefluxRules::new();
        let e = rules.explain();
        assert!(e.contains("饮食规避"));
        assert!(e.contains("餐后护理"));
        assert!(e.contains("生活调整"));
    }
}
