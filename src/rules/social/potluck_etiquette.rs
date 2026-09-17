//! 家常聚餐礼仪
//!
//! 自带菜式聚餐、分工与共享的礼貌规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PotluckEtiquetteRules,
    name: "家常聚餐礼仪",
    desc: "自带菜式聚餐、分工与共享的礼貌规则",
    origin: "国际",
    tags: ["社交", "礼仪", "聚餐", "分享"]
}

impl PotluckEtiquetteRules {
    /// 携物约定
    pub fn bring(&self) -> Vec<&'static str> {
        vec![
            "按分工带足分量",
            "携带符合主题的菜式",
            "标注特殊成分过敏原",
            "不清楚时先问主办",
        ]
    }

    /// 共享礼仪
    pub fn share(&self) -> Vec<&'static str> {
        vec![
            "取餐适量不哄抢",
            "用公用餐具分取",
            "品尝多种不挑食",
            "让长辈优先取用",
        ]
    }

    /// 参与协助
    pub fn help(&self) -> Vec<&'static str> {
        vec![
            "主动帮忙摆放收拾",
            "不只顾清空美食",
            "协助清理桌面",
            "感谢主人与厨艺",
        ]
    }

    /// 氛围和谐
    pub fn atmosphere(&self) -> Vec<&'static str> {
        vec![
            "控制量不浪费",
            "宾主互相尊重",
            "适合食品分流余食",
            "愉快闲聊融洽气氛",
        ]
    }
}

impl Rule for PotluckEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("potluck")
    }

    fn explain(&self) -> String {
        format!(
            "【家常聚餐礼仪】\n{}",
            [
                format!(
                    "携物约定：\\n{}",
                    self.bring()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "共享礼仪：\\n{}",
                    self.share()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "参与协助：\\n{}",
                    self.help()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "氛围和谐：\\n{}",
                    self.atmosphere()
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
    fn test_potlucketiquetterules_basic() {
        let rules = PotluckEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "家常聚餐礼仪");
        assert!(!rules.bring().is_empty());
        assert!(!rules.share().is_empty());
        assert!(!rules.help().is_empty());
        assert!(!rules.atmosphere().is_empty());
    }

    #[test]
    fn test_potlucketiquetterules_validation() {
        let rules = PotluckEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("potluck"));
    }

    #[test]
    fn test_potlucketiquetterules_explain() {
        let rules = PotluckEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("携物约定"));
        assert!(e.contains("共享礼仪"));
        assert!(e.contains("参与协助"));
    }
}
