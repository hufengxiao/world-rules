//! 在线游戏礼仪
//!
//! 网络游戏与电竞对局中的文明竞技与交流礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: GamingEtiquetteRules,
    name: "在线游戏礼仪",
    desc: "网络游戏与电竞对局中的文明竞技与交流礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "游戏", "电竞", "在线"]
}

impl GamingEtiquetteRules {
    /// 开局礼仪
    pub fn start(&self) -> Vec<&'static str> {
        vec![
            "开局前彼此问好表达尊重",
            "选择角色或线路先沟通",
            "组队时说明自己水平",
            "不对新手嘲讽",
        ]
    }

    /// 对局中
    pub fn during(&self) -> Vec<&'static str> {
        vec![
            "不因失误责骂队友",
            "不发表攻击性歧视言论",
            "不刷屏或恶意挂机",
            "配合团队战术而非一味逞强",
            "不使用外挂与作弊程序",
        ]
    }

    /// 输赢心态
    pub fn mindset(&self) -> Vec<&'static str> {
        vec![
            "输局不抱怨归咎他人",
            "赢局不嘲讽对手",
            "尊重对手的技巧与努力",
            "复盘讨论用平和语气",
        ]
    }

    /// 团队沟通
    pub fn communication(&self) -> Vec<&'static str> {
        vec![
            "使用清晰简洁的指挥提醒",
            "不一失误就否定队友",
            "佩戴耳机不打扰他人",
            "鼓励新手并耐心指导",
        ]
    }
}

impl Rule for GamingEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("gaming")
    }

    fn explain(&self) -> String {
        format!(
            "【在线游戏礼仪】\n{}",
            [
                format!(
                    "开局礼仪：\\n{}",
                    self.start()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "对局中：\\n{}",
                    self.during()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "输赢心态：\\n{}",
                    self.mindset()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "团队沟通：\\n{}",
                    self.communication()
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
    fn test_gamingetiquetterules_basic() {
        let rules = GamingEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "在线游戏礼仪");
        assert!(!rules.start().is_empty());
        assert!(!rules.during().is_empty());
        assert!(!rules.mindset().is_empty());
        assert!(!rules.communication().is_empty());
    }

    #[test]
    fn test_gamingetiquetterules_validation() {
        let rules = GamingEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("gaming"));
    }

    #[test]
    fn test_gamingetiquetterules_explain() {
        let rules = GamingEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("开局礼仪"));
        assert!(e.contains("对局中"));
        assert!(e.contains("输赢心态"));
    }
}
