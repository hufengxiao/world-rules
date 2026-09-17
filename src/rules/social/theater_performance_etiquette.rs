//! 剧场观演礼仪
//!
//! 音乐厅、剧院观演时的安静、鼓掌与秩序礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TheaterPerformanceEtiquetteRules,
    name: "剧场观演礼仪",
    desc: "音乐厅、剧院观演时的安静、鼓掌与秩序礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "剧场", "观演", "演出"]
}

impl TheaterPerformanceEtiquetteRules {
    /// 入场就座
    pub fn seating(&self) -> Vec<&'static str> {
        vec![
            "准时入场开场前就座",
            "迟到在幕间再入内",
            "按号就座不占位",
            "随身物品放脚边",
        ]
    }

    /// 演出安静
    pub fn quiet(&self) -> Vec<&'static str> {
        vec![
            "手机保持关机或静音",
            "不交头接耳议论",
            "不打开闪光灯拍照",
            "不在座席间走动",
        ]
    }

    /// 鼓掌时机
    pub fn applause(&self) -> Vec<&'static str> {
        vec![
            "演出段落结束再鼓掌",
            "乐章之间避免鼓掌",
            "谢幕时热烈鼓掌",
            "尊重演员谢幕",
        ]
    }

    /// 剧场行为
    pub fn manner(&self) -> Vec<&'static str> {
        vec![
            "不进食爆米花有声食品",
            "不翘脚挡后排视线",
            "散场有序不拥挤",
            "保持座位整洁",
        ]
    }
}

impl Rule for TheaterPerformanceEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("theater")
    }

    fn explain(&self) -> String {
        format!(
            "【剧场观演礼仪】\n{}",
            [
                format!(
                    "入场就座：\\n{}",
                    self.seating()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "演出安静：\\n{}",
                    self.quiet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "鼓掌时机：\\n{}",
                    self.applause()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "剧场行为：\\n{}",
                    self.manner()
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
    fn test_theaterperformanceetiquetterules_basic() {
        let rules = TheaterPerformanceEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "剧场观演礼仪");
        assert!(!rules.seating().is_empty());
        assert!(!rules.quiet().is_empty());
        assert!(!rules.applause().is_empty());
        assert!(!rules.manner().is_empty());
    }

    #[test]
    fn test_theaterperformanceetiquetterules_validation() {
        let rules = TheaterPerformanceEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("theater"));
    }

    #[test]
    fn test_theaterperformanceetiquetterules_explain() {
        let rules = TheaterPerformanceEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("入场就座"));
        assert!(e.contains("演出安静"));
        assert!(e.contains("鼓掌时机"));
    }
}
