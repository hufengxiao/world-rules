//! 红酒礼仪
//!
//! 涵盖红酒品鉴的礼仪规范，包括选酒、开瓶、品饮、配餐等礼仪。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WineTastingRules,
    name: "红酒礼仪",
    desc: "红酒品鉴礼仪规范，包括选酒、开瓶、品饮、配餐等",
    origin: "西方",
    tags: ["社交", "礼仪", "餐饮", "红酒"]
}

impl WineTastingRules {
    /// 选酒礼仪
    pub fn wine_selection(&self) -> Vec<&'static str> {
        vec![
            "了解餐厅酒单",
            "询问侍酒师建议",
            "考虑配餐需求",
            "预算范围内选择",
            "可以先品尝小样",
            "尊重主人选择",
            "了解产区特色",
            "适度选购年份酒",
        ]
    }

    /// 开瓶礼仪
    pub fn opening(&self) -> Vec<&'static str> {
        vec![
            "让侍酒师开瓶",
            "观察开瓶过程",
            "检查瓶塞状态",
            "不要自己强行开瓶",
            "等待侍酒师倒酒",
            "欣赏开瓶仪式",
            "检查瓶口清洁",
            "注意酒瓶温度",
        ]
    }

    /// 品酒礼仪
    pub fn tasting(&self) -> Vec<&'static str> {
        vec![
            "观色：观察酒液色泽",
            "摇杯：轻轻摇晃醒酒",
            "闻香：品味酒香层次",
            "品尝：小口品尝风味",
            "不要一口喝完",
            "体会酒体结构",
            "可以品后吐酒",
            "品酒记录心得",
        ]
    }

    /// 持杯礼仪
    pub fn glass_holding(&self) -> Vec<&'static str> {
        vec![
            "持杯脚或杯底",
            "不要握杯身",
            "避免改变酒温度",
            "持杯姿势优雅",
            "不要摇晃过度",
            "杯口保持清洁",
            "不要用杯碰撞",
            "轻轻放下酒杯",
        ]
    }

    /// 配餐礼仪
    pub fn pairing(&self) -> Vec<&'static str> {
        vec![
            "红酒配红肉",
            "白酒配白肉",
            "轻酒配清淡菜肴",
            "重酒配浓郁菜肴",
            "甜酒配甜点",
            "气泡酒配开胃菜",
            "考虑菜肴风味",
            "尝试创意搭配",
        ]
    }

    /// 饮酒礼仪
    pub fn drinking(&self) -> Vec<&'static str> {
        vec![
            "适量饮酒",
            "不要强迫他人",
            "敬酒时目光交流",
            "碰杯时杯沿低于长者",
            "可以拒绝饮酒",
            "保持清醒社交",
            "不要过度饮酒",
            "酒后不要驾车",
        ]
    }

    /// 存酒礼仪
    pub fn storage(&self) -> Vec<&'static str> {
        vec![
            "温度适宜存放",
            "避免光照直射",
            "湿度适中",
            "横放或斜放",
            "避免震动",
            "定期检查",
            "不要过早开封",
            "了解保质期",
        ]
    }

    /// 餐厅礼仪
    pub fn restaurant(&self) -> Vec<&'static str> {
        vec![
            "让侍酒师服务",
            "不要自己倒酒",
            "可以要求换酒",
            "适度选购酒品",
            "可以分享酒品",
            "感谢侍酒师",
            "支付酒水费用",
            "不要带走餐厅酒",
        ]
    }

    /// 禁忌事项
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "不要一口喝完",
            "不要握杯身",
            "不要过度摇晃",
            "不要强迫他人饮酒",
            "不要酒后失态",
            "不要大声讨论酒",
            "不要批评他人选择",
            "不要酒后驾车",
        ]
    }

    /// 文化知识
    pub fn wine_knowledge(&self) -> Vec<&'static str> {
        vec![
            "了解产区特色",
            "学习葡萄品种",
            "了解年份影响",
            "体会酿造工艺",
            "学习品酒技巧",
            "了解酒庄历史",
            "掌握评分标准",
            "分享酒文化",
        ]
    }
}

impl Rule for WineTastingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("dining")
    }

    fn explain(&self) -> String {
        format!(
            "【红酒礼仪】\n\n\
            选酒礼仪：\n{}\n\n\
            开瓶礼仪：\n{}\n\n\
            品酒礼仪：\n{}\n\n\
            持杯礼仪：\n{}\n\n\
            配餐礼仪：\n{}\n\n\
            饮酒礼仪：\n{}\n\n\
            存酒礼仪：\n{}\n\n\
            餐厅礼仪：\n{}\n\n\
            禁忌事项：\n{}\n\n\
            文化知识：\n{}",
            self.wine_selection()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.opening()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.tasting()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.glass_holding()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.pairing()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.drinking()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.storage()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.restaurant()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.taboos()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.wine_knowledge()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_wine_tasting_rules() {
        let rules = WineTastingRules::new();
        assert_eq!(rules.metadata().name, "红酒礼仪");
        assert!(!rules.wine_selection().is_empty());
        assert!(!rules.opening().is_empty());
        assert!(!rules.tasting().is_empty());
        assert!(!rules.glass_holding().is_empty());
        assert!(!rules.pairing().is_empty());
        assert!(!rules.drinking().is_empty());
        assert!(!rules.storage().is_empty());
        assert!(!rules.restaurant().is_empty());
        assert!(!rules.taboos().is_empty());
        assert!(!rules.wine_knowledge().is_empty());
    }

    #[test]
    fn test_wine_tasting_validation() {
        let rules = WineTastingRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("dining"));
    }

    #[test]
    fn test_wine_tasting_explain() {
        let rules = WineTastingRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("选酒礼仪"));
        assert!(explanation.contains("品酒礼仪"));
        assert!(explanation.contains("配餐礼仪"));
        assert!(explanation.contains("禁忌事项"));
    }
}
