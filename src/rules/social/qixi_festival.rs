//! 七夕节礼仪 - 中国情人节传统礼仪规范
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! { struct: QixiFestivalRules, name: "七夕节礼仪", desc: "中国七夕节传统礼仪规范", origin: "中国", tags: ["社交", "节日", "爱情"] }

impl QixiFestivalRules {
    /// 七夕传说与文化
    pub fn legend_culture(&self) -> Vec<&'static str> {
        vec![
            "牛郎织女传说 - 七夕起源于牛郎织女的爱情故事",
            "鹊桥相会 - 喜鹊搭桥让牛郎织女相会",
            "银河阻隔 - 天河将牛郎织女分隔两岸",
            "乞巧习俗 - 女子在七夕向织女乞求巧艺",
            "爱情象征 - 七夕成为中国传统情人节",
        ]
    }

    /// 传统习俗
    pub fn traditional_customs(&self) -> Vec<&'static str> {
        vec![
            "穿针乞巧 - 月下穿针比赛谁更快",
            "投针验巧 - 将针浮于水面验巧",
            "种生求子 - 种植豆苗祈求子嗣",
            "拜织女 - 祭拜织女祈求巧艺",
            "吃巧果 - 食用各种巧果点心",
            "染指甲 - 少女用凤仙花染指甲",
        ]
    }

    /// 现代庆祝方式
    pub fn modern_celebrations(&self) -> Vec<&'static str> {
        vec![
            "送玫瑰花 - 表达爱意的经典方式",
            "共进晚餐 - 情侣约会聚餐",
            "送巧克力 - 甜蜜爱情的象征",
            "看电影 - 情侣观看爱情电影",
            "互赠礼物 - 表达心意",
            "观星赏月 - 寻找牛郎织女星",
        ]
    }

    /// 礼仪规范
    pub fn etiquette(&self) -> Vec<&'static str> {
        vec![
            "表达爱意 - 勇敢向心爱的人表白",
            "尊重传统 - 了解七夕的文化内涵",
            "真诚祝福 - 向恋人送上真挚祝福",
            "浪漫约会 - 精心安排约会活动",
            "表达感谢 - 感谢伴侣的陪伴与付出",
        ]
    }

    /// 禁忌事项
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "不争吵 - 七夕不宜与恋人争吵",
            "不提分手 - 情人节不宜谈论分手",
            "不独自哭泣 - 传统认为不吉利",
            "不过度消费 - 理性消费，量力而行",
            "不忘礼物 - 忘记礼物是大忌",
        ]
    }
}

impl Rule for QixiFestivalRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("qixi_festival")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "七夕节礼仪",
            &[
                ("传说与文化", &self.legend_culture()),
                ("传统习俗", &self.traditional_customs()),
                ("现代庆祝", &self.modern_celebrations()),
                ("礼仪规范", &self.etiquette()),
                ("禁忌事项", &self.taboos()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qixi_festival_rules() {
        let rules = QixiFestivalRules::new();
        assert_eq!(rules.metadata().name, "七夕节礼仪");
        assert!(!rules.explain().is_empty());
        assert!(rules.legend_culture().len() >= 5);
        assert!(rules.traditional_customs().len() >= 5);
        assert!(rules.modern_celebrations().len() >= 5);
        assert!(rules.etiquette().len() >= 5);
    }
}
