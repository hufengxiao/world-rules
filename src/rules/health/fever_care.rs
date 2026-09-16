//! 发热家居护理
//!
//! 发热时的居家监测、退热处理与就医判断规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FeverCareRules,
    name: "发热家居护理",
    desc: "发热时的居家监测、退热处理与就医判断规则",
    origin: "医学",
    tags: ["健康", "发热", "退烧", "护理"]
}

impl FeverCareRules {
    /// 体温监测
    pub fn monitor(&self) -> Vec<&'static str> {
        vec![
            "定时测量记录体温变化",
            "观察呼吸、精神状态与皮肤",
            "动态评估是否有所缓解",
            "记录用药时间与剂量",
        ]
    }

    /// 降温措施
    pub fn cooling(&self) -> Vec<&'static str> {
        vec![
            "体温过高时适度物理降温",
            "用温水擦浴而非酒精擦身",
            "多喝温水防止脱水",
            "减少厚被束缚利散热",
        ]
    }

    /// 用药注意
    pub fn medication(&self) -> Vec<&'static str> {
        vec![
            "发热量高时按剂量间歇服退烧药",
            "不叠加相同成分重复用药",
            "严格遵循说明书或医嘱",
            "儿童不用阿司匹林",
        ]
    }

    /// 危险信号
    pub fn danger_signs(&self) -> Vec<&'static str> {
        vec![
            "持续高热多有惊厥立即求助",
            "出现皮疹瘀斑或颈项强直",
            "呼吸困难或意识模糊",
            "婴幼儿烍吵异常立即就医",
        ]
    }
}

impl Rule for FeverCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("fever")
    }

    fn explain(&self) -> String {
        format!(
            "【发热家居护理】\n{}",
            [
                format!(
                    "体温监测：\\n{}",
                    self.monitor()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "降温措施：\\n{}",
                    self.cooling()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "用药注意：\\n{}",
                    self.medication()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "危险信号：\\n{}",
                    self.danger_signs()
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
    fn test_fevercarerules_basic() {
        let rules = FeverCareRules::new();
        assert_eq!(rules.metadata().name, "发热家居护理");
        assert!(!rules.monitor().is_empty());
        assert!(!rules.cooling().is_empty());
        assert!(!rules.medication().is_empty());
        assert!(!rules.danger_signs().is_empty());
    }

    #[test]
    fn test_fevercarerules_validation() {
        let rules = FeverCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("fever"));
    }

    #[test]
    fn test_fevercarerules_explain() {
        let rules = FeverCareRules::new();
        let e = rules.explain();
        assert!(e.contains("体温监测"));
        assert!(e.contains("降温措施"));
        assert!(e.contains("用药注意"));
    }
}
