//! 图书馆自习室礼仪
//!
//! 图书馆、自习室阅读与自习时的安静与礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ReadingRoomEtiquetteRules,
    name: "图书馆自习室礼仪",
    desc: "图书馆、自习室阅读与自习时的安静与礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "图书馆", "自习", "安静"]
}

impl ReadingRoomEtiquetteRules {
    /// 保持安静
    pub fn quiet(&self) -> Vec<&'static str> {
        vec![
            "不大声交谈或接打电话",
            "手机静音震动",
            "搬动椅器物轻放",
            "需要接听走到室外",
        ]
    }

    /// 占位自律
    pub fn seating(&self) -> Vec<&'static str> {
        vec![
            "不长期空占座位",
            "离开较久避免占位",
            "尊重先到者的位置",
            "用餐时到休息区",
        ]
    }

    /// 公共秩序
    pub fn order(&self) -> Vec<&'static str> {
        vec![
            "书籍阅后归位",
            "排队借还书",
            "遵守开放时间",
            "饿了到指定区域进食",
        ]
    }

    /// 彼此体谅
    pub fn consideration(&self) -> Vec<&'static str> {
        vec![
            "使用设备戴耳机",
            "避免频繁进出",
            "带资料轻拿轻放",
            "尊重他人专注学习",
        ]
    }
}

impl Rule for ReadingRoomEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("reading_room")
    }

    fn explain(&self) -> String {
        format!(
            "【图书馆自习室礼仪】\n{}",
            [
                format!(
                    "保持安静：\\n{}",
                    self.quiet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "占位自律：\\n{}",
                    self.seating()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "公共秩序：\\n{}",
                    self.order()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "彼此体谅：\\n{}",
                    self.consideration()
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
    fn test_readingroometiquetterules_basic() {
        let rules = ReadingRoomEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "图书馆自习室礼仪");
        assert!(!rules.quiet().is_empty());
        assert!(!rules.seating().is_empty());
        assert!(!rules.order().is_empty());
        assert!(!rules.consideration().is_empty());
    }

    #[test]
    fn test_readingroometiquetterules_validation() {
        let rules = ReadingRoomEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("reading_room"));
    }

    #[test]
    fn test_readingroometiquetterules_explain() {
        let rules = ReadingRoomEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("保持安静"));
        assert!(e.contains("占位自律"));
        assert!(e.contains("公共秩序"));
    }
}
