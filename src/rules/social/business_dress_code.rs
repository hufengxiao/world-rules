//! 商务着装礼仪
//!
//! 涵盖各类商务场合的着装规范，包括正式商务、商务休闲、行业差异等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BusinessDressCodeRules,
    name: "商务着装礼仪",
    desc: "商务场合着装规范，包括正式商务装、商务休闲装、行业差异等",
    origin: "国际通用",
    tags: ["社交", "礼仪", "商务", "着装"]
}

impl BusinessDressCodeRules {
    /// 男性正式商务着装
    pub fn men_formal(&self) -> Vec<&'static str> {
        vec![
            "深色西装（黑色、深蓝、深灰）",
            "长袖正装衬衫（白色、浅蓝）",
            "保守色领带（深红、深蓝、条纹）",
            "皮鞋与皮带颜色一致",
            "深色正装袜",
            "发型整洁，胡须修剪整齐",
            "避免过多配饰",
            "手表简洁大方",
            "公文包简约专业",
        ]
    }

    /// 女性正式商务着装
    pub fn women_formal(&self) -> Vec<&'static str> {
        vec![
            "职业套装（裙装或裤装）",
            "保守色系（黑、深蓝、灰、棕）",
            "衬衫或正装上衣",
            "裙长过膝",
            "保守高跟鞋（3-5厘米）",
            "肤色或深色丝袜",
            "简约首饰",
            "妆容淡雅",
            "发型整洁",
            "指甲修剪整齐",
        ]
    }

    /// 商务休闲着装
    pub fn business_casual(&self) -> Vec<&'static str> {
        vec![
            "卡其裤或西裤",
            "有领衬衫或Polo衫",
            "休闲西装外套（可选）",
            "皮鞋或休闲皮鞋",
            "毛衣或开衫",
            "避免牛仔裤（除非公司文化允许）",
            "避免运动鞋",
            "避免过于花哨的图案",
            "服装整洁、熨烫平整",
            "保持专业形象",
        ]
    }

    /// 智能休闲着装
    pub fn smart_casual(&self) -> Vec<&'static str> {
        vec![
            "深色牛仔裤（整洁、无破损）",
            "休闲衬衫或高质感T恤",
            "西装外套或休闲外套",
            "皮鞋或干净的运动鞋",
            "避免过度休闲",
            "保持整洁得体",
            "配饰简约",
            "适合周五或创意行业",
        ]
    }

    /// 不同行业着装差异
    pub fn industry_dress_code(&self) -> Vec<&'static str> {
        vec![
            "金融业：正式商务装，保守传统",
            "法律业：正式商务装，严谨专业",
            "咨询业：正式商务装，精致得体",
            "科技业：商务休闲或智能休闲",
            "创意业：相对自由，体现个性",
            "制造业：商务休闲，注重安全",
            "零售业：商务休闲或制服",
            "教育业：商务休闲，端庄得体",
        ]
    }

    /// 商务场合着装禁忌
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "避免过于暴露的服装",
            "避免透视或紧身衣物",
            "避免过于鲜艳花哨的颜色",
            "避免拖鞋或人字拖",
            "避免运动服装（运动场合除外）",
            "避免过多的首饰和配饰",
            "避免皱巴巴的衣服",
            "避免强烈的香水或古龙水",
            "避免有争议图案的T恤",
            "避免过度个性化的发型",
        ]
    }

    /// 面试着装建议
    pub fn interview_dress(&self) -> Vec<&'static str> {
        vec![
            "选择正式商务装",
            "颜色保守为主",
            "服装整洁、熨烫平整",
            "鞋擦干净，皮带扣光亮",
            "发型整齐，指甲干净",
            "香水淡雅或不使用",
            "避免过分时尚的款式",
            "了解公司文化后适当调整",
        ]
    }

    /// 文化差异注意
    pub fn cultural_notes(&self) -> Vec<&'static str> {
        vec![
            "中东：女性着装保守，遮盖手臂和腿",
            "日本：正式场合着装保守，避免醒目配饰",
            "印度：可接受传统服装，女性注意遮盖",
            "夏威夷：可穿阿罗哈衬衫",
            "欧洲南部：夏季着装相对宽松",
            "热带地区：轻薄面料可接受",
            "北欧：简洁实用，避免过度奢华",
        ]
    }
}

impl Rule for BusinessDressCodeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("business")
    }

    fn explain(&self) -> String {
        format!(
            "【商务着装礼仪】\n\n\
            男性正式商务着装：\n{}\n\n\
            女性正式商务着装：\n{}\n\n\
            商务休闲着装：\n{}\n\n\
            智能休闲着装：\n{}\n\n\
            不同行业着装差异：\n{}\n\n\
            商务场合着装禁忌：\n{}\n\n\
            面试着装建议：\n{}\n\n\
            文化差异注意：\n{}",
            self.men_formal()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.women_formal()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.business_casual()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.smart_casual()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.industry_dress_code()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.taboos()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.interview_dress()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cultural_notes()
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
    fn test_business_dress_code_rules() {
        let rules = BusinessDressCodeRules::new();
        assert_eq!(rules.metadata().name, "商务着装礼仪");
        assert!(!rules.men_formal().is_empty());
        assert!(!rules.women_formal().is_empty());
        assert!(!rules.business_casual().is_empty());
        assert!(!rules.smart_casual().is_empty());
        assert!(!rules.industry_dress_code().is_empty());
        assert!(!rules.taboos().is_empty());
        assert!(!rules.interview_dress().is_empty());
        assert!(!rules.cultural_notes().is_empty());
    }

    #[test]
    fn test_business_dress_code_validation() {
        let rules = BusinessDressCodeRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("business"));
    }

    #[test]
    fn test_business_dress_code_explain() {
        let rules = BusinessDressCodeRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("男性正式商务着装"));
        assert!(explanation.contains("女性正式商务着装"));
        assert!(explanation.contains("商务休闲着装"));
        assert!(explanation.contains("着装禁忌"));
    }
}
