//! 中国传统书信礼仪
//!
//! 传统书信是中国人际交往的重要方式，有严格的格式和用词规范。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ChineseCorrespondenceRules,
    name: "中国传统书信礼仪",
    desc: "传统书信格式与用词规范",
    origin: "中国",
    tags: ["社交", "礼仪", "书信", "传统"]
}

impl ChineseCorrespondenceRules {
    /// 书信格式
    pub fn letter_format(&self) -> Vec<&'static str> {
        vec![
            "抬头 - 收信人称呼，顶格书写",
            "启辞 - 开场寒暄语",
            "正文 - 书信主体内容",
            "结语 - 结尾客套语",
            "祝颂 - 祝福语",
            "署名 - 发信人姓名",
            "日期 - 写信时间",
            "附言 - 补充说明",
        ]
    }

    /// 称呼格式
    pub fn address_format(&self) -> Vec<&'static str> {
        vec![
            "长辈 - 尊前、尊鉴、膝下、膝前",
            "师长 - 道席、函丈、座右、讲席",
            "平辈 - 足下、阁下、台鉴、左右",
            "晚辈 - 如面、如晤、收阅、启",
            "官长 - 钧鉴、勋鉴、钧座",
            "夫妇 - 俪鉴、双鉴、同鉴",
            "学者 - 道鉴、著席、撰席",
            "商界 - 台鉴、雅鉴、惠鉴",
        ]
    }

    /// 启辞用语
    pub fn opening_phrases(&self) -> Vec<&'static str> {
        vec![
            "问候启辞 - 久未晤面，甚为挂念",
            "复信启辞 - 顷接来信，欣悉一切",
            "请托启辞 - 冒昧致函，恳请相助",
            "慰问启辞 - 近闻不适，甚为挂念",
            "贺喜启辞 - 欣闻捷报，衷心祝贺",
            "致谢启辞 - 承蒙厚爱，深表感谢",
            "道歉启辞 - 昨失礼仪，深感愧疚",
            "告别启辞 - 即将远行，特此辞别",
        ]
    }

    /// 祝颂语
    pub fn closing_phrases(&self) -> Vec<&'static str> {
        vec![
            "长辈 - 敬请 钧安、恭请 金安",
            "师长 - 敬请 道安、恭请 教安",
            "平辈 - 即请 大安、顺颂 时祺",
            "晚辈 - 即问 近好、顺问 近佳",
            "官长 - 敬请 钧安、恭请 勋安",
            "秋节 - 顺颂 秋安、即请 秋祺",
            "春节 - 恭贺 新禧、敬祝 春祺",
            "通用 - 即问 近好、顺颂 近佳",
        ]
    }

    /// 谦称用语
    pub fn humble_terms(&self) -> Vec<&'static str> {
        vec![
            "自称 - 鄙人、在下、不才、愚",
            "家称 - 家父、家母、家兄、家姐",
            "舍称 - 舍弟、舍妹、舍亲",
            "小称 - 小女、小儿、小婿",
            "敝称 - 敝友、敝同乡、敝业",
            "愚称 - 愚见、愚意、愚以为",
            "拙称 - 拙作、拙见、拙荆",
            "寒称 - 寒舍、寒门、寒族",
        ]
    }

    /// 敬称用语
    pub fn honorific_terms(&self) -> Vec<&'static str> {
        vec![
            "尊称 - 尊父、尊母、尊驾",
            "令称 - 令尊、令堂、令郎、令爱",
            "贵称 - 贵府、贵友、贵同乡",
            "高称 - 高见、高论、高寿",
            "贤称 - 贤弟、贤侄、贤婿",
            "宝称 - 宝眷、宝号、宝地",
            "大称 - 大作、大函、大礼",
            "惠称 - 惠函、惠书、惠赠",
        ]
    }

    /// 书信禁忌
    pub fn correspondence_taboos(&self) -> Vec<&'static str> {
        vec![
            "不可直呼长辈姓名",
            "不可用红笔写信（绝交之意）",
            "不可用铅笔写信（不郑重）",
            "不可写潦草字迹",
            "不可在信纸上涂改",
            "不可写不吉利的话",
            "不可用丧家笔墨",
            "不可私信拆阅他人信件",
        ]
    }

    /// 现代书信
    pub fn modern_correspondence(&self) -> Vec<&'static str> {
        vec![
            "电子邮件 - 格式可简化，用语需礼貌",
            "短信微信 - 简洁明了，不失礼节",
            "商务函件 - 格式规范，用语得体",
            "感谢信 - 真诚表达，不空洞",
            "邀请函 - 信息完整，礼貌周全",
            "慰问信 - 语气诚恳，情感真挚",
            "道歉信 - 真心认错，提出补救",
            "辞职信 - 礼貌得体，好聚好散",
        ]
    }
}

impl Rule for ChineseCorrespondenceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_correspondence")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中国传统书信礼仪",
            &[
                ("书信格式", &self.letter_format()),
                ("称呼格式", &self.address_format()),
                ("启辞用语", &self.opening_phrases()),
                ("祝颂语", &self.closing_phrases()),
                ("谦称用语", &self.humble_terms()),
                ("敬称用语", &self.honorific_terms()),
                ("书信禁忌", &self.correspondence_taboos()),
                ("现代书信", &self.modern_correspondence()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correspondence_rules() {
        let rules = ChineseCorrespondenceRules::new();
        assert_eq!(rules.metadata().name, "中国传统书信礼仪");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_letter_format() {
        let rules = ChineseCorrespondenceRules::new();
        let format = rules.letter_format();
        assert!(format.iter().any(|f| f.contains("抬头")));
        assert!(format.iter().any(|f| f.contains("正文")));
        assert!(format.len() >= 6);
    }

    #[test]
    fn test_address_format() {
        let rules = ChineseCorrespondenceRules::new();
        let address = rules.address_format();
        assert!(address.iter().any(|a| a.contains("长辈")));
        assert!(address.iter().any(|a| a.contains("师长")));
        assert!(address.len() >= 6);
    }

    #[test]
    fn test_humble_terms() {
        let rules = ChineseCorrespondenceRules::new();
        let terms = rules.humble_terms();
        assert!(terms.iter().any(|t| t.contains("鄙人")));
        assert!(terms.iter().any(|t| t.contains("愚")));
        assert!(terms.len() >= 6);
    }
}