//! 商务名片礼仪
//!
//! 涵盖商务名片的设计、交换、使用和管理规范。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BusinessCardRules,
    name: "商务名片礼仪",
    desc: "商务名片的设计、交换、使用和管理规范",
    origin: "国际通用",
    tags: ["社交", "礼仪", "商务", "名片"]
}

impl BusinessCardRules {
    /// 名片设计原则
    pub fn design_principles(&self) -> Vec<&'static str> {
        vec![
            "信息准确完整",
            "字体清晰易读",
            "布局简洁专业",
            "颜色搭配协调",
            "纸张质量优良",
            "体现公司形象",
            "联系方式齐全",
            "职务信息准确",
        ]
    }

    /// 名片信息内容
    pub fn card_content(&self) -> Vec<&'static str> {
        vec![
            "姓名（中英文）",
            "职位/职称",
            "公司名称",
            "公司地址",
            "电话号码",
            "邮箱地址",
            "公司网站",
            "二维码（可选）",
            "公司标志",
            "社交媒体账号（相关时）",
        ]
    }

    /// 递交名片礼仪
    pub fn presenting(&self) -> Vec<&'static str> {
        vec![
            "双手持名片，文字朝向对方",
            "微微鞠躬表示尊重",
            "自报姓名和职务",
            "确保名片方向正确",
            "避免遮挡名片文字",
            "时机恰当（交谈开始或结束时）",
            "依次递送，不要遗漏",
            "动作从容优雅",
        ]
    }

    /// 接收名片礼仪
    pub fn receiving(&self) -> Vec<&'static str> {
        vec![
            "双手接收名片",
            "仔细阅读名片内容",
            "确认姓名发音",
            "妥善收好名片",
            "不要立即收起不看",
            "不要在名片上写字（亚洲文化）",
            "不要折叠或弄皱名片",
            "将名片放在名片夹中",
        ]
    }

    /// 名片交换时机
    pub fn exchange_timing(&self) -> Vec<&'static str> {
        vec![
            "初次见面介绍时",
            "会议开始前寒暄时",
            "商务场合自我介绍时",
            "离开时表示感谢",
            "对方主动递交时",
            "避免在忙碌或尴尬时递交",
            "避免在餐桌上递交",
            "避免一次递交多张",
        ]
    }

    /// 不同文化差异
    pub fn cultural_differences(&self) -> Vec<&'static str> {
        vec![
            "日本：高度重视名片交换，双手递交和接收，仔细阅读",
            "中国：双手交换，认真查看，表示尊重",
            "韩国：与日本类似，等级重要",
            "美国：相对随意，可单手交换",
            "欧洲：正式但不过分讲究",
            "中东：右手递交，左手可托右手腕",
            "印度：尊重地接收，避免用左手",
            "国际：双手交换是安全选择",
        ]
    }

    /// 名片管理
    pub fn card_management(&self) -> Vec<&'static str> {
        vec![
            "使用名片夹存放名片",
            "按字母或类别整理",
            "及时录入联系人系统",
            "备注会面时间和地点",
            "定期清理和更新",
            "备份电子版本",
            "保护名片免受损坏",
            "随身携带足够名片",
        ]
    }

    /// 名片禁忌
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "不要递送脏污或折皱的名片",
            "不要递送过时信息名片",
            "不要在名片上随意涂写",
            "不要把玩对方名片",
            "不要将名片放入裤兜",
            "不要在名片上放杯子",
            "不要过早收起对方名片",
            "不要拒绝接收名片",
        ]
    }

    /// 电子名片
    pub fn digital_cards(&self) -> Vec<&'static str> {
        vec![
            "保持信息同步更新",
            "设计简洁专业",
            "易于分享和保存",
            "包含社交媒体链接",
            "二维码清晰可扫描",
            "适配不同设备显示",
            "保护个人隐私信息",
            "与传统名片配合使用",
        ]
    }
}

impl Rule for BusinessCardRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("business")
    }

    fn explain(&self) -> String {
        format!(
            "【商务名片礼仪】\n\n\
            名片设计原则：\n{}\n\n\
            名片信息内容：\n{}\n\n\
            递交名片礼仪：\n{}\n\n\
            接收名片礼仪：\n{}\n\n\
            名片交换时机：\n{}\n\n\
            不同文化差异：\n{}\n\n\
            名片管理：\n{}\n\n\
            名片禁忌：\n{}\n\n\
            电子名片：\n{}",
            self.design_principles()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.card_content()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.presenting()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.receiving()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.exchange_timing()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cultural_differences()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.card_management()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.taboos()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.digital_cards()
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
    fn test_business_card_rules() {
        let rules = BusinessCardRules::new();
        assert_eq!(rules.metadata().name, "商务名片礼仪");
        assert!(!rules.design_principles().is_empty());
        assert!(!rules.card_content().is_empty());
        assert!(!rules.presenting().is_empty());
        assert!(!rules.receiving().is_empty());
        assert!(!rules.exchange_timing().is_empty());
        assert!(!rules.cultural_differences().is_empty());
        assert!(!rules.card_management().is_empty());
        assert!(!rules.taboos().is_empty());
        assert!(!rules.digital_cards().is_empty());
    }

    #[test]
    fn test_business_card_validation() {
        let rules = BusinessCardRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("business"));
    }

    #[test]
    fn test_business_card_explain() {
        let rules = BusinessCardRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("名片设计原则"));
        assert!(explanation.contains("递交名片礼仪"));
        assert!(explanation.contains("名片禁忌"));
    }
}
