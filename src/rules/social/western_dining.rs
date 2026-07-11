//! 西餐礼仪
//!
//! 涵盖西方餐桌礼仪的详细规范，包括餐具使用、用餐顺序、餐桌礼仪等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WesternDiningRules,
    name: "西餐礼仪",
    desc: "西方餐桌礼仪详细规范，包括餐具使用、用餐顺序、餐桌礼仪等",
    origin: "西方",
    tags: ["社交", "礼仪", "餐饮", "西餐"]
}

impl WesternDiningRules {
    /// 餐具摆放礼仪
    pub fn table_setting(&self) -> Vec<&'static str> {
        vec![
            "餐具从外向内依次使用",
            "刀叉摆放：叉在左，刀在右，刀刃朝内",
            "勺子放在右侧，甜点勺在上方",
            "面包盘放在左侧，黄油刀放在盘上",
            "水杯放在餐刀上方，酒杯放在右侧",
            "餐巾放在盘子下方或左侧",
            "甜点餐具横放在餐盘上方",
            "咖啡杯碟放在右侧",
        ]
    }

    /// 用餐顺序礼仪
    pub fn course_order(&self) -> Vec<&'static str> {
        vec![
            "开胃菜（Appetizer）- 唤醒味蕾",
            "汤（Soup）- 温暖开场",
            "前菜（Entrée）- 轻盈过渡",
            "主菜（Main Course）- 正式主角",
            "沙拉（Salad）- 清爽间歇（欧洲顺序）",
            "奶酪（Cheese Course）- 风味体验",
            "甜点（Dessert）- 甜蜜收尾",
            "咖啡或茶（Coffee/Tea）- 完美结束",
        ]
    }

    /// 刀叉使用礼仪
    pub fn utensil_etiquette(&self) -> Vec<&'static str> {
        vec![
            "握刀叉时不要握得太紧",
            "切割食物时动作轻柔",
            "一次切一口大小的食物",
            "用餐中途休息时刀叉呈'V'字形摆放",
            "用餐完毕时刀叉并排放在盘中",
            "刀刃朝内，叉齿朝上",
            "不要用刀将食物推向叉子",
            "不要挥舞刀叉说话",
        ]
    }

    /// 餐巾礼仪
    pub fn napkin_etiquette(&self) -> Vec<&'static str> {
        vec![
            "入座后展开餐巾放在膝盖上",
            "不要将餐巾塞在领口",
            "用餐中途离席时餐巾放在椅子上",
            "用餐结束时将餐巾松松地放在桌上左侧",
            "不要用餐巾擦桌子或餐具",
            "餐巾用于轻拭嘴唇",
            "不要将餐巾团成一团",
            "正式场合餐巾应保持整洁",
        ]
    }

    /// 进餐礼仪
    pub fn dining_manners(&self) -> Vec<&'static str> {
        vec![
            "等待所有人上菜后再开始用餐",
            "主人开始用餐后才能动筷",
            "细嚼慢咽，不要发出声音",
            "咀嚼时嘴巴闭紧",
            "不要说话时口中有食物",
            "口中有食物时不要喝水",
            "不要伸手越过他人取菜",
            "请他人传递远处食物",
        ]
    }

    /// 饮酒礼仪
    pub fn wine_etiquette(&self) -> Vec<&'static str> {
        vec![
            "持杯脚或杯底，不要握杯身",
            "红酒配红肉，白酒配白肉",
            "品酒：观色、摇杯、闻香、品尝",
            "不要一口喝完",
            "敬酒时目光注视对方",
            "碰杯时杯沿低于长者",
            "不强迫他人饮酒",
            "保持适量，不醉酒失态",
        ]
    }

    /// 面包礼仪
    pub fn bread_etiquette(&self) -> Vec<&'static str> {
        vec![
            "面包放在左侧面包盘上",
            "用手撕成小块食用",
            "不要用刀切面包",
            "不要整块咬食面包",
            "可以涂黄油或果酱",
            "不要将面包浸在汤中",
            "面包是配菜，不是主菜",
            "不要浪费面包",
        ]
    }

    /// 汤品礼仪
    pub fn soup_etiquette(&self) -> Vec<&'static str> {
        vec![
            "勺子由内向外舀汤",
            "不要吹汤降温",
            "汤少时可略微倾斜汤碗",
            "不要直接从汤碗喝汤",
            "喝汤时不要发出声音",
            "勺子放在汤碗下方的碟子上",
            "汤碗不要端起来",
            "用完后勺子放在汤碗中",
        ]
    }

    /// 交谈礼仪
    pub fn conversation(&self) -> Vec<&'static str> {
        vec![
            "选择轻松愉快的话题",
            "避免谈论争议性话题",
            "不要在餐桌上谈论工作",
            "与左右邻座交谈",
            "音量适中",
            "不要打断他人说话",
            "用餐时避免使用手机",
            "礼貌地倾听他人",
        ]
    }

    /// 离席礼仪
    pub fn departure(&self) -> Vec<&'static str> {
        vec![
            "等待主人示意用餐结束",
            "向主人表示感谢",
            "餐巾放在桌上左侧",
            "不要立即离席",
            "与同桌客人道别",
            "感谢主人的款待",
            "可发送感谢短信或卡片",
            "适时赠送小礼物表示感谢",
        ]
    }
}

impl Rule for WesternDiningRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("dining")
    }

    fn explain(&self) -> String {
        format!(
            "【西餐礼仪】\n\n\
            餐具摆放礼仪：\n{}\n\n\
            用餐顺序礼仪：\n{}\n\n\
            刀叉使用礼仪：\n{}\n\n\
            餐巾礼仪：\n{}\n\n\
            进餐礼仪：\n{}\n\n\
            饮酒礼仪：\n{}\n\n\
            面包礼仪：\n{}\n\n\
            汤品礼仪：\n{}\n\n\
            交谈礼仪：\n{}\n\n\
            离席礼仪：\n{}",
            self.table_setting()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.course_order()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.utensil_etiquette()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.napkin_etiquette()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.dining_manners()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.wine_etiquette()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.bread_etiquette()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.soup_etiquette()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.conversation()
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
    fn test_western_dining_rules() {
        let rules = WesternDiningRules::new();
        assert_eq!(rules.metadata().name, "西餐礼仪");
        assert!(!rules.table_setting().is_empty());
        assert!(!rules.course_order().is_empty());
        assert!(!rules.utensil_etiquette().is_empty());
        assert!(!rules.napkin_etiquette().is_empty());
        assert!(!rules.dining_manners().is_empty());
        assert!(!rules.wine_etiquette().is_empty());
        assert!(!rules.bread_etiquette().is_empty());
        assert!(!rules.soup_etiquette().is_empty());
        assert!(!rules.conversation().is_empty());
        assert!(!rules.departure().is_empty());
    }

    #[test]
    fn test_western_dining_validation() {
        let rules = WesternDiningRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("dining"));
    }

    #[test]
    fn test_western_dining_explain() {
        let rules = WesternDiningRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("餐具摆放礼仪"));
        assert!(explanation.contains("用餐顺序礼仪"));
        assert!(explanation.contains("刀叉使用礼仪"));
        assert!(explanation.contains("餐巾礼仪"));
    }
}
