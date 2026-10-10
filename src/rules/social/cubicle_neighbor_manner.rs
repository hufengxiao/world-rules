//! 工位邻里礼仪
//!
//! 相邻工位同事之间的相处分寸与礼节

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CubicleNeighborMannerRules,
    name: "工位邻里礼仪",
    desc: "相邻工位同事之间的相处分寸与礼节",
    origin: "职场通用",
    tags: ["职场", "办公", "礼仪"]
}

impl CubicleNeighborMannerRules {
    /// 空间尊重
    pub fn space(&self) -> Vec<&'static str> {
        vec![
            "不越界占用邻位",
            "借用物品先征询",
            "不窥探他人屏幕",
            "保持工位整洁",
        ]
    }

    /// 减少干扰
    pub fn interference(&self) -> Vec<&'static str> {
        vec![
            "专注时不随意打扰",
            "有事简短说明",
            "不长时间闲聊",
            "走动不碰邻位",
        ]
    }

    /// 日常相处
    pub fn interaction(&self) -> Vec<&'static str> {
        vec![
            "见面点头问候",
            "互帮小忙有来有往",
            "尊重各自习惯",
            "不传办公室八卦",
        ]
    }

    /// 矛盾处理
    pub fn conflict(&self) -> Vec<&'static str> {
        vec![
            "小摩擦委婉沟通",
            "不积怨不冷战",
            "问题就事论事",
            "必要时请上级协调",
        ]
    }
}

impl Rule for CubicleNeighborMannerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("cubicle_neighbor_manner")
    }

    fn explain(&self) -> String {
        format!(
            "【工位邻里礼仪】\n{}",
            [
                format!(
                    "空间尊重：\\n{}",
                    self.space()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "减少干扰：\\n{}",
                    self.interference()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "日常相处：\\n{}",
                    self.interaction()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "矛盾处理：\\n{}",
                    self.conflict()
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
    fn test_cubicleneighbormannerrules_basic() {
        let rules = CubicleNeighborMannerRules::new();
        assert_eq!(rules.metadata().name, "工位邻里礼仪");
        assert!(!rules.space().is_empty());
        assert!(!rules.interference().is_empty());
        assert!(!rules.interaction().is_empty());
        assert!(!rules.conflict().is_empty());
    }

    #[test]
    fn test_cubicleneighbormannerrules_validation() {
        let rules = CubicleNeighborMannerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::social("cubicle_neighbor_manner")
        );
    }

    #[test]
    fn test_cubicleneighbormannerrules_explain() {
        let rules = CubicleNeighborMannerRules::new();
        let e = rules.explain();
        assert!(e.contains("空间尊重"));
        assert!(e.contains("减少干扰"));
        assert!(e.contains("日常相处"));
    }
}
