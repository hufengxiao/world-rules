//! 正式宴会礼仪
//!
//! 涵盖正式宴会的礼仪规范，包括入场、座位、致辞、用餐等流程礼仪。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FormalBanquetRules,
    name: "正式宴会礼仪",
    desc: "正式宴会礼仪规范，包括入场、座位、致辞、用餐等流程",
    origin: "国际通用",
    tags: ["社交", "礼仪", "餐饮", "宴会"]
}

impl FormalBanquetRules {
    /// 入场礼仪
    pub fn entrance(&self) -> Vec<&'static str> {
        vec![
            "提前到达，不要迟到",
            "着装正式得体",
            "签到并领取座位号",
            "按指示进入宴会厅",
            "礼貌问候接待人员",
            "不要携带不适当的物品",
            "检查座位安排",
            "等待引导入场",
        ]
    }

    /// 座位礼仪
    pub fn seating(&self) -> Vec<&'static str> {
        vec![
            "按照安排入座，不要擅自换位",
            "等待主人或主宾入座",
            "坐姿端正优雅",
            "不要占用他人座位",
            "座位卡片放在桌面上",
            "与邻座保持友好",
            "餐前不要将物品放在桌上",
            "入座后整理衣着",
        ]
    }

    /// 致辞礼仪
    pub fn speech(&self) -> Vec<&'static str> {
        vec![
            "认真聆听致辞",
            "保持安静",
            "不要在致辞时用餐",
            "适时鼓掌",
            "不要打断致辞",
            "保持专注",
            "不要看手机",
            "重要信息可以记录",
        ]
    }

    /// 用餐流程礼仪
    pub fn dining_process(&self) -> Vec<&'static str> {
        vec![
            "等待所有人上菜后再开始",
            "按照上菜顺序用餐",
            "与同桌保持同步",
            "细嚼慢咽",
            "不要急于用餐",
            "保持餐桌整洁",
            "不要大声喧哗",
            "适量取用食物",
        ]
    }

    /// 敬酒礼仪
    pub fn toast(&self) -> Vec<&'static str> {
        vec![
            "等待主人先敬酒",
            "敬酒时站起身（视场合）",
            "持杯姿势正确",
            "敬酒词简洁得体",
            "碰杯时目光注视对方",
            "杯沿低于主宾",
            "不要强迫他人饮酒",
            "保持适量饮酒",
        ]
    }

    /// 交谈礼仪
    pub fn conversation(&self) -> Vec<&'static str> {
        vec![
            "选择轻松话题",
            "与左右邻座交谈",
            "轮流发言",
            "保持音量适中",
            "避免争议话题",
            "倾听他人发言",
            "不要只与一人交谈",
            "保持礼貌友好",
        ]
    }

    /// 谢礼礼仪
    pub fn gratitude(&self) -> Vec<&'static str> {
        vec![
            "宴会结束后向主人致谢",
            "感谢主办方安排",
            "感谢同桌客人",
            "可以赠送感谢卡片",
            "不要过早离场",
            "离场时有序退出",
            "不要带走宴会物品",
            "保持得体风度",
        ]
    }

    /// 礼物礼仪
    pub fn gift(&self) -> Vec<&'static str> {
        vec![
            "可以赠送适当的礼物",
            "礼物包装精美",
            "不要过于奢华",
            "符合场合性质",
            "递交礼物时双手",
            "附上贺卡或感谢词",
            "礼物不当场打开",
            "礼物放在指定位置",
        ]
    }

    /// 摄影礼仪
    pub fn photography(&self) -> Vec<&'static str> {
        vec![
            "遵守宴会摄影规定",
            "不要频繁拍照影响他人",
            "重要环节可以拍摄",
            "与人合影前征得同意",
            "不要使用闪光灯",
            "可以请专业摄影师",
            "照片分享征得同意",
            "不要拍摄用餐细节",
        ]
    }

    /// 离场礼仪
    pub fn departure(&self) -> Vec<&'static str> {
        vec![
            "等待宴会正式结束",
            "向主人和主办方致谢",
            "有序离场",
            "不要提前离开",
            "带走个人物品",
            "保持得体形象",
            "离场后发送感谢短信",
            "可以后续赠送礼物",
        ]
    }
}

impl Rule for FormalBanquetRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("dining")
    }

    fn explain(&self) -> String {
        format!(
            "【正式宴会礼仪】\n\n\
            入场礼仪：\n{}\n\n\
            座位礼仪：\n{}\n\n\
            致辞礼仪：\n{}\n\n\
            用餐流程礼仪：\n{}\n\n\
            敬酒礼仪：\n{}\n\n\
            交谈礼仪：\n{}\n\n\
            谢礼礼仪：\n{}\n\n\
            礼物礼仪：\n{}\n\n\
            摄影礼仪：\n{}\n\n\
            离场礼仪：\n{}",
            self.entrance()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.seating()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.speech()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.dining_process()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.toast()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.conversation()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.gratitude()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.gift()
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
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_formal_banquet_rules() {
        let rules = FormalBanquetRules::new();
        assert_eq!(rules.metadata().name, "正式宴会礼仪");
        assert!(!rules.entrance().is_empty());
        assert!(!rules.seating().is_empty());
        assert!(!rules.speech().is_empty());
        assert!(!rules.dining_process().is_empty());
        assert!(!rules.toast().is_empty());
        assert!(!rules.conversation().is_empty());
        assert!(!rules.gratitude().is_empty());
        assert!(!rules.gift().is_empty());
        assert!(!rules.photography().is_empty());
        assert!(!rules.departure().is_empty());
    }

    #[test]
    fn test_formal_banquet_validation() {
        let rules = FormalBanquetRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("dining"));
    }

    #[test]
    fn test_formal_banquet_explain() {
        let rules = FormalBanquetRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("入场礼仪"));
        assert!(explanation.contains("座位礼仪"));
        assert!(explanation.contains("致辞礼仪"));
        assert!(explanation.contains("离场礼仪"));
    }
}