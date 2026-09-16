//! 酒店住宿礼仪
//!
//! 酒店、宾馆入住住宿时的公共礼仪与自律规范

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HotelStayEtiquetteRules,
    name: "酒店住宿礼仪",
    desc: "酒店、宾馆入住住宿时的公共礼仪与自律规范",
    origin: "住宿",
    tags: ["社交", "礼仪", "酒店", "住宿", "出行"]
}

impl HotelStayEtiquetteRules {
    /// 入住登记
    pub fn checkin(&self) -> Vec<&'static str> {
        vec![
            "入住时出示有效证件登记",
            "询问清楚退房与押金事宜",
            "不虚报住宿人数",
            "妥善保管房卡与钥匙",
        ]
    }

    /// 房间礼仪
    pub fn room_etiquette(&self) -> Vec<&'static str> {
        vec![
            "出入房间保持安静勿喧哗",
            "减少走廊大声交谈",
            "不向窗外丢弃物品",
            "保持室内整洁有序",
        ]
    }

    /// 晚间行为
    pub fn nighttime(&self) -> Vec<&'static str> {
        vec![
            "深夜降低电视与说话音量",
            "不外放音响打扰邻房",
            "聚会娱乐注意时段",
            "走廊走动轻缓",
        ]
    }

    /// 退房
    pub fn checkout(&self) -> Vec<&'static str> {
        vec![
            "按时办理退房",
            "回顾房间有无遗漏随身物品",
            "不乱损坏酒店财物",
            "对提供的便利表达感谢",
        ]
    }
}

impl Rule for HotelStayEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("hotel_stay")
    }

    fn explain(&self) -> String {
        format!(
            "【酒店住宿礼仪】\n{}",
            [
                format!(
                    "入住登记：\\n{}",
                    self.checkin()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "房间礼仪：\\n{}",
                    self.room_etiquette()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "晚间行为：\\n{}",
                    self.nighttime()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "退房：\\n{}",
                    self.checkout()
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
    fn test_hotelstayetiquetterules_basic() {
        let rules = HotelStayEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "酒店住宿礼仪");
        assert!(!rules.checkin().is_empty());
        assert!(!rules.room_etiquette().is_empty());
        assert!(!rules.nighttime().is_empty());
        assert!(!rules.checkout().is_empty());
    }

    #[test]
    fn test_hotelstayetiquetterules_validation() {
        let rules = HotelStayEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("hotel_stay"));
    }

    #[test]
    fn test_hotelstayetiquetterules_explain() {
        let rules = HotelStayEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("入住登记"));
        assert!(e.contains("房间礼仪"));
        assert!(e.contains("晚间行为"));
    }
}
