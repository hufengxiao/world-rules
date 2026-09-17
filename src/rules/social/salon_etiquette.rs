//! 理发店与美容院礼仪
//!
//! 理发、美容服务中预约、沟通与互动的礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SalonEtiquetteRules,
    name: "理发店与美容院礼仪",
    desc: "理发、美容服务中预约、沟通与互动的礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "理发", "美容", "预约"]
}

impl SalonEtiquetteRules {
    /// 预约守时
    pub fn appointment(&self) -> Vec<&'static str> {
        vec![
            "提前预约并准时到店",
            "临时前往先问等待时长",
            "改期早通知店家",
            "了解服务与价格",
        ]
    }

    /// 清晰沟通
    pub fn brief(&self) -> Vec<&'static str> {
        vec![
            "说明期望的发型风格",
            "提供参考图片更直观",
            "说出不能接受的细节",
            "有异议及时反馈",
        ]
    }

    /// 店内行为
    pub fn store(&self) -> Vec<&'static str> {
        vec![
            "保持安静不霸占座位闲聊",
            "手机静音少接听",
            "不同意未消费升级推销",
            "不影响其他顾客",
        ]
    }

    /// 结束致谢
    pub fn finish(&self) -> Vec<&'static str> {
        vec![
            "先检查效果再离开",
            "不满意礼貌沟通调整",
            "对小费或感谢适度表达",
            "下次记住偏好",
        ]
    }
}

impl Rule for SalonEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("salon")
    }

    fn explain(&self) -> String {
        format!(
            "【理发店与美容院礼仪】\n{}",
            [
                format!(
                    "预约守时：\\n{}",
                    self.appointment()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "清晰沟通：\\n{}",
                    self.brief()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "店内行为：\\n{}",
                    self.store()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "结束致谢：\\n{}",
                    self.finish()
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
    fn test_salonetiquetterules_basic() {
        let rules = SalonEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "理发店与美容院礼仪");
        assert!(!rules.appointment().is_empty());
        assert!(!rules.brief().is_empty());
        assert!(!rules.store().is_empty());
        assert!(!rules.finish().is_empty());
    }

    #[test]
    fn test_salonetiquetterules_validation() {
        let rules = SalonEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("salon"));
    }

    #[test]
    fn test_salonetiquetterules_explain() {
        let rules = SalonEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("预约守时"));
        assert!(e.contains("清晰沟通"));
        assert!(e.contains("店内行为"));
    }
}
