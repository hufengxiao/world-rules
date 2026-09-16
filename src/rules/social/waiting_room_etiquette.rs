//! 等候室与候诊礼仪
//!
//! 医院候诊室、服务窗口等候区等的秩序与安静礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WaitingRoomEtiquetteRules,
    name: "等候室与候诊礼仪",
    desc: "医院候诊室、服务窗口等候区等的秩序与安静礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "候诊", "等候", "医院"]
}

impl WaitingRoomEtiquetteRules {
    /// 排队秩序
    pub fn queue(&self) -> Vec<&'static str> {
        vec![
            "按规定依次排队取号",
            "不插队不代排越多人",
            "尊重叫号次序",
            "需要帮助的老弱可礼貌请求优先",
        ]
    }

    /// 保持安静
    pub fn quiet(&self) -> Vec<&'static str> {
        vec![
            "候诊时保持安静",
            "接打电话到外压低音量",
            "不围观他人就诊信息",
            "照顾儿童不使其喧闹",
        ]
    }

    /// 尊重他人
    pub fn respect(&self) -> Vec<&'static str> {
        vec![
            "不随意评论他人病情",
            "为携带婴儿者让座",
            "不探听他人就医隐私",
            "理解医护接待顺序",
        ]
    }

    /// 环境卫生
    pub fn hygiene(&self) -> Vec<&'static str> {
        vec![
            "不随意丢弃垃圾",
            "咳嗽喷嚏用手肘遮挡",
            "症状急重者及时告知前台",
            "离开带走随身物品",
        ]
    }
}

impl Rule for WaitingRoomEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("waiting_room")
    }

    fn explain(&self) -> String {
        format!(
            "【等候室与候诊礼仪】\n{}",
            [
                format!(
                    "排队秩序：\\n{}",
                    self.queue()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "保持安静：\\n{}",
                    self.quiet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "尊重他人：\\n{}",
                    self.respect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "环境卫生：\\n{}",
                    self.hygiene()
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
    fn test_waitingroometiquetterules_basic() {
        let rules = WaitingRoomEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "等候室与候诊礼仪");
        assert!(!rules.queue().is_empty());
        assert!(!rules.quiet().is_empty());
        assert!(!rules.respect().is_empty());
        assert!(!rules.hygiene().is_empty());
    }

    #[test]
    fn test_waitingroometiquetterules_validation() {
        let rules = WaitingRoomEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("waiting_room"));
    }

    #[test]
    fn test_waitingroometiquetterules_explain() {
        let rules = WaitingRoomEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("排队秩序"));
        assert!(e.contains("保持安静"));
        assert!(e.contains("尊重他人"));
    }
}
