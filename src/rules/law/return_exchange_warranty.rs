//! 退换货与三包
//!
//! 网购七日无理由退货与商品三包权益

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ReturnExchangeWarrantyRules,
    name: "退换货与三包",
    desc: "网购七日无理由退货与商品三包权益",
    origin: "中国",
    tags: ["消费", "退换货", "三包"]
}

impl ReturnExchangeWarrantyRules {
    /// 七日无理由
    pub fn seven_day(&self) -> Vec<&'static str> {
        vec![
            "网购可七日无理由退",
            "包装完好不影响",
            "部分商品不适用",
            "注意有无界定",
        ]
    }

    /// 质量问题
    pub fn defect(&self) -> Vec<&'static str> {
        vec![
            "质量有问题可退换",
            "十五日可换或修",
            "包退包换包修",
            "三包权益",
        ]
    }

    /// 退换办理
    pub fn process(&self) -> Vec<&'static str> {
        vec!["联系卖家申请", "按流程寄回", "运费依约", "凭证保留"]
    }

    /// 维权方式
    pub fn remedy(&self) -> Vec<&'static str> {
        vec!["商家推诿找平台", "协商投诉", "仲裁诉讼可选", "依法主张"]
    }
}

impl Rule for ReturnExchangeWarrantyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("return_warranty")
    }

    fn explain(&self) -> String {
        format!(
            "【退换货与三包】\n{}",
            [
                format!(
                    "七日无理由：\\n{}",
                    self.seven_day()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "质量问题：\\n{}",
                    self.defect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "退换办理：\\n{}",
                    self.process()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "维权方式：\\n{}",
                    self.remedy()
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
    fn test_returnexchangewarrantyrules_basic() {
        let rules = ReturnExchangeWarrantyRules::new();
        assert_eq!(rules.metadata().name, "退换货与三包");
        assert!(!rules.seven_day().is_empty());
        assert!(!rules.defect().is_empty());
        assert!(!rules.process().is_empty());
        assert!(!rules.remedy().is_empty());
    }

    #[test]
    fn test_returnexchangewarrantyrules_validation() {
        let rules = ReturnExchangeWarrantyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("return_warranty"));
    }

    #[test]
    fn test_returnexchangewarrantyrules_explain() {
        let rules = ReturnExchangeWarrantyRules::new();
        let e = rules.explain();
        assert!(e.contains("七日无理由"));
        assert!(e.contains("质量问题"));
        assert!(e.contains("退换办理"));
    }
}
