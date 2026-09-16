//! 健身房礼仪
//!
//! 健身房、健身中心锻炼时的公共礼仪与安全约定

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: GymEtiquetteRules,
    name: "健身房礼仪",
    desc: "健身房、健身中心锻炼时的公共礼仪与安全约定",
    origin: "国际",
    tags: ["社交", "礼仪", "健身房", "健身"]
}

impl GymEtiquetteRules {
    /// 器械使用
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "用完器材擦拭干净",
            "不长时间霸占用同一器械",
            "归还哑铃与杠铃到原位",
            "器械调节适合自己的设置后再使用",
            "重物搬运时注意周围人群",
        ]
    }

    /// 空间与照镜
    pub fn space(&self) -> Vec<&'static str> {
        vec![
            "不做挡住他人的夸张动作",
            "多人共用镜子时轮流使用",
            "不长时间占据镜子前位置",
            "抱团或闲聊不当道",
        ]
    }

    /// 声音管理
    pub fn noise(&self) -> Vec<&'static str> {
        vec![
            "不刻意发出过大喊叫",
            "音乐耳机不漏音",
            "避免大声打电话",
            "剧烈发力可轻声呼气",
        ]
    }

    /// 卫生与安全
    pub fn hygiene(&self) -> Vec<&'static str> {
        vec![
            "训练前擦健身凳与把手",
            "出汗多时及时擦干",
            "不赤脚在器材区活动",
            "重训前检查器材完好",
        ]
    }
}

impl Rule for GymEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("gym")
    }

    fn explain(&self) -> String {
        let parts = vec![
            format!(
                "器械使用：\\n{}",
                self.equipment()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
            format!(
                "空间与照镜：\\n{}",
                self.space()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
            format!(
                "声音管理：\\n{}",
                self.noise()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
            format!(
                "卫生与安全：\\n{}",
                self.hygiene()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
        ];
        format!("【健身房礼仪】\n{}", parts.join("\n\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_gymetiquetterules_basic() {
        let rules = GymEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "健身房礼仪");
        assert!(!rules.equipment().is_empty());
        assert!(!rules.space().is_empty());
        assert!(!rules.noise().is_empty());
        assert!(!rules.hygiene().is_empty());
    }

    #[test]
    fn test_gymetiquetterules_validation() {
        let rules = GymEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("gym"));
    }

    #[test]
    fn test_gymetiquetterules_explain() {
        let rules = GymEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("器械使用"));
        assert!(e.contains("空间与照镜"));
        assert!(e.contains("声音管理"));
    }
}
