//! 社交媒体礼仪
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: SocialMediaEtiquetteRules,
    name: "社交媒体礼仪",
    desc: "社交媒体使用礼仪",
    origin: "国际",
    tags: ["社交", "网络"],
    category: RuleCategory::social("social_media_etiquette"),
    sections: [("发布", section_0), ("互动", section_1)]
}

impl SocialMediaEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["不发不当内容", "尊重他人肖像权", "注明出处"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["文明评论", "不网暴", "理性讨论"]
    }
}
