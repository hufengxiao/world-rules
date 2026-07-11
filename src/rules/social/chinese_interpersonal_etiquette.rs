//! 中国传统待人接物礼仪
//!
//! 待人接物是中国传统礼仪的核心内容，涉及人际交往的各个方面。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ChineseInterpersonalEtiquetteRules,
    name: "中国待人接物礼仪",
    desc: "传统人际交往礼仪规范",
    origin: "中国",
    tags: ["社交", "礼仪", "人际", "传统"]
}

impl ChineseInterpersonalEtiquetteRules {
    /// 见面礼仪
    pub fn greeting_etiquette(&self) -> Vec<&'static str> {
        vec![
            "拱手礼 - 右手握拳，左手抱右拳",
            "作揖礼 - 双手合抱，俯身致意",
            "鞠躬礼 - 根据对象深浅有度",
            "点头礼 - 平辈之间的问候",
            "问安礼 - 晚辈向长辈请安",
            "尊称礼 - 使用尊称，不直呼其名",
            "长幼有序 - 长辈先开口，晚辈回应",
            "男女有别 - 保持适当距离",
        ]
    }

    /// 称呼礼仪
    pub fn address_etiquette(&self) -> Vec<&'static str> {
        vec![
            "长辈称呼 - 伯父、伯母、叔父、婶母",
            "平辈称呼 - 兄、弟、姐、妹",
            "晚辈称呼 - 侄、甥、儿、女",
            "师长称呼 - 先生、老师、夫子",
            "官职称呼 - 大人、老爷、县令",
            "敬称 - 足下、阁下、仁兄",
            "谦称 - 鄙人、在下、小弟",
            "避讳 - 不直呼长辈名讳",
        ]
    }

    /// 待客礼仪
    pub fn hosting_etiquette(&self) -> Vec<&'static str> {
        vec![
            "迎客于门 - 主人出门迎接",
            "请客先行 - 主人让客人先行",
            "座次有序 - 尊者上座",
            "奉茶敬烟 - 先客后主",
            "敬茶方式 - 双手奉茶，茶斟七分",
            "敬酒礼仪 - 先客后主，先长后幼",
            "劝酒适度 - 不可强人所难",
            "送客于门 - 主人送客人至门口",
        ]
    }

    /// 作客礼仪
    pub fn guest_etiquette(&self) -> Vec<&'static str> {
        vec![
            "预约拜访 - 提前约定时间",
            "准时到达 - 不迟到、不早到",
            "敲门礼仪 - 敲三下，等待回应",
            "脱鞋入室 - 根据主人习惯",
            "礼物携带 - 空手不登门",
            "入座规范 - 等主人安排座次",
            "用餐礼仪 - 等主人动筷",
            "告辞时机 - 不久留、适时告辞",
        ]
    }

    /// 送礼礼仪
    pub fn gift_giving(&self) -> Vec<&'static str> {
        vec![
            "礼物选择 - 投其所好",
            "礼尚往来 - 有来有往",
            "送礼时机 - 节庆、喜事、拜访",
            "礼物包装 - 精美包装，表心意",
            "送礼方式 - 悄悄送、不当众送",
            "收礼回礼 - 表达感谢、适度回礼",
            "避讳之物 - 钟、伞、梨、鞋等",
            "送礼数量 - 成双不成单",
        ]
    }

    /// 饮茶礼仪
    pub fn tea_etiquette(&self) -> Vec<&'static str> {
        vec![
            "泡茶规范 - 温杯、洗茶、冲泡",
            "敬茶方式 - 双手奉茶，茶斟七分",
            "接茶方式 - 双手接茶，点头致谢",
            "品茶礼仪 - 先闻香，后品茶",
            "续茶时机 - 茶喝半杯即续",
            "茶桌礼仪 - 不可随意翻动茶具",
            "叩指礼 - 长辈倒茶时行叩指礼",
            "谢茶辞 - 喝完三杯后表达感谢",
        ]
    }

    /// 言谈礼仪
    pub fn speech_etiquette(&self) -> Vec<&'static str> {
        vec![
            "言必有礼 - 语言文明，不粗俗",
            "言必有据 - 说话有据，不妄言",
            "言必有信 - 说到做到，守信诺",
            "言必有度 - 说话有度，不逾矩",
            "倾听为主 - 少说多听",
            "不打断他人 - 等人说完再发言",
            "不背后议论 - 不道人长短",
            "和颜悦色 - 说话态度和蔼",
        ]
    }

    /// 交往禁忌
    pub fn interaction_taboos(&self) -> Vec<&'static str> {
        vec![
            "不可对人指指点点",
            "不可当众剔牙、挖耳",
            "不可对人打哈欠、伸懒腰",
            "不可对着人咳嗽、打喷嚏",
            "不可当众整理衣物",
            "不可窥视他人隐私",
            "不可打听他人收入",
            "不可评论他人长相",
        ]
    }
}

impl Rule for ChineseInterpersonalEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_interpersonal_etiquette")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中国待人接物礼仪",
            &[
                ("见面礼仪", &self.greeting_etiquette()),
                ("称呼礼仪", &self.address_etiquette()),
                ("待客礼仪", &self.hosting_etiquette()),
                ("作客礼仪", &self.guest_etiquette()),
                ("送礼礼仪", &self.gift_giving()),
                ("饮茶礼仪", &self.tea_etiquette()),
                ("言谈礼仪", &self.speech_etiquette()),
                ("交往禁忌", &self.interaction_taboos()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpersonal_etiquette_rules() {
        let rules = ChineseInterpersonalEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "中国待人接物礼仪");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_greeting_etiquette() {
        let rules = ChineseInterpersonalEtiquetteRules::new();
        let greeting = rules.greeting_etiquette();
        assert!(greeting.iter().any(|g| g.contains("拱手")));
        assert!(greeting.iter().any(|g| g.contains("鞠躬")));
        assert!(greeting.len() >= 6);
    }

    #[test]
    fn test_hosting_etiquette() {
        let rules = ChineseInterpersonalEtiquetteRules::new();
        let hosting = rules.hosting_etiquette();
        assert!(hosting.iter().any(|h| h.contains("迎客")));
        assert!(hosting.iter().any(|h| h.contains("送客")));
        assert!(hosting.len() >= 6);
    }

    #[test]
    fn test_tea_etiquette() {
        let rules = ChineseInterpersonalEtiquetteRules::new();
        let tea = rules.tea_etiquette();
        assert!(tea.iter().any(|t| t.contains("奉茶")));
        assert!(tea.iter().any(|t| t.contains("叩指")));
        assert!(tea.len() >= 6);
    }
}
