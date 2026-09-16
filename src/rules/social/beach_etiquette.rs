//! 海滨沙滩礼仪
//!
//! 海边沙滩度假时的公共礼仪与安全约定

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BeachEtiquetteRules,
    name: "海滨沙滩礼仪",
    desc: "海边沙滩度假时的公共礼仪与安全约定",
    origin: "国际",
    tags: ["社交", "礼仪", "海滩", "度假"]
}

impl BeachEtiquetteRules {
    /// 沙滩占位
    pub fn beach_placeholder(&self) -> Vec<&'static str> {
        vec![
            "不用浴巾占过宽的沙滩空间",
            "尊重他人日晒空间的私人距离",
            "帐篷与遮阳伞不妨碍通行",
            "离开时带走随身垃圾与物品",
            "不随意踩踏他人浴巾与玩物",
        ]
    }

    /// 水上礼仪
    pub fn water(&self) -> Vec<&'static str> {
        vec![
            "游泳注意来往人群避免碰撞",
            "不向他人泼水挑衅",
            "带球入水注意周边游客",
            "浪大时看好儿童",
        ]
    }

    /// 音响与言行
    pub fn noise(&self) -> Vec<&'static str> {
        vec![
            "音乐播放音量避免干扰他人",
            "不在人群中高声喧哗",
            "管好儿童避免奔跑碰撞",
            "不在海滩抽烟熏人",
        ]
    }

    /// 安全与卫生
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "注意潮汐与救生员指示",
            "不把垃圾留在沙滩",
            "防晒并及时补水",
            "儿童须有大人陪同",
        ]
    }
}

impl Rule for BeachEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("beach")
    }

    fn explain(&self) -> String {
        let parts = vec![
            format!(
                "沙滩占位：\\n{}",
                self.beach_placeholder()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
            format!(
                "水上礼仪：\\n{}",
                self.water()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
            format!(
                "音响与言行：\\n{}",
                self.noise()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
            format!(
                "安全与卫生：\\n{}",
                self.safety()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
        ];
        format!("【海滨沙滩礼仪】\n{}", parts.join("\n\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_beachetiquetterules_basic() {
        let rules = BeachEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "海滨沙滩礼仪");
        assert!(!rules.beach_placeholder().is_empty());
        assert!(!rules.water().is_empty());
        assert!(!rules.noise().is_empty());
        assert!(!rules.safety().is_empty());
    }

    #[test]
    fn test_beachetiquetterules_validation() {
        let rules = BeachEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("beach"));
    }

    #[test]
    fn test_beachetiquetterules_explain() {
        let rules = BeachEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("沙滩占位"));
        assert!(e.contains("水上礼仪"));
        assert!(e.contains("音响与言行"));
    }
}
