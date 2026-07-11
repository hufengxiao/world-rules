//! 咖啡厅礼仪
//!
//! 涵盖咖啡厅的礼仪规范，包括入座、点餐、品饮、交谈等礼仪。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CoffeeHouseRules,
    name: "咖啡厅礼仪",
    desc: "咖啡厅礼仪规范，包括入座、点餐、品饮、交谈等",
    origin: "国际通用",
    tags: ["社交", "礼仪", "餐饮", "咖啡"]
}

impl CoffeeHouseRules {
    /// 入座礼仪
    pub fn seating(&self) -> Vec<&'static str> {
        vec![
            "选择合适的座位",
            "不要占用过多空间",
            "尊重他人座位选择",
            "不要在繁忙时段久坐",
            "座位保持整洁",
            "不要将物品放在空座位上",
            "等待引导入座",
            "不要随意换座位",
        ]
    }

    /// 点餐礼仪
    pub fn ordering(&self) -> Vec<&'static str> {
        vec![
            "了解咖啡品种",
            "礼貌询问服务员",
            "选择适合的饮品",
            "可以要求定制",
            "点餐时音量适中",
            "确认订单无误",
            "等待耐心",
            "感谢服务员",
        ]
    }

    /// 品饮礼仪
    pub fn drinking(&self) -> Vec<&'static str> {
        vec![
            "品尝咖啡风味",
            "不要一口喝完",
            "欣赏咖啡香气",
            "可以加糖或奶",
            "咖啡温度适宜",
            "不要大声喝咖啡",
            "杯碟保持整洁",
            "品味咖啡文化",
        ]
    }

    /// 交谈礼仪
    pub fn conversation(&self) -> Vec<&'static str> {
        vec![
            "保持音量适中",
            "不要大声喧哗",
            "选择轻松话题",
            "尊重他人隐私",
            "不要在公共区域谈论敏感话题",
            "与朋友保持友好交流",
            "不要占用公共电话",
            "保持咖啡馆氛围",
        ]
    }

    /// 工作礼仪
    pub fn working(&self) -> Vec<&'static str> {
        vec![
            "不要占用座位太久",
            "保持工作区域整洁",
            "不要大声讨论工作",
            "使用耳机听音频",
            "不要占用过多电源",
            "适度购买饮品",
            "尊重他人工作空间",
            "高峰时段适当离开",
        ]
    }

    /// 网络礼仪
    pub fn internet(&self) -> Vec<&'static str> {
        vec![
            "遵守网络使用规定",
            "不要下载大文件",
            "保护个人信息",
            "不要浏览不适当内容",
            "不要长时间占用网络",
            "尊重他人网络使用",
            "适时断开连接",
            "感谢免费网络服务",
        ]
    }

    /// 拍照礼仪
    pub fn photography(&self) -> Vec<&'static str> {
        vec![
            "可以拍摄咖啡和糕点",
            "不要频繁拍照",
            "不要拍摄他人",
            "遵守咖啡馆规定",
            "不要使用闪光灯",
            "分享照片征得同意",
            "不要占用拍照空间",
            "记录美好时刻",
        ]
    }

    /// 离座礼仪
    pub fn departure(&self) -> Vec<&'static str> {
        vec![
            "收拾个人物品",
            "清理桌面",
            "归还借用的物品",
            "感谢服务员",
            "不要留下垃圾",
            "座位留给其他客人",
            "有序离开",
            "保持得体形象",
        ]
    }

    /// 礼仪禁忌
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "不要大声喧哗",
            "不要占用座位太久",
            "不要带走咖啡馆物品",
            "不要吸烟（室内禁止）",
            "不要在高峰时段久坐",
            "不要随意触碰他人",
            "不要在座位上睡觉",
            "不要带宠物（除非允许）",
        ]
    }

    /// 环境保护
    pub fn environment(&self) -> Vec<&'static str> {
        vec![
            "减少一次性杯子使用",
            "自带杯子可以享优惠",
            "垃圾分类投放",
            "节约用水",
            "爱护咖啡馆环境",
            "不要浪费食物",
            "提倡环保消费",
            "支持绿色咖啡厅",
        ]
    }
}

impl Rule for CoffeeHouseRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("dining")
    }

    fn explain(&self) -> String {
        format!(
            "【咖啡厅礼仪】\n\n\
            入座礼仪：\n{}\n\n\
            点餐礼仪：\n{}\n\n\
            品饮礼仪：\n{}\n\n\
            交谈礼仪：\n{}\n\n\
            工作礼仪：\n{}\n\n\
            网络礼仪：\n{}\n\n\
            拍照礼仪：\n{}\n\n\
            离座礼仪：\n{}\n\n\
            礼仪禁忌：\n{}\n\n\
            环境保护：\n{}",
            self.seating()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.ordering()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.drinking()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.conversation()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.working()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.internet()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.photography()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.departure()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.taboos()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.environment()
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
    fn test_coffee_house_rules() {
        let rules = CoffeeHouseRules::new();
        assert_eq!(rules.metadata().name, "咖啡厅礼仪");
        assert!(!rules.seating().is_empty());
        assert!(!rules.ordering().is_empty());
        assert!(!rules.drinking().is_empty());
        assert!(!rules.conversation().is_empty());
        assert!(!rules.working().is_empty());
        assert!(!rules.internet().is_empty());
        assert!(!rules.photography().is_empty());
        assert!(!rules.departure().is_empty());
        assert!(!rules.taboos().is_empty());
        assert!(!rules.environment().is_empty());
    }

    #[test]
    fn test_coffee_house_validation() {
        let rules = CoffeeHouseRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("dining"));
    }

    #[test]
    fn test_coffee_house_explain() {
        let rules = CoffeeHouseRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("入座礼仪"));
        assert!(explanation.contains("品饮礼仪"));
        assert!(explanation.contains("交谈礼仪"));
        assert!(explanation.contains("礼仪禁忌"));
    }
}
