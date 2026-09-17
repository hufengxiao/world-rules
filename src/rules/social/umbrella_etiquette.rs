//! 雨天共伞与通勤礼仪
//!
//! 雨天撑伞、借伞、通行的公共礼仪与体谅规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: UmbrellaEtiquetteRules,
    name: "雨天共伞与通勤礼仪",
    desc: "雨天撑伞、借伞、通行的公共礼仪与体谅规则",
    origin: "国际",
    tags: ["社交", "礼仪", "雨天", "伞", "公共"]
}

impl UmbrellaEtiquetteRules {
    /// 撑伞通行
    pub fn walking(&self) -> Vec<&'static str> {
        vec![
            "与他人并行时伞尖朝外避免戳碰",
            "经过窄道时收拢一侧伞",
            "进出建筑前先收伞",
            "甩水时注意不溅到他人",
        ]
    }

    /// 公共空间
    pub fn public_space(&self) -> Vec<&'static str> {
        vec![
            "不在门口或通道撑伞挡路",
            "湿伞存放于沥水处",
            "不把湿伞靠他人座位",
            "湿伞收进袋中减少滴水",
        ]
    }

    /// 共伞与借伞
    pub fn sharing(&self) -> Vec<&'static str> {
        vec![
            "主动邀请无伞者同撑",
            "借伞及时归还",
            "归还时致谢对方",
            "不无故长时间占用他人伞",
        ]
    }

    /// 骑车与行车
    pub fn riding(&self) -> Vec<&'static str> {
        vec![
            "骑车尽量穿戴雨衣而非单手撑伞",
            "打伞行走注意来往车辆视线",
            "雨天减速留意防滑",
            "避免在窄道撑伞缓阻人群",
        ]
    }
}

impl Rule for UmbrellaEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("umbrella")
    }

    fn explain(&self) -> String {
        format!(
            "【雨天共伞与通勤礼仪】\n{}",
            [
                format!(
                    "撑伞通行：\\n{}",
                    self.walking()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "公共空间：\\n{}",
                    self.public_space()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "共伞与借伞：\\n{}",
                    self.sharing()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "骑车与行车：\\n{}",
                    self.riding()
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
    fn test_umbrellaetiquetterules_basic() {
        let rules = UmbrellaEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "雨天共伞与通勤礼仪");
        assert!(!rules.walking().is_empty());
        assert!(!rules.public_space().is_empty());
        assert!(!rules.sharing().is_empty());
        assert!(!rules.riding().is_empty());
    }

    #[test]
    fn test_umbrellaetiquetterules_validation() {
        let rules = UmbrellaEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("umbrella"));
    }

    #[test]
    fn test_umbrellaetiquetterules_explain() {
        let rules = UmbrellaEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("撑伞通行"));
        assert!(e.contains("公共空间"));
        assert!(e.contains("共伞与借伞"));
    }
}
