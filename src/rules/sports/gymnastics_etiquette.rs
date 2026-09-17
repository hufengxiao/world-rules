//! 体操观赛与礼仪
//!
//! 观赏体操比赛、评价与竞技风度礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: GymnasticsEtiquetteRules,
    name: "体操观赛与礼仪",
    desc: "观赏体操比赛、评价与竞技风度礼仪",
    origin: "国际",
    tags: ["体育", "体操", "观赛", "礼仪"]
}

impl GymnasticsEtiquetteRules {
    /// 观赛守则
    pub fn watch(&self) -> Vec<&'static str> {
        vec![
            "保持安静关注动作",
            "起评分等专业术语尊重",
            "不在旋转时闪光打扰",
            "动作结束鼓掌鼓励",
        ]
    }

    /// 尊重选手
    pub fn respect(&self) -> Vec<&'static str> {
        vec![
            "不以失误落井下石",
            "尊重每位选手努力",
            "不喧哗干扰起评",
            "公平看待裁判评定",
        ]
    }

    /// 竞技风范
    pub fn spirit(&self) -> Vec<&'static str> {
        vec![
            "尊重评判结果",
            "赢者谦逊败者坦然",
            "不指责对手或裁判",
            "喝彩得体",
        ]
    }

    /// 安全理解
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "理解高难度动作风险",
            "受伤时给予掌声鼓励",
            "不在未设区练习危险动作",
            "场馆遵循教练指导",
        ]
    }
}

impl Rule for GymnasticsEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("gymnastics")
    }

    fn explain(&self) -> String {
        format!(
            "【体操观赛与礼仪】\n{}",
            [
                format!(
                    "观赛守则：\\n{}",
                    self.watch()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "尊重选手：\\n{}",
                    self.respect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "竞技风范：\\n{}",
                    self.spirit()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全理解：\\n{}",
                    self.safety()
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
    fn test_gymnasticsetiquetterules_basic() {
        let rules = GymnasticsEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "体操观赛与礼仪");
        assert!(!rules.watch().is_empty());
        assert!(!rules.respect().is_empty());
        assert!(!rules.spirit().is_empty());
        assert!(!rules.safety().is_empty());
    }

    #[test]
    fn test_gymnasticsetiquetterules_validation() {
        let rules = GymnasticsEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("gymnastics"));
    }

    #[test]
    fn test_gymnasticsetiquetterules_explain() {
        let rules = GymnasticsEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("观赛守则"));
        assert!(e.contains("尊重选手"));
        assert!(e.contains("竞技风范"));
    }
}
