//! 自助餐礼仪
//!
//! 涵盖自助餐的礼仪规范，包括取餐、用餐、分享、秩序等礼仪。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BuffetDiningRules,
    name: "自助餐礼仪",
    desc: "自助餐礼仪规范，包括取餐、用餐、分享、秩序等",
    origin: "国际通用",
    tags: ["社交", "礼仪", "餐饮", "自助餐"]
}

impl BuffetDiningRules {
    /// 取餐礼仪
    pub fn serving(&self) -> Vec<&'static str> {
        vec![
            "排队有序取餐",
            "不要拥挤推搡",
            "使用干净的盘子",
            "一次取适量食物",
            "可以多次取餐",
            "不要一次取太多",
            "使用公勺公夹",
            "不要用手直接取餐",
        ]
    }

    /// 用餐礼仪
    pub fn dining(&self) -> Vec<&'static str> {
        vec![
            "返回座位用餐",
            "不要在取餐区停留",
            "细嚼慢咽",
            "品尝不同食物",
            "保持餐桌整洁",
            "与他人分享美食",
            "不要大声喧哗",
            "适量取用饮品",
        ]
    }

    /// 分享礼仪
    pub fn sharing(&self) -> Vec<&'static str> {
        vec![
            "与他人分享美食",
            "推荐喜欢的菜品",
            "询问他人喜好",
            "不占用过多食物",
            "留些给其他人",
            "感谢厨师准备",
            "介绍特色菜品",
            "一起品尝新食物",
        ]
    }

    /// 饮品礼仪
    pub fn beverages(&self) -> Vec<&'static str> {
        vec![
            "适量取用饮品",
            "使用杯子或玻璃杯",
            "不要浪费饮品",
            "可以尝试不同饮品",
            "注意饮品温度",
            "不要一次取太多杯",
            "用完后杯子放回",
            "保持饮品区域整洁",
        ]
    }

    /// 餐具礼仪
    pub fn utensils(&self) -> Vec<&'static str> {
        vec![
            "使用干净的餐具",
            "不要用手直接取食",
            "餐后餐具放回指定位置",
            "不要带走餐具",
            "刀叉使用得当",
            "盘子不要堆积过多",
            "保持餐具整洁",
            "用完后清理餐桌",
        ]
    }

    /// 卫生礼仪
    pub fn hygiene(&self) -> Vec<&'static str> {
        vec![
            "取餐前洗手",
            "不要用手接触食物",
            "咳嗽或打喷嚏远离食物",
            "不要舔舐餐具",
            "保持个人卫生",
            "不要触碰他人食物",
            "注意食物保质",
            "发现问题及时反馈",
        ]
    }

    /// 环保礼仪
    pub fn environment(&self) -> Vec<&'static str> {
        vec![
            "减少食物浪费",
            "不要取太多不吃完",
            "适量取餐",
            "减少一次性餐具使用",
            "垃圾分类投放",
            "爱护环境卫生",
            "节约用水",
            "提倡绿色用餐",
        ]
    }

    /// 时间礼仪
    pub fn timing(&self) -> Vec<&'static str> {
        vec![
            "在开放时间内用餐",
            "不要逗留太久",
            "给其他客人留空间",
            "高峰期快速取餐",
            "不要在取餐区停留",
            "用餐时间适中",
            "不要太早或太晚",
            "遵守餐厅规定",
        ]
    }

    /// 禁忌事项
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "不要插队",
            "不要用手直接取餐",
            "不要将食物带走",
            "不要浪费食物",
            "不要大声喧哗",
            "不要奔跑嬉戏",
            "不要触摸展示食物",
            "不要占用过多座位",
        ]
    }

    /// 儿童礼仪
    pub fn children(&self) -> Vec<&'static str> {
        vec![
            "陪同儿童取餐",
            "教导儿童用餐礼仪",
            "控制儿童取餐量",
            "帮助儿童用餐",
            "儿童不要奔跑",
            "儿童保持安静",
            "监督儿童行为",
            "照顾儿童需求",
        ]
    }
}

impl Rule for BuffetDiningRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("dining")
    }

    fn explain(&self) -> String {
        format!(
            "【自助餐礼仪】\n\n\
            取餐礼仪：\n{}\n\n\
            用餐礼仪：\n{}\n\n\
            分享礼仪：\n{}\n\n\
            饮品礼仪：\n{}\n\n\
            餐具礼仪：\n{}\n\n\
            卫生礼仪：\n{}\n\n\
            环保礼仪：\n{}\n\n\
            时间礼仪：\n{}\n\n\
            禁忌事项：\n{}\n\n\
            儿童礼仪：\n{}",
            self.serving()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.dining()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.sharing()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.beverages()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.utensils()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.hygiene()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.environment()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.timing()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.taboos()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.children()
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
    fn test_buffet_dining_rules() {
        let rules = BuffetDiningRules::new();
        assert_eq!(rules.metadata().name, "自助餐礼仪");
        assert!(!rules.serving().is_empty());
        assert!(!rules.dining().is_empty());
        assert!(!rules.sharing().is_empty());
        assert!(!rules.beverages().is_empty());
        assert!(!rules.utensils().is_empty());
        assert!(!rules.hygiene().is_empty());
        assert!(!rules.environment().is_empty());
        assert!(!rules.timing().is_empty());
        assert!(!rules.taboos().is_empty());
        assert!(!rules.children().is_empty());
    }

    #[test]
    fn test_buffet_dining_validation() {
        let rules = BuffetDiningRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("dining"));
    }

    #[test]
    fn test_buffet_dining_explain() {
        let rules = BuffetDiningRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("取餐礼仪"));
        assert!(explanation.contains("用餐礼仪"));
        assert!(explanation.contains("卫生礼仪"));
        assert!(explanation.contains("禁忌事项"));
    }
}