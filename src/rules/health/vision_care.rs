//! 用眼护眼健康
//!
//! 日常用眼卫生、护眼习惯与视力保护规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: VisionCareRules,
    name: "用眼护眼健康",
    desc: "日常用眼卫生、护眼习惯与视力保护规则",
    origin: "医学",
    tags: ["健康", "眼睛", "视力", "护眼", "用眼"]
}

impl VisionCareRules {
    /// 用眼习惯
    pub fn habits(&self) -> Vec<&'static str> {
        vec![
            "每用眼约40分钟远眺休息",
            "保持适当阅读或屏幕距离",
            "光线充足均匀照明",
            "不躺着或乘车强光下看书",
        ]
    }

    /// 屏幕护眼
    pub fn screen(&self) -> Vec<&'static str> {
        vec![
            "屏幕亮度适中不过亮过暗",
            "间歇闭眼或向远处放松",
            "使用抗蓝光设置宜适度",
            "久屏不宜过度揉眼",
        ]
    }

    /// 卫生与检查
    pub fn care(&self) -> Vec<&'static str> {
        vec![
            "不揉眼以防感染",
            "佩戴眼镜者定期检查屈光",
            "眼部异常如飞蚊症及时就诊",
            "儿童护眼重视早期筛查",
        ]
    }

    /// 营养与户外
    pub fn lifestyle(&self) -> Vec<&'static str> {
        vec![
            "多摄取富含维A食材",
            "多蔬菜多深色蔬果",
            "适当户外活动让眼远望",
            "避免用眼过度疲劳",
        ]
    }
}

impl Rule for VisionCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("vision")
    }

    fn explain(&self) -> String {
        format!(
            "【用眼护眼健康】\n{}",
            [
                format!(
                    "用眼习惯：\\n{}",
                    self.habits()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "屏幕护眼：\\n{}",
                    self.screen()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "卫生与检查：\\n{}",
                    self.care()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "营养与户外：\\n{}",
                    self.lifestyle()
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
    fn test_visioncarerules_basic() {
        let rules = VisionCareRules::new();
        assert_eq!(rules.metadata().name, "用眼护眼健康");
        assert!(!rules.habits().is_empty());
        assert!(!rules.screen().is_empty());
        assert!(!rules.care().is_empty());
        assert!(!rules.lifestyle().is_empty());
    }

    #[test]
    fn test_visioncarerules_validation() {
        let rules = VisionCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("vision"));
    }

    #[test]
    fn test_visioncarerules_explain() {
        let rules = VisionCareRules::new();
        let e = rules.explain();
        assert!(e.contains("用眼习惯"));
        assert!(e.contains("屏幕护眼"));
        assert!(e.contains("卫生与检查"));
    }
}
