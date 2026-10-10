//! 直播带货法律
//!
//! 直播销售中主播、商家与平台的法律责任

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: LiveCommerceLawRules,
    name: "直播带货法律",
    desc: "直播销售中主播、商家与平台的法律责任",
    origin: "中国",
    tags: ["法律", "电商", "直播"]
}

impl LiveCommerceLawRules {
    /// 主体责任
    pub fn liability(&self) -> Vec<&'static str> {
        vec![
            "商家对商品质量负责",
            "主播虚假宣传须担责",
            "平台负审核管理义务",
            "广告代言人亦担责",
        ]
    }

    /// 宣传规范
    pub fn advertising(&self) -> Vec<&'static str> {
        vec![
            "不得虚假夸大宣传",
            "禁用绝对化用语",
            "数据销量须真实",
            "医疗保健特殊监管",
        ]
    }

    /// 消费者权利
    pub fn consumer_rights(&self) -> Vec<&'static str> {
        vec![
            "享七日无理由退货",
            "质量问题可要求赔偿",
            "虚假宣传可主张三倍赔",
            "保留直播录屏为证",
        ]
    }

    /// 维权途径
    pub fn remedy(&self) -> Vec<&'static str> {
        vec![
            "先与商家协商退换",
            "向平台投诉举报",
            "拨打一二三一五投诉",
            "必要时提起诉讼",
        ]
    }
}

impl Rule for LiveCommerceLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("live_commerce_law")
    }

    fn explain(&self) -> String {
        format!(
            "【直播带货法律】\n{}",
            [
                format!(
                    "主体责任：\\n{}",
                    self.liability()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "宣传规范：\\n{}",
                    self.advertising()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "消费者权利：\\n{}",
                    self.consumer_rights()
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
    fn test_livecommercelawrules_basic() {
        let rules = LiveCommerceLawRules::new();
        assert_eq!(rules.metadata().name, "直播带货法律");
        assert!(!rules.liability().is_empty());
        assert!(!rules.advertising().is_empty());
        assert!(!rules.consumer_rights().is_empty());
        assert!(!rules.remedy().is_empty());
    }

    #[test]
    fn test_livecommercelawrules_validation() {
        let rules = LiveCommerceLawRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("live_commerce_law"));
    }

    #[test]
    fn test_livecommercelawrules_explain() {
        let rules = LiveCommerceLawRules::new();
        let e = rules.explain();
        assert!(e.contains("主体责任"));
        assert!(e.contains("宣传规范"));
        assert!(e.contains("消费者权利"));
    }
}
