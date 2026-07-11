//! 合同签署礼仪
//!
//! 涵盖商务合同签署过程中的礼仪规范，包括签署仪式、文件准备、签字顺序等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ContractSigningEtiquetteRules,
    name: "合同签署礼仪",
    desc: "商务合同签署礼仪规范，包括签署仪式、文件准备、签字顺序等",
    origin: "国际通用",
    tags: ["社交", "礼仪", "商务", "合同", "签署"]
}

impl ContractSigningEtiquetteRules {
    /// 合同准备礼仪
    pub fn preparation(&self) -> Vec<&'static str> {
        vec![
            "确保合同文本准确完整",
            "核对双方名称和签署权限",
            "准备签署所需的印章和签字笔",
            "确认签署时间和地点",
            "安排见证人员",
            "准备备份文件",
            "检查合同页码和附件",
            "确认法律顾问在场",
        ]
    }

    /// 签署仪式礼仪
    pub fn ceremony(&self) -> Vec<&'static str> {
        vec![
            "选择庄重的签署场所",
            "布置签署台和座位",
            "安排媒体拍照（如需要）",
            "准备签署用的签字笔",
            "确保合同文本摆放正确",
            "安排见证人站位",
            "预留双方交换合同的时间",
            "准备庆祝仪式（如庆功宴）",
        ]
    }

    /// 签字顺序礼仪
    pub fn signing_order(&self) -> Vec<&'static str> {
        vec![
            "遵循约定顺序（通常甲方先签）",
            "每页签字或盖骑缝章",
            "签字位置准确规范",
            "签字清晰可辨识",
            "盖章位置正确",
            "见证人签字见证",
            "日期填写准确",
            "双方签字完成后交换确认",
        ]
    }

    /// 签署后礼仪
    pub fn after_signing(&self) -> Vec<&'static str> {
        vec![
            "握手祝贺合作",
            "交换签署文本",
            "合影留念（如需要）",
            "感谢各方参与",
            "妥善保管合同原件",
            "归档和备份签署文件",
            "通知相关部门",
            "安排后续履行事宜",
        ]
    }

    /// 签署禁忌
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "不要在空白处随意签字",
            "不要签署未经审核的合同",
            "不要让无授权人员签字",
            "不要遗漏必要页码签字",
            "不要使用不规范的印章",
            "不要在非正式场所签署重要合同",
            "不要让外人随意翻阅合同",
            "不要延迟交换签署文本",
        ]
    }

    /// 电子签署礼仪
    pub fn electronic_signing(&self) -> Vec<&'static str> {
        vec![
            "确保电子签名平台合法可靠",
            "验证签署人身份真实性",
            "确认电子签名的法律效力",
            "保存签署过程记录",
            "备份电子合同文件",
            "通知对方签署完成",
            "确认双方都收到完整合同",
            "定期检查电子签名有效期",
        ]
    }

    /// 不同文化差异
    pub fn cultural_differences(&self) -> Vec<&'static str> {
        vec![
            "中国：重视盖章仪式，可能安排正式签署典礼",
            "日本：签字和盖章都重要，仪式庄重",
            "美国：签字为主，流程简洁",
            "欧洲：重视签字仪式，可能有见证人",
            "中东：可能需要多方见证，仪式隆重",
            "印度：重视仪式感，可能涉及宗教元素",
            "拉美：热情友好，签署后可能有庆祝活动",
            "东南亚：尊重当地习俗，注意宗教因素",
        ]
    }

    /// 国际合同签署
    pub fn international(&self) -> Vec<&'static str> {
        vec![
            "确认合同语言版本一致性",
            "明确适用法律和管辖权",
            "准备双语签署文本",
            "考虑跨境签署的法律效力",
            "安排翻译人员在场",
            "确认远程签署的合法性",
            "注意时区和日期格式差异",
            "了解对方国家的签署习惯",
        ]
    }
}

impl Rule for ContractSigningEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("business")
    }

    fn explain(&self) -> String {
        format!(
            "【合同签署礼仪】\n\n\
            合同准备礼仪：\n{}\n\n\
            签署仪式礼仪：\n{}\n\n\
            签字顺序礼仪：\n{}\n\n\
            签署后礼仪：\n{}\n\n\
            签署禁忌：\n{}\n\n\
            电子签署礼仪：\n{}\n\n\
            不同文化差异：\n{}\n\n\
            国际合同签署：\n{}",
            self.preparation()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.ceremony()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.signing_order()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.after_signing()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.taboos()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.electronic_signing()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cultural_differences()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.international()
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
    fn test_contract_signing_rules() {
        let rules = ContractSigningEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "合同签署礼仪");
        assert!(!rules.preparation().is_empty());
        assert!(!rules.ceremony().is_empty());
        assert!(!rules.signing_order().is_empty());
        assert!(!rules.after_signing().is_empty());
        assert!(!rules.taboos().is_empty());
        assert!(!rules.electronic_signing().is_empty());
        assert!(!rules.cultural_differences().is_empty());
        assert!(!rules.international().is_empty());
    }

    #[test]
    fn test_contract_signing_validation() {
        let rules = ContractSigningEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("business"));
    }

    #[test]
    fn test_contract_signing_explain() {
        let rules = ContractSigningEtiquetteRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("合同准备礼仪"));
        assert!(explanation.contains("签署仪式礼仪"));
        assert!(explanation.contains("签字顺序礼仪"));
        assert!(explanation.contains("签署禁忌"));
    }
}
