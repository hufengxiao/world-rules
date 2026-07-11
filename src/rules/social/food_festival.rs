//! 美食节礼仪
//!
//! 涵盖美食节的礼仪规范，包括入场、品尝、社交、环保等礼仪。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FoodFestivalRules,
    name: "美食节礼仪",
    desc: "美食节礼仪规范，包括入场、品尝、社交、环保等",
    origin: "国际通用",
    tags: ["社交", "礼仪", "餐饮", "美食节"]
}

impl FoodFestivalRules {
    /// 入场礼仪
    pub fn entrance(&self) -> Vec<&'static str> {
        vec![
            "购买入场票",
            "遵守入场规定",
            "有序排队",
            "不要拥挤推搡",
            "保持入场整洁",
            "感谢工作人员",
            "领取活动资料",
            "了解活动安排",
        ]
    }

    /// 品尝礼仪
    pub fn tasting(&self) -> Vec<&'static str> {
        vec![
            "排队有序品尝",
            "少量品尝多品种",
            "不要浪费食物",
            "感谢摊主",
            "可以询问食材来源",
            "了解美食特色",
            "分享品尝心得",
            "尊重摊主文化",
        ]
    }

    /// 购买礼仪
    pub fn purchasing(&self) -> Vec<&'static str> {
        vec![
            "排队有序购买",
            "准备好付款方式",
            "确认购买内容",
            "感谢摊主",
            "不要讨价还价",
            "保持耐心等待",
            "支持摊主经营",
            "适度选购",
        ]
    }

    /// 社交礼仪
    pub fn networking(&self) -> Vec<&'static str> {
        vec![
            "与他人分享美食",
            "交流品尝心得",
            "推荐喜欢的摊位",
            "保持友好交流",
            "不要大声喧哗",
            "与摊主友好互动",
            "询问美食故事",
            "支持摊主文化",
        ]
    }

    /// 摊主礼仪
    pub fn vendor_interaction(&self) -> Vec<&'static str> {
        vec![
            "尊重摊主文化",
            "感谢摊主准备",
            "了解美食特色",
            "不要批评摊主",
            "可以询问制作方法",
            "适度品尝",
            "支持摊主经营",
            "购买时保持耐心",
        ]
    }

    /// 摄影礼仪
    pub fn photography(&self) -> Vec<&'static str> {
        vec![
            "可以拍摄美食",
            "征得摊主同意",
            "不要影响他人",
            "不要占用过多空间",
            "可以分享社交媒体",
            "感谢摊主配合",
            "不要使用闪光灯",
            "记录美好时刻",
        ]
    }

    /// 环保礼仪
    pub fn environment(&self) -> Vec<&'static str> {
        vec![
            "垃圾分类投放",
            "减少一次性用品",
            "自带餐具或杯子",
            "不要浪费食物",
            "保持活动场地整洁",
            "不要乱扔垃圾",
            "支持环保摊位",
            "提倡绿色消费",
        ]
    }

    /// 儿童礼仪
    pub fn children(&self) -> Vec<&'static str> {
        vec![
            "陪同儿童品尝",
            "教导儿童排队",
            "不要让儿童奔跑",
            "控制儿童品尝量",
            "照顾儿童需求",
            "儿童保持安静",
            "监督儿童行为",
            "儿童不要触摸摊位",
        ]
    }

    /// 禁忌事项
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "不要插队",
            "不要浪费食物",
            "不要大声喧哗",
            "不要乱扔垃圾",
            "不要批评摊主",
            "不要占用摊位空间",
            "不要强迫摊主",
            "不要带走摊位物品",
        ]
    }

    /// 离场礼仪
    pub fn departure(&self) -> Vec<&'static str> {
        vec![
            "清理个人垃圾",
            "感谢摊主和主办方",
            "带走个人物品",
            "有序离场",
            "保持场地整洁",
            "可以发送感谢信息",
            "支持摊主后续",
            "分享美好体验",
        ]
    }
}

impl Rule for FoodFestivalRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("dining")
    }

    fn explain(&self) -> String {
        format!(
            "【美食节礼仪】\n\n\
            入场礼仪：\n{}\n\n\
            品尝礼仪：\n{}\n\n\
            购买礼仪：\n{}\n\n\
            社交礼仪：\n{}\n\n\
            摊主礼仪：\n{}\n\n\
            摄影礼仪：\n{}\n\n\
            环保礼仪：\n{}\n\n\
            儿童礼仪：\n{}\n\n\
            禁忌事项：\n{}\n\n\
            离场礼仪：\n{}",
            self.entrance()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.tasting()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.purchasing()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.networking()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.vendor_interaction()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.photography()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.environment()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.children()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.taboos()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.departure()
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
    fn test_food_festival_rules() {
        let rules = FoodFestivalRules::new();
        assert_eq!(rules.metadata().name, "美食节礼仪");
        assert!(!rules.entrance().is_empty());
        assert!(!rules.tasting().is_empty());
        assert!(!rules.purchasing().is_empty());
        assert!(!rules.networking().is_empty());
        assert!(!rules.vendor_interaction().is_empty());
        assert!(!rules.photography().is_empty());
        assert!(!rules.environment().is_empty());
        assert!(!rules.children().is_empty());
        assert!(!rules.taboos().is_empty());
        assert!(!rules.departure().is_empty());
    }

    #[test]
    fn test_food_festival_validation() {
        let rules = FoodFestivalRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("dining"));
    }

    #[test]
    fn test_food_festival_explain() {
        let rules = FoodFestivalRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("入场礼仪"));
        assert!(explanation.contains("品尝礼仪"));
        assert!(explanation.contains("社交礼仪"));
        assert!(explanation.contains("禁忌事项"));
    }
}
