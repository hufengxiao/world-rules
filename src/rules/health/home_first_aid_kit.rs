//! 家庭急救箱配置
//!
//! 配置家庭急救箱的用品清单、检查与正确使用规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HomeFirstAidKitRules,
    name: "家庭急救箱配置",
    desc: "配置家庭急救箱的用品清单、检查与正确使用规则",
    origin: "安全",
    tags: ["健康", "急救箱", "家庭", "应急"]
}

impl HomeFirstAidKitRules {
    /// 常备用品
    pub fn supplies(&self) -> Vec<&'static str> {
        vec![
            "备齐无菌纱布绷带与胶布",
            "备消毒棉签与碘伏酒精",
            "准备一次性手套与剪刀",
            "备体温计与常用药(遵医嘱)",
        ]
    }

    /// 放置保管
    pub fn storage(&self) -> Vec<&'static str> {
        vec![
            "放置在儿童不易触到处",
            "放在通风干燥阴凉处",
            "标记清楚摆为何物",
            "让家人知晓位置",
        ]
    }

    /// 检查维护
    pub fn maintain(&self) -> Vec<&'static str> {
        vec![
            "定期检查药品保质期",
            "及时补充消耗品",
            "核对器械是否完好",
            "过期药物妥善处理",
        ]
    }

    /// 正确使用
    pub fn usage(&self) -> Vec<&'static str> {
        vec![
            "按说明正确使用器材药品",
            "不确定时求助专业",
            "大规模伤情先就医",
            "急救后及时记录备查",
        ]
    }
}

impl Rule for HomeFirstAidKitRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("first_aid_kit")
    }

    fn explain(&self) -> String {
        format!(
            "【家庭急救箱配置】\n{}",
            [
                format!(
                    "常备用品：\\n{}",
                    self.supplies()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "放置保管：\\n{}",
                    self.storage()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "检查维护：\\n{}",
                    self.maintain()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "正确使用：\\n{}",
                    self.usage()
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
    fn test_homefirstaidkitrules_basic() {
        let rules = HomeFirstAidKitRules::new();
        assert_eq!(rules.metadata().name, "家庭急救箱配置");
        assert!(!rules.supplies().is_empty());
        assert!(!rules.storage().is_empty());
        assert!(!rules.maintain().is_empty());
        assert!(!rules.usage().is_empty());
    }

    #[test]
    fn test_homefirstaidkitrules_validation() {
        let rules = HomeFirstAidKitRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("first_aid_kit"));
    }

    #[test]
    fn test_homefirstaidkitrules_explain() {
        let rules = HomeFirstAidKitRules::new();
        let e = rules.explain();
        assert!(e.contains("常备用品"));
        assert!(e.contains("放置保管"));
        assert!(e.contains("检查维护"));
    }
}
