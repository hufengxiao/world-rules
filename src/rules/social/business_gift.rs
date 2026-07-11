//! 商务礼品礼仪
//!
//! 涵盖商务场合礼品赠送规范，包括选择、时机、包装、禁忌等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BusinessGiftRules,
    name: "商务礼品礼仪",
    desc: "商务场合礼品赠送规范，包括选择、时机、包装、禁忌等",
    origin: "国际通用",
    tags: ["社交", "礼仪", "商务", "礼品", "赠送"]
}

impl BusinessGiftRules {
    /// 礼品选择原则
    pub fn selection_principles(&self) -> Vec<&'static str> {
        vec![
            "体现对收礼者的尊重和了解",
            "价值适中，不过于昂贵",
            "符合商务关系和场合",
            "具有纪念意义或实用性",
            "质量优良，包装精美",
            "避免过于私人化",
            "考虑对方公司的政策",
            "符合法律法规要求",
        ]
    }

    /// 合适的商务礼品
    pub fn appropriate_gifts(&self) -> Vec<&'static str> {
        vec![
            "高品质办公用品（笔记本、笔）",
            "公司定制纪念品",
            "当地特产和美食",
            "书籍或订阅服务",
            "艺术品或装饰品",
            "高品质茶或咖啡",
            "葡萄酒或烈酒（注意文化）",
            "科技产品配件",
            "植物或花卉",
            "慈善捐赠（以对方名义）",
        ]
    }

    /// 送礼时机
    pub fn timing(&self) -> Vec<&'static str> {
        vec![
            "首次会面后表示感谢",
            "节日或新年问候",
            "项目完成庆祝",
            "合同签订纪念",
            "访问对方公司时",
            "对方晋升或成就",
            "商务访问结束",
            "避免敏感时期（决策前）",
        ]
    }

    /// 礼品包装礼仪
    pub fn wrapping(&self) -> Vec<&'static str> {
        vec![
            "使用专业包装材料",
            "颜色选择考虑文化差异",
            "避免黑色或白色包装（亚洲）",
            "附上精美贺卡",
            "手写祝福语",
            "避免过度包装",
            "确保包装完好",
            "红色或金色适合中国",
        ]
    }

    /// 送礼礼仪
    pub fn giving_etiquette(&self) -> Vec<&'static str> {
        vec![
            "双手递交礼品",
            "说明礼品含义和来源",
            "表达真诚的祝福",
            "不要强调礼品价值",
            "避免要求立即打开",
            "尊重对方接受与否",
            "不在公共场合送礼（视情况）",
            "记录送礼对象和时间",
        ]
    }

    /// 收礼礼仪
    pub fn receiving_etiquette(&self) -> Vec<&'static str> {
        vec![
            "双手接受礼品",
            "表达感谢",
            "不当面评价礼品价值",
            "询问是否可以打开",
            "注意公司礼品政策",
            "避免礼品冲突",
            "及时写感谢信",
            "记录礼品和送礼者",
        ]
    }

    /// 各国文化差异
    pub fn cultural_differences(&self) -> Vec<&'static str> {
        vec![
            "中国：红色包装吉利，避免钟、伞、梨",
            "日本：包装重要，不当面打开，避免4、9数量",
            "美国：可当面打开，表达感谢",
            "欧洲：葡萄酒、巧克力受欢迎",
            "中东：避免酒类和猪肉制品",
            "印度：避免牛皮制品，素食者多",
            "韩国：双手递交，不当面打开",
            "法国：品质优于数量，艺术品受欢迎",
        ]
    }

    /// 礼品禁忌
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "避免过于昂贵的礼品",
            "避免过于私人化物品",
            "避免可能被视为贿赂",
            "避免竞争对手产品",
            "避免宗教敏感物品",
            "避免政治相关礼品",
            "避免暗示性物品",
            "注意过敏和禁忌（食物）",
            "避免对方文化禁忌物品",
            "避免超过公司规定限额",
        ]
    }

    /// 公司礼品政策
    pub fn company_policies(&self) -> Vec<&'static str> {
        vec![
            "了解对方公司礼品政策",
            "遵守本公司的规定",
            "记录所有礼品往来",
            "避免现金或等价物",
            "必要时申报或拒绝",
            "透明公开处理",
            "避免利益冲突",
            "符合反腐败法规",
        ]
    }
}

impl Rule for BusinessGiftRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("business")
    }

    fn explain(&self) -> String {
        format!(
            "【商务礼品礼仪】\n\n\
            礼品选择原则：\n{}\n\n\
            合适的商务礼品：\n{}\n\n\
            送礼时机：\n{}\n\n\
            礼品包装礼仪：\n{}\n\n\
            送礼礼仪：\n{}\n\n\
            收礼礼仪：\n{}\n\n\
            各国文化差异：\n{}\n\n\
            礼品禁忌：\n{}\n\n\
            公司礼品政策：\n{}",
            self.selection_principles()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.appropriate_gifts()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.timing()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.wrapping()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.giving_etiquette()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.receiving_etiquette()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cultural_differences()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.taboos()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.company_policies()
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
    fn test_business_gift_rules() {
        let rules = BusinessGiftRules::new();
        assert_eq!(rules.metadata().name, "商务礼品礼仪");
        assert!(!rules.selection_principles().is_empty());
        assert!(!rules.appropriate_gifts().is_empty());
        assert!(!rules.timing().is_empty());
        assert!(!rules.wrapping().is_empty());
        assert!(!rules.giving_etiquette().is_empty());
        assert!(!rules.receiving_etiquette().is_empty());
        assert!(!rules.cultural_differences().is_empty());
        assert!(!rules.taboos().is_empty());
        assert!(!rules.company_policies().is_empty());
    }

    #[test]
    fn test_business_gift_validation() {
        let rules = BusinessGiftRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("business"));
    }

    #[test]
    fn test_business_gift_explain() {
        let rules = BusinessGiftRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("礼品选择原则"));
        assert!(explanation.contains("送礼时机"));
        assert!(explanation.contains("礼品禁忌"));
    }
}
