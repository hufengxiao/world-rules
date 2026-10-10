//! 智齿护理
//!
//! 智齿萌出与拔除前后的口腔护理要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WisdomToothCareRules,
    name: "智齿护理",
    desc: "智齿萌出与拔除前后的口腔护理要点",
    origin: "中国",
    tags: ["口腔", "牙齿", "护理"]
}

impl WisdomToothCareRules {
    /// 常见问题
    pub fn problems(&self) -> Vec<&'static str> {
        vec![
            "阻生智齿易发炎",
            "冠周炎致牙龈肿痛",
            "挤压邻牙引发龋坏",
            "反复肿痛应处理",
        ]
    }

    /// 发炎护理
    pub fn inflammation(&self) -> Vec<&'static str> {
        vec![
            "淡盐水漱口清洁",
            "饭后及时漱口",
            "冷敷缓解肿痛",
            "疼痛明显应就医",
        ]
    }

    /// 拔牙前后
    pub fn extraction(&self) -> Vec<&'static str> {
        vec![
            "拔牙前如实告知病史",
            "拔后咬紧棉球止血",
            "当日不漱口不刷牙",
            "进温凉软食",
        ]
    }

    /// 术后注意
    pub fn aftercare(&self) -> Vec<&'static str> {
        vec![
            "避免用力吐口水",
            "勿用吸管饮水",
            "遵医嘱服药",
            "异常出血及时复诊",
        ]
    }
}

impl Rule for WisdomToothCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("wisdom_tooth_care")
    }

    fn explain(&self) -> String {
        format!(
            "【智齿护理】\n{}",
            [
                format!(
                    "常见问题：\\n{}",
                    self.problems()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "发炎护理：\\n{}",
                    self.inflammation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "拔牙前后：\\n{}",
                    self.extraction()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "术后注意：\\n{}",
                    self.aftercare()
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
    fn test_wisdomtoothcarerules_basic() {
        let rules = WisdomToothCareRules::new();
        assert_eq!(rules.metadata().name, "智齿护理");
        assert!(!rules.problems().is_empty());
        assert!(!rules.inflammation().is_empty());
        assert!(!rules.extraction().is_empty());
        assert!(!rules.aftercare().is_empty());
    }

    #[test]
    fn test_wisdomtoothcarerules_validation() {
        let rules = WisdomToothCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("wisdom_tooth_care"));
    }

    #[test]
    fn test_wisdomtoothcarerules_explain() {
        let rules = WisdomToothCareRules::new();
        let e = rules.explain();
        assert!(e.contains("常见问题"));
        assert!(e.contains("发炎护理"));
        assert!(e.contains("拔牙前后"));
    }
}
