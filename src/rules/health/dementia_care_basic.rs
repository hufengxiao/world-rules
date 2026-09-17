//! 失智症照护
//!
//! 失智老人的照护、沟通与安全要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DementiaCareBasicRules,
    name: "失智症照护",
    desc: "失智老人的照护、沟通与安全要点",
    origin: "医学",
    tags: ["健康", "失智", "照护", "老年"]
}

impl DementiaCareBasicRules {
    /// 及时就医
    pub fn diagnosis(&self) -> Vec<&'static str> {
        vec!["健忘影响生活就医", "家属早关注", "家属记录症状", "配合诊治"]
    }

    /// 耐心沟通
    pub fn communicate(&self) -> Vec<&'static str> {
        vec!["放慢语速耐心讲", "不纠正不责怪", "用简单句子", "顺应情绪"]
    }

    /// 安全环境
    pub fn safety(&self) -> Vec<&'static str> {
        vec!["防走失带铭牌", "明确家庭布置", "防跌倒防烫", "看护同伴"]
    }

    /// 生活陪伴
    pub fn companion(&self) -> Vec<&'static str> {
        vec!["规律作息活动", "回忆往事陪伴", "适量锻炼", "家属关爱"]
    }
}

impl Rule for DementiaCareBasicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("dementia")
    }

    fn explain(&self) -> String {
        format!(
            "【失智症照护】\n{}",
            [
                format!(
                    "及时就医：\\n{}",
                    self.diagnosis()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "耐心沟通：\\n{}",
                    self.communicate()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全环境：\\n{}",
                    self.safety()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活陪伴：\\n{}",
                    self.companion()
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
    fn test_dementiacarebasicrules_basic() {
        let rules = DementiaCareBasicRules::new();
        assert_eq!(rules.metadata().name, "失智症照护");
        assert!(!rules.diagnosis().is_empty());
        assert!(!rules.communicate().is_empty());
        assert!(!rules.safety().is_empty());
        assert!(!rules.companion().is_empty());
    }

    #[test]
    fn test_dementiacarebasicrules_validation() {
        let rules = DementiaCareBasicRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("dementia"));
    }

    #[test]
    fn test_dementiacarebasicrules_explain() {
        let rules = DementiaCareBasicRules::new();
        let e = rules.explain();
        assert!(e.contains("及时就医"));
        assert!(e.contains("耐心沟通"));
        assert!(e.contains("安全环境"));
    }
}
