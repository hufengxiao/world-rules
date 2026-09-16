//! 菜市场与集市礼仪
//!
//! 菜市场、农贸市场赶集中的购物、议价与公共礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MarketEtiquetteRules,
    name: "菜市场与集市礼仪",
    desc: "菜市场、农贸市场赶集中的购物、议价与公共礼仪",
    origin: "大众",
    tags: ["社交", "礼仪", "市场", "集市", "买菜"]
}

impl MarketEtiquetteRules {
    /// 购物行为
    pub fn shopping(&self) -> Vec<&'static str> {
        vec![
            "遵守排队次序购买",
            "挑选货物轻拿轻放",
            "不大幅翻拣损坏商品",
            "称重后不随意更换",
        ]
    }

    /// 议价分寸
    pub fn bargain(&self) -> Vec<&'static str> {
        vec![
            "议价友好不争执",
            "尊重摊主定价",
            "不故意贬低货品压价",
            "成交后不与摊主纠缠",
        ]
    }

    /// 空间与通行
    pub fn space(&self) -> Vec<&'static str> {
        vec![
            "不长时间占用摊位前通道",
            "便道让行推车与老人",
            "不堵在路口讨价还价",
            "雨天注意防滑",
        ]
    }

    /// 公平与卫生
    pub fn fair(&self) -> Vec<&'static str> {
        vec![
            "不偷拿顺带食材",
            "确认计价透明",
            "保持摊档前整洁",
            "还原用过的秤盘",
        ]
    }
}

impl Rule for MarketEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("market")
    }

    fn explain(&self) -> String {
        format!(
            "【菜市场与集市礼仪】\n{}",
            [
                format!(
                    "购物行为：\\n{}",
                    self.shopping()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "议价分寸：\\n{}",
                    self.bargain()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "空间与通行：\\n{}",
                    self.space()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "公平与卫生：\\n{}",
                    self.fair()
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
    fn test_marketetiquetterules_basic() {
        let rules = MarketEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "菜市场与集市礼仪");
        assert!(!rules.shopping().is_empty());
        assert!(!rules.bargain().is_empty());
        assert!(!rules.space().is_empty());
        assert!(!rules.fair().is_empty());
    }

    #[test]
    fn test_marketetiquetterules_validation() {
        let rules = MarketEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("market"));
    }

    #[test]
    fn test_marketetiquetterules_explain() {
        let rules = MarketEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("购物行为"));
        assert!(e.contains("议价分寸"));
        assert!(e.contains("空间与通行"));
    }
}
