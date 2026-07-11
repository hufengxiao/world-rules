//! 鸡尾酒会礼仪
//!
//! 涵盖鸡尾酒会的礼仪规范，包括入场、社交、饮酒、小食等礼仪。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: CocktailPartyRules,
    name: "鸡尾酒会礼仪",
    desc: "鸡尾酒会礼仪规范，包括入场、社交、饮酒、小食等",
    origin: "西方",
    tags: ["社交", "礼仪", "餐饮", "酒会"]
}

impl CocktailPartyRules {
    /// 入场礼仪
    pub fn entrance(&self) -> Vec<&'static str> {
        vec![
            "穿着时尚得体",
            "准时到达",
            "签到或登记",
            "领取酒会资料",
            "问候接待人员",
            "熟悉场地布局",
            "找到饮品和食物区域",
            "开始社交互动",
        ]
    }

    /// 餐饮礼仪
    pub fn dining(&self) -> Vec<&'static str> {
        vec![
            "适量取用小食",
            "一手持杯，一手取食",
            "使用小盘子或餐巾",
            "不要一次取太多",
            "边吃边社交",
            "保持餐桌整洁",
            "小食分享给他人",
            "吃完后清空盘子",
        ]
    }

    /// 饮酒礼仪
    pub fn drinking(&self) -> Vec<&'static str> {
        vec![
            "适量饮酒",
            "可以选择非酒精饮品",
            "持杯姿势优雅",
            "不要强迫他人饮酒",
            "轮流尝试不同鸡尾酒",
            "了解酒品特色",
            "不要过度饮酒",
            "保持清醒社交",
        ]
    }

    /// 社交礼仪
    pub fn networking(&self) -> Vec<&'static str> {
        vec![
            "主动与人交谈",
            "交换名片或联系方式",
            "礼貌介绍自己",
            "倾听他人发言",
            "不要只与熟人交谈",
            "轮流结识不同客人",
            "保持友好开放态度",
            "适时结束对话",
        ]
    }

    /// 交谈礼仪
    pub fn conversation(&self) -> Vec<&'static str> {
        vec![
            "选择轻松话题",
            "音量适中",
            "不要谈论敏感话题",
            "保持眼神交流",
            "询问对方背景",
            "分享有趣经历",
            "不要垄断对话",
            "轮流发言",
        ]
    }

    /// 自助小食礼仪
    pub fn finger_food(&self) -> Vec<&'static str> {
        vec![
            "取用手指食物",
            "一口大小食用",
            "不要用手直接抓取",
            "使用牙签或小叉",
            "保持手指清洁",
            "不要遗留食物残渣",
            "品尝不同风味",
            "感谢厨师准备",
        ]
    }

    /// 舞蹈礼仪
    pub fn dancing(&self) -> Vec<&'static str> {
        vec![
            "可以参与轻松舞蹈",
            "邀请他人跳舞",
            "接受或礼貌拒绝",
            "保持舞池秩序",
            "不要过度张扬",
            "尊重他人空间",
            "保持优雅舞姿",
            "适时退出舞池",
        ]
    }

    /// 合影礼仪
    pub fn photo(&self) -> Vec<&'static str> {
        vec![
            "可以合影留念",
            "征得他人同意",
            "姿势自然友好",
            "不要频繁拍照",
            "分享照片征得同意",
            "感谢摄影师",
            "照片不要过于夸张",
            "保存美好回忆",
        ]
    }

    /// 告别礼仪
    pub fn farewell(&self) -> Vec<&'static str> {
        vec![
            "适时告别离场",
            "感谢主办方",
            "与结识的人道别",
            "不要过早离开",
            "不要拖延告别",
            "带走个人物品",
            "发送感谢信息",
            "保持后续联系",
        ]
    }

    /// 禁忌事项
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "不要醉酒失态",
            "不要强行推销",
            "不要谈论争议话题",
            "不要过度喧哗",
            "不要占用他人时间",
            "不要频繁看手机",
            "不要穿着过于暴露",
            "不要带走酒会物品",
        ]
    }
}

impl Rule for CocktailPartyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("dining")
    }

    fn explain(&self) -> String {
        format!(
            "【鸡尾酒会礼仪】\n\n\
            入场礼仪：\n{}\n\n\
            餐饮礼仪：\n{}\n\n\
            饮酒礼仪：\n{}\n\n\
            社交礼仪：\n{}\n\n\
            交谈礼仪：\n{}\n\n\
            自助小食礼仪：\n{}\n\n\
            舞蹈礼仪：\n{}\n\n\
            合影礼仪：\n{}\n\n\
            告别礼仪：\n{}\n\n\
            禁忌事项：\n{}",
            self.entrance()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.dining()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.drinking()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.networking()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.conversation()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.finger_food()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.dancing()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.photo()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.farewell()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.taboos()
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
    fn test_cocktail_party_rules() {
        let rules = CocktailPartyRules::new();
        assert_eq!(rules.metadata().name, "鸡尾酒会礼仪");
        assert!(!rules.entrance().is_empty());
        assert!(!rules.dining().is_empty());
        assert!(!rules.drinking().is_empty());
        assert!(!rules.networking().is_empty());
        assert!(!rules.conversation().is_empty());
        assert!(!rules.finger_food().is_empty());
        assert!(!rules.dancing().is_empty());
        assert!(!rules.photo().is_empty());
        assert!(!rules.farewell().is_empty());
        assert!(!rules.taboos().is_empty());
    }

    #[test]
    fn test_cocktail_party_validation() {
        let rules = CocktailPartyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("dining"));
    }

    #[test]
    fn test_cocktail_party_explain() {
        let rules = CocktailPartyRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("入场礼仪"));
        assert!(explanation.contains("饮酒礼仪"));
        assert!(explanation.contains("社交礼仪"));
        assert!(explanation.contains("禁忌事项"));
    }
}
