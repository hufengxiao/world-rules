//! 漏水纠纷法律
//!
//! 楼上楼下漏水引发的责任划分与索赔处理

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WaterLeakDisputeLawRules,
    name: "漏水纠纷法律",
    desc: "楼上楼下漏水引发的责任划分与索赔处理",
    origin: "中国",
    tags: ["法律", "邻里", "侵权"]
}

impl WaterLeakDisputeLawRules {
    /// 责任认定
    pub fn liability(&self) -> Vec<&'static str> {
        vec![
            "查明漏水原因与责任方",
            "楼上过错致损应担责",
            "房屋质量问题找开发商",
            "公共管道问题由物业负责",
        ]
    }

    /// 证据固定
    pub fn evidence(&self) -> Vec<&'static str> {
        vec![
            "及时拍照录像留证",
            "记录损失物品清单",
            "请物业到场见证",
            "必要时做损失鉴定",
        ]
    }

    /// 处理步骤
    pub fn process(&self) -> Vec<&'static str> {
        vec![
            "先协商止损与维修",
            "请物业居间调解",
            "明确修复与赔偿方案",
            "达成书面协议",
        ]
    }

    /// 维权途径
    pub fn remedy(&self) -> Vec<&'static str> {
        vec![
            "协商不成可起诉",
            "依相邻关系主张权利",
            "可申请财产损失鉴定",
            "注意保留费用凭证",
        ]
    }
}

impl Rule for WaterLeakDisputeLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("water_leak_dispute_law")
    }

    fn explain(&self) -> String {
        format!(
            "【漏水纠纷法律】\n{}",
            [
                format!(
                    "责任认定：\\n{}",
                    self.liability()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "证据固定：\\n{}",
                    self.evidence()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "处理步骤：\\n{}",
                    self.process()
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
    fn test_waterleakdisputelawrules_basic() {
        let rules = WaterLeakDisputeLawRules::new();
        assert_eq!(rules.metadata().name, "漏水纠纷法律");
        assert!(!rules.liability().is_empty());
        assert!(!rules.evidence().is_empty());
        assert!(!rules.process().is_empty());
        assert!(!rules.remedy().is_empty());
    }

    #[test]
    fn test_waterleakdisputelawrules_validation() {
        let rules = WaterLeakDisputeLawRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::law("water_leak_dispute_law")
        );
    }

    #[test]
    fn test_waterleakdisputelawrules_explain() {
        let rules = WaterLeakDisputeLawRules::new();
        let e = rules.explain();
        assert!(e.contains("责任认定"));
        assert!(e.contains("证据固定"));
        assert!(e.contains("处理步骤"));
    }
}
