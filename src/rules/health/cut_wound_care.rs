//! 割伤擦伤护理
//!
//! 皮肤割伤、擦伤等小伤口的清洗止血与护理规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CutWoundCareRules,
    name: "割伤擦伤护理",
    desc: "皮肤割伤、擦伤等小伤口的清洗止血与护理规则",
    origin: "医学",
    tags: ["健康", "伤口", "割伤", "擦伤", "止血"]
}

impl CutWoundCareRules {
    /// 清洁止血
    pub fn stop_bleeding(&self) -> Vec<&'static str> {
        vec![
            "用洁净清水冲洗伤口清除污物",
            "用无菌纱布按压止血",
            "持续按压数分钟至出血减缓",
            "伤口较深出血不止需就医",
        ]
    }

    /// 消毒包扎
    pub fn dressing(&self) -> Vec<&'static str> {
        vec![
            "涂抹消毒液进行消毒",
            "用无菌纱布包扎保持干燥",
            "定期更换敷料观察愈合",
            "不用脏布等直接覆盖伤口",
        ]
    }

    /// 感染观察
    pub fn infection(&self) -> Vec<&'static str> {
        vec![
            "观察红肿、发热、渗脓等迹象",
            "伤口疼痛加剧需就诊",
            "持续发热提示可能感染",
            "有异物残留时就医取出",
        ]
    }

    /// 特别提醒
    pub fn special(&self) -> Vec<&'static str> {
        vec![
            "动物咬伤深层需评估破伤风",
            "铁锈、污染较深伤口就医",
            "糖尿病者小伤也应及时处理",
            "成人陈旧但异常逐渐出血就医",
        ]
    }
}

impl Rule for CutWoundCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("cut_wound")
    }

    fn explain(&self) -> String {
        format!(
            "【割伤擦伤护理】\n{}",
            [
                format!(
                    "清洁止血：\\n{}",
                    self.stop_bleeding()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "消毒包扎：\\n{}",
                    self.dressing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "感染观察：\\n{}",
                    self.infection()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "特别提醒：\\n{}",
                    self.special()
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
    fn test_cutwoundcarerules_basic() {
        let rules = CutWoundCareRules::new();
        assert_eq!(rules.metadata().name, "割伤擦伤护理");
        assert!(!rules.stop_bleeding().is_empty());
        assert!(!rules.dressing().is_empty());
        assert!(!rules.infection().is_empty());
        assert!(!rules.special().is_empty());
    }

    #[test]
    fn test_cutwoundcarerules_validation() {
        let rules = CutWoundCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("cut_wound"));
    }

    #[test]
    fn test_cutwoundcarerules_explain() {
        let rules = CutWoundCareRules::new();
        let e = rules.explain();
        assert!(e.contains("清洁止血"));
        assert!(e.contains("消毒包扎"));
        assert!(e.contains("感染观察"));
    }
}
