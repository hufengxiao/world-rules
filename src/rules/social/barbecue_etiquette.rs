//! 烧烤聚会礼仪
//!
//! 野外烧烤、BBQ聚会中的分工、用火安全与共享礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BarbecueEtiquetteRules,
    name: "烧烤聚会礼仪",
    desc: "野外烧烤、BBQ聚会中的分工、用火安全与共享礼仪",
    origin: "聚会",
    tags: ["社交", "礼仪", "烧烤", "聚会", "用火"]
}

impl BarbecueEtiquetteRules {
    /// 准备与分工
    pub fn preparation(&self) -> Vec<&'static str> {
        vec![
            "提前备好食材、炭火与工具",
            "分工负责烤制、递盘与照顾火源",
            "备好湿纸巾、洗手与灭火用品",
            "考虑大家的饮食偏好与忌口",
        ]
    }

    /// 用火安全
    pub fn fire(&self) -> Vec<&'static str> {
        vec![
            "在允许生火的地点烤制",
            "明火附近避免堆积易燃物",
            "烤架旁不离人看管",
            "结束后彻底熄灭火源清理灰烬",
        ]
    }

    /// 分享与照顾
    pub fn sharing(&self) -> Vec<&'static str> {
        vec![
            "烤熟食物依次分享给在场者",
            "不独占或抢食",
            "照顾食素者或忌口者的需求",
            "喝酒适量不劝酒过度",
        ]
    }

    /// 场地整洁
    pub fn cleanup(&self) -> Vec<&'static str> {
        vec![
            "结束带走喝完的食物与垃圾",
            "清洁烤架与桌面",
            "不在禁烤区留下痕迹",
            "归还场地整洁如初",
        ]
    }
}

impl Rule for BarbecueEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("barbecue")
    }

    fn explain(&self) -> String {
        format!(
            "【烧烤聚会礼仪】\n{}",
            [
                format!(
                    "准备与分工：\\n{}",
                    self.preparation()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "用火安全：\\n{}",
                    self.fire()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "分享与照顾：\\n{}",
                    self.sharing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "场地整洁：\\n{}",
                    self.cleanup()
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
    fn test_barbecueetiquetterules_basic() {
        let rules = BarbecueEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "烧烤聚会礼仪");
        assert!(!rules.preparation().is_empty());
        assert!(!rules.fire().is_empty());
        assert!(!rules.sharing().is_empty());
        assert!(!rules.cleanup().is_empty());
    }

    #[test]
    fn test_barbecueetiquetterules_validation() {
        let rules = BarbecueEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("barbecue"));
    }

    #[test]
    fn test_barbecueetiquetterules_explain() {
        let rules = BarbecueEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("准备与分工"));
        assert!(e.contains("用火安全"));
        assert!(e.contains("分享与照顾"));
    }
}
