//! 外交礼仪
//!
//! 涵盖外交场合的详细规范，包括外交礼仪、外交特权、外交礼节等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DiplomaticEtiquetteRules,
    name: "外交礼仪",
    desc: "外交场合礼仪详细规范，包括外交礼仪、外交特权、外交礼节等",
    origin: "国际",
    tags: ["社交", "礼仪", "国际", "外交"]
}

impl DiplomaticEtiquetteRules {
    /// 外交特权与豁免
    pub fn privileges_and_immunities(&self) -> Vec<&'static str> {
        vec![
            "外交人员享有刑事管辖豁免权",
            "外交人员享有民事管辖豁免权",
            "外交馆舍不受侵犯",
            "外交档案和文件不受侵犯",
            "外交邮袋不受检查",
            "外交人员享有税收豁免",
            "外交人员享有关税豁免",
            "外交人员及其家属享有特权",
        ]
    }

    /// 国书递交礼仪
    pub fn credentials_presentation(&self) -> Vec<&'static str> {
        vec![
            "新任大使抵达后应尽快安排递交国书",
            "国书由元首签署，致接受国元首",
            "递交国书时着正装或民族服装",
            "大使向接受国元首致颂词",
            "接受国元首致答词",
            "递交国书后拜会外交部长",
            "拜会其他外交团团长",
            "正式拜会前需事先联系安排",
        ]
    }

    /// 外交会见礼仪
    pub fn diplomatic_meetings(&self) -> Vec<&'static str> {
        vec![
            "正式会见需提前预约",
            "准备会谈议程和备忘录",
            "准时到达，不迟到也不早到",
            "着装得体，符合外交礼仪",
            "握手时力度适中，目光注视",
            "交换名片时双手递送",
            "座次按国际惯例安排",
            "会谈中保持礼貌和专业",
        ]
    }

    /// 国宴礼仪
    pub fn state_banquets(&self) -> Vec<&'static str> {
        vec![
            "国宴是最高规格的外交宴会",
            "座位按礼宾顺序安排",
            "主宾位于主人右侧",
            "着正装或民族礼服",
            "餐具按国际标准摆放",
            "菜谱体现国家特色",
            "祝酒按礼宾顺序进行",
            "国歌演奏时全体起立",
        ]
    }

    /// 外交赠礼礼仪
    pub fn diplomatic_gifts(&self) -> Vec<&'static str> {
        vec![
            "外交赠礼体现两国友谊",
            "礼品应具有本国特色",
            "价值适中，不过于昂贵",
            "附上说明和翻译",
            "正式场合由代表团团长赠送",
            "接收时表示感谢",
            "不公开讨论礼品价值",
            "礼品归国家所有，非个人",
        ]
    }

    /// 国旗礼仪
    pub fn flag_protocol(&self) -> Vec<&'static str> {
        vec![
            "国旗是国家象征，应受到尊重",
            "两国国旗并挂时，右为客方国旗",
            "多国国旗按国名英文首字母排列",
            "国旗日出升旗，日落降旗",
            "降半旗表示哀悼",
            "国旗不能接触地面",
            "破损国旗应及时更换",
            "国旗不能倒挂",
        ]
    }

    /// 外交文书礼仪
    pub fn diplomatic_correspondence(&self) -> Vec<&'static str> {
        vec![
            "外交文书格式规范严谨",
            "正式照会用于重要事项",
            "普通照会用于日常事务",
            "备忘录记录会谈内容",
            "外交函件用语正式",
            "称谓准确无误",
            "签名和印章完整",
            "文书记录存档",
        ]
    }

    /// 外交礼节
    pub fn diplomatic_courtesy(&self) -> Vec<&'static str> {
        vec![
            "外交人员应遵守接受国法律",
            "不干涉接受国内政",
            "尊重接受国风俗习惯",
            "保持外交人员尊严",
            "维护国家形象",
            "与各国使节保持友好",
            "参加外交活动积极得体",
            "离职时做好交接工作",
        ]
    }
}

impl Rule for DiplomaticEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("international")
    }

    fn explain(&self) -> String {
        format!(
            "【外交礼仪】\n\n\
            外交特权与豁免：\n{}\n\n\
            国书递交礼仪：\n{}\n\n\
            外交会见礼仪：\n{}\n\n\
            国宴礼仪：\n{}\n\n\
            外交赠礼礼仪：\n{}\n\n\
            国旗礼仪：\n{}\n\n\
            外交文书礼仪：\n{}\n\n\
            外交礼节：\n{}",
            self.privileges_and_immunities()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.credentials_presentation()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.diplomatic_meetings()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.state_banquets()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.diplomatic_gifts()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.flag_protocol()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.diplomatic_correspondence()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.diplomatic_courtesy()
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
    fn test_diplomatic_etiquette_rules() {
        let rules = DiplomaticEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "外交礼仪");
        assert!(!rules.privileges_and_immunities().is_empty());
        assert!(!rules.credentials_presentation().is_empty());
        assert!(!rules.diplomatic_meetings().is_empty());
        assert!(!rules.state_banquets().is_empty());
        assert!(!rules.diplomatic_gifts().is_empty());
        assert!(!rules.flag_protocol().is_empty());
        assert!(!rules.diplomatic_correspondence().is_empty());
        assert!(!rules.diplomatic_courtesy().is_empty());
    }

    #[test]
    fn test_diplomatic_etiquette_validation() {
        let rules = DiplomaticEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("international"));
    }

    #[test]
    fn test_diplomatic_etiquette_explain() {
        let rules = DiplomaticEtiquetteRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("外交特权与豁免"));
        assert!(explanation.contains("国书递交礼仪"));
        assert!(explanation.contains("外交会见礼仪"));
        assert!(explanation.contains("国宴礼仪"));
    }
}