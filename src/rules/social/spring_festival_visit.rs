//! 春节拜年礼仪
//!
//! 春节串门拜年、礼品与问候的礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SpringFestivalVisitRules,
    name: "春节拜年礼仪",
    desc: "春节串门拜年、礼品与问候的礼仪",
    origin: "中国",
    tags: ["社交", "礼仪", "春节", "拜年"]
}

impl SpringFestivalVisitRules {
    /// 拜年时机
    pub fn timing(&self) -> Vec<&'static str> {
        vec![
            "拜年选合适时间段",
            "避免过早或过晚打扰",
            "提前打个招呼",
            "配合对方到访安排",
        ]
    }

    /// 礼物礼节
    pub fn gift(&self) -> Vec<&'static str> {
        vec![
            "带恰当节日礼品",
            "贵重在心意",
            "礼品简洁得体",
            "不说服对方破费",
        ]
    }

    /// 问候交谈
    pub fn greet(&self) -> Vec<&'static str> {
        vec![
            "主动向长辈拜年",
            "送上吉祥祝福",
            "热情交谈融洽",
            "不探人隐私",
        ]
    }

    /// 告别礼让
    pub fn depart(&self) -> Vec<&'static str> {
        vec![
            "适时告辞不恋战",
            "感谢热情款待",
            "出门口衷心道别",
            "节日气氛其乐融融",
        ]
    }
}

impl Rule for SpringFestivalVisitRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("spring_festival")
    }

    fn explain(&self) -> String {
        format!(
            "【春节拜年礼仪】\n{}",
            [
                format!(
                    "拜年时机：\\n{}",
                    self.timing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "礼物礼节：\\n{}",
                    self.gift()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "问候交谈：\\n{}",
                    self.greet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "告别礼让：\\n{}",
                    self.depart()
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
    fn test_springfestivalvisitrules_basic() {
        let rules = SpringFestivalVisitRules::new();
        assert_eq!(rules.metadata().name, "春节拜年礼仪");
        assert!(!rules.timing().is_empty());
        assert!(!rules.gift().is_empty());
        assert!(!rules.greet().is_empty());
        assert!(!rules.depart().is_empty());
    }

    #[test]
    fn test_springfestivalvisitrules_validation() {
        let rules = SpringFestivalVisitRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("spring_festival"));
    }

    #[test]
    fn test_springfestivalvisitrules_explain() {
        let rules = SpringFestivalVisitRules::new();
        let e = rules.explain();
        assert!(e.contains("拜年时机"));
        assert!(e.contains("礼物礼节"));
        assert!(e.contains("问候交谈"));
    }
}
