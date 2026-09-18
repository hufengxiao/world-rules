//! 缺陷产品召回与赔偿
//!
//! 有安全缺陷的产品须召回并赔偿消费者的法律规定

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ProductDefectRecallRules,
    name: "缺陷产品召回与赔偿",
    desc: "有安全缺陷的产品须召回并赔偿消费者的法律规定",
    origin: "中国",
    tags: ["法律", "产品", "召回", "消费者"]
}

impl ProductDefectRecallRules {
    /// 缺陷定义
    pub fn identify(&self) -> Vec<&'static str> {
        vec![
            "存在不安全风险",
            "设计或制造缺陷",
            "警示不足",
            "危及人身财产",
        ]
    }

    /// 召回义务
    pub fn recall(&self) -> Vec<&'static str> {
        vec![
            "生产者须召回",
            "停止销售问题产品",
            "免费更换或退",
            "及时公告警示",
        ]
    }

    /// 赔偿责任
    pub fn liability(&self) -> Vec<&'static str> {
        vec![
            "因缺陷致损可索赔",
            "赔偿实际损失",
            "销售者先行负责",
            "可向生产者追偿",
        ]
    }

    /// 维权途径
    pub fn remedy(&self) -> Vec<&'static str> {
        vec!["保留购买凭证", "找商家反映", "投诉市场监管", "诉讼或仲裁"]
    }
}

impl Rule for ProductDefectRecallRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("product_defect")
    }

    fn explain(&self) -> String {
        format!(
            "【缺陷产品召回与赔偿】\n{}",
            [
                format!(
                    "缺陷定义：\\n{}",
                    self.identify()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "召回义务：\\n{}",
                    self.recall()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "赔偿责任：\\n{}",
                    self.liability()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "维权途径：\\n{}",
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
    fn test_productdefectrecallrules_basic() {
        let rules = ProductDefectRecallRules::new();
        assert_eq!(rules.metadata().name, "缺陷产品召回与赔偿");
        assert!(!rules.identify().is_empty());
        assert!(!rules.recall().is_empty());
        assert!(!rules.liability().is_empty());
        assert!(!rules.remedy().is_empty());
    }

    #[test]
    fn test_productdefectrecallrules_validation() {
        let rules = ProductDefectRecallRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("product_defect"));
    }

    #[test]
    fn test_productdefectrecallrules_explain() {
        let rules = ProductDefectRecallRules::new();
        let e = rules.explain();
        assert!(e.contains("缺陷定义"));
        assert!(e.contains("召回义务"));
        assert!(e.contains("赔偿责任"));
    }
}
