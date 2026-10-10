//! 经济补偿金
//!
//! 解除劳动合同经济补偿的适用与计算标准

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SeverancePayLawRules,
    name: "经济补偿金",
    desc: "解除劳动合同经济补偿的适用与计算标准",
    origin: "中国",
    tags: ["法律", "劳动", "补偿"]
}

impl SeverancePayLawRules {
    /// 适用情形
    pub fn scenarios(&self) -> Vec<&'static str> {
        vec![
            "协商一致解除",
            "单位无过失性辞退",
            "经济性裁员",
            "劳动者被迫解除",
        ]
    }

    /// 计算标准
    pub fn calculation(&self) -> Vec<&'static str> {
        vec![
            "每满一年支付一个月工资",
            "满半年不满一年按一年算",
            "不满半年支付半个月",
            "按解除前十二个月平均工资",
        ]
    }

    /// 上限规定
    pub fn cap(&self) -> Vec<&'static str> {
        vec![
            "月工资超三倍社平工资封顶",
            "年限最高按十二年计",
            "依当地上年度社平工资",
            "分两段分别计算",
        ]
    }

    /// 维权提示
    pub fn remedy(&self) -> Vec<&'static str> {
        vec![
            "违法解除须付赔偿金",
            "赔偿金为补偿金两倍",
            "保留解除通知书",
            "可申请劳动仲裁",
        ]
    }
}

impl Rule for SeverancePayLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("severance_pay_law")
    }

    fn explain(&self) -> String {
        format!(
            "【经济补偿金】\n{}",
            [
                format!(
                    "适用情形：\\n{}",
                    self.scenarios()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "计算标准：\\n{}",
                    self.calculation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "上限规定：\\n{}",
                    self.cap()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "维权提示：\\n{}",
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
    fn test_severancepaylawrules_basic() {
        let rules = SeverancePayLawRules::new();
        assert_eq!(rules.metadata().name, "经济补偿金");
        assert!(!rules.scenarios().is_empty());
        assert!(!rules.calculation().is_empty());
        assert!(!rules.cap().is_empty());
        assert!(!rules.remedy().is_empty());
    }

    #[test]
    fn test_severancepaylawrules_validation() {
        let rules = SeverancePayLawRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("severance_pay_law"));
    }

    #[test]
    fn test_severancepaylawrules_explain() {
        let rules = SeverancePayLawRules::new();
        let e = rules.explain();
        assert!(e.contains("适用情形"));
        assert!(e.contains("计算标准"));
        assert!(e.contains("上限规定"));
    }
}
