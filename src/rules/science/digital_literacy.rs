//! 数字素养与事实核查
//!
//! 辨识网络信息真伪、保护个人信息的数据素养规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DigitalLiteracyRules,
    name: "数字素养与事实核查",
    desc: "辨识网络信息真伪、保护个人信息的数据素养规则",
    origin: "国际",
    tags: ["科学", "数字素养", "信息核查", "网络安全"]
}

impl DigitalLiteracyRules {
    /// 信息辨析
    pub fn verification(&self) -> Vec<&'static str> {
        vec![
            "对夸张标题保持警惕",
            "多方核对权威来源",
            "留意信息发布时间",
            "识别营销与软文内容",
        ]
    }

    /// 隐私保护
    pub fn privacy(&self) -> Vec<&'static str> {
        vec![
            "不轻易传播个人敏感信息",
            "设置复杂且独立的密码",
            "谨慎同意应用权限",
            "警惕钓鱼链接与诈骗",
        ]
    }

    /// 网络分享
    pub fn sharing(&self) -> Vec<&'static str> {
        vec![
            "转载信息注明来源",
            "不传播未验证的谣言",
            "尊重他人隐私与肖像",
            "理性表达避免网络暴力",
        ]
    }

    /// 批判思维
    pub fn critical(&self) -> Vec<&'static str> {
        vec![
            "质疑信息背后的动机",
            "区分事实与观点",
            "辨别数据与照片的操纵",
            "用证据支撑判断",
        ]
    }
}

impl Rule for DigitalLiteracyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("digital_literacy")
    }

    fn explain(&self) -> String {
        format!(
            "【数字素养与事实核查】\n{}",
            [
                format!(
                    "信息辨析：\\n{}",
                    self.verification()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "隐私保护：\\n{}",
                    self.privacy()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "网络分享：\\n{}",
                    self.sharing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "批判思维：\\n{}",
                    self.critical()
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
    fn test_digitalliteracyrules_basic() {
        let rules = DigitalLiteracyRules::new();
        assert_eq!(rules.metadata().name, "数字素养与事实核查");
        assert!(!rules.verification().is_empty());
        assert!(!rules.privacy().is_empty());
        assert!(!rules.sharing().is_empty());
        assert!(!rules.critical().is_empty());
    }

    #[test]
    fn test_digitalliteracyrules_validation() {
        let rules = DigitalLiteracyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("digital_literacy"));
    }

    #[test]
    fn test_digitalliteracyrules_explain() {
        let rules = DigitalLiteracyRules::new();
        let e = rules.explain();
        assert!(e.contains("信息辨析"));
        assert!(e.contains("隐私保护"));
        assert!(e.contains("网络分享"));
    }
}
