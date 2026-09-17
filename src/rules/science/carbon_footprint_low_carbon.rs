//! 碳足迹与低碳生活
//!
//! 认识碳排放来源与日常减排的科学常识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CarbonFootprintLowCarbonRules,
    name: "碳足迹与低碳生活",
    desc: "认识碳排放来源与日常减排的科学常识",
    origin: "国际",
    tags: ["科学", "碳足迹", "低碳", "环保"]
}

impl CarbonFootprintLowCarbonRules {
    /// 理解碳足迹
    pub fn understand(&self) -> Vec<&'static str> {
        vec![
            "碳足迹衡量活动排放",
            "交通用电饮食都含碳",
            "化石燃料是主要来源",
            "个体选择影响总量",
        ]
    }

    /// 交通减排
    pub fn transport(&self) -> Vec<&'static str> {
        vec![
            "多步行骑车乘公交",
            "减少不必要的出行",
            "拼车提高效率",
            "选择绿色交通方式",
        ]
    }

    /// 能源与饮食
    pub fn home(&self) -> Vec<&'static str> {
        vec![
            "节约用电随手关",
            "合理设定空调温度",
            "减少食物浪费",
            "多选本地应季食材",
        ]
    }

    /// 持续参与
    pub fn participate(&self) -> Vec<&'static str> {
        vec![
            "理性看待个体贡献",
            "支持环保产品与服务",
            "不迷信无意义噱头",
            "长期坚持低碳习惯",
        ]
    }
}

impl Rule for CarbonFootprintLowCarbonRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("carbon_footprint")
    }

    fn explain(&self) -> String {
        format!(
            "【碳足迹与低碳生活】\n{}",
            [
                format!(
                    "理解碳足迹：\\n{}",
                    self.understand()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "交通减排：\\n{}",
                    self.transport()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "能源与饮食：\\n{}",
                    self.home()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "持续参与：\\n{}",
                    self.participate()
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
    fn test_carbonfootprintlowcarbonrules_basic() {
        let rules = CarbonFootprintLowCarbonRules::new();
        assert_eq!(rules.metadata().name, "碳足迹与低碳生活");
        assert!(!rules.understand().is_empty());
        assert!(!rules.transport().is_empty());
        assert!(!rules.home().is_empty());
        assert!(!rules.participate().is_empty());
    }

    #[test]
    fn test_carbonfootprintlowcarbonrules_validation() {
        let rules = CarbonFootprintLowCarbonRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("carbon_footprint"));
    }

    #[test]
    fn test_carbonfootprintlowcarbonrules_explain() {
        let rules = CarbonFootprintLowCarbonRules::new();
        let e = rules.explain();
        assert!(e.contains("理解碳足迹"));
        assert!(e.contains("交通减排"));
        assert!(e.contains("能源与饮食"));
    }
}
