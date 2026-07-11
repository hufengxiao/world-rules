//! 国庆节礼仪 - 中国国庆节礼仪规范
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! { struct: NationalDayRules, name: "国庆节礼仪", desc: "中国国庆节礼仪规范", origin: "中国", tags: ["社交", "节日", "国家"] }

impl NationalDayRules {
    /// 节日意义
    pub fn significance(&self) -> Vec<&'static str> {
        vec![
            "纪念建国 - 纪念中华人民共和国成立",
            "爱国教育 - 开展爱国主义教育活动",
            "民族团结 - 弘扬民族团结精神",
            "国家庆典 - 国家层面的盛大庆典",
            "展示成就 - 展示国家建设取得的伟大成就",
        ]
    }

    /// 公务礼仪
    pub fn official_etiquette(&self) -> Vec<&'static str> {
        vec![
            "升旗仪式 - 参加升国旗仪式",
            "向人民英雄纪念碑献花 - 缅怀革命先烈",
            "阅兵式 - 观看或参加阅兵仪式",
            "国庆招待会 - 国家举行国庆招待会",
            "表彰先进 - 表彰为国家做出贡献的个人和集体",
            "政府公告 - 发布政府国庆公告",
        ]
    }

    /// 民间庆祝
    pub fn public_celebrations(&self) -> Vec<&'static str> {
        vec![
            "观看升旗 - 前往天安门广场观看升旗",
            "挂国旗 - 在家中或单位悬挂国旗",
            "观看晚会 - 观看国庆文艺晚会",
            "外出旅游 - 利用国庆假期外出旅游",
            "家庭聚餐 - 全家团聚庆祝国庆",
            "参观展览 - 参观博物馆和历史展览",
        ]
    }

    /// 爱国礼仪
    pub fn patriotic_etiquette(&self) -> Vec<&'static str> {
        vec![
            "尊重国旗 - 正确使用和尊重国旗",
            "唱国歌 - 庄重唱响国歌",
            "了解历史 - 了解国家历史和发展",
            "珍惜和平 - 珍惜来之不易的和平",
            "建设国家 - 为国家发展贡献力量",
            "民族团结 - 维护民族团结和国家统一",
        ]
    }

    /// 礼仪规范
    pub fn etiquette_rules(&self) -> Vec<&'static str> {
        vec![
            "文明出行 - 旅游时注意文明礼貌",
            "遵守秩序 - 公共场合遵守秩序",
            "爱护环境 - 保护环境不乱扔垃圾",
            "安全意识 - 注意假期安全",
            "理性消费 - 节日期间理性消费",
            "遵守法律 - 遵守国家法律法规",
        ]
    }

    /// 祝福用语
    pub fn greetings(&self) -> Vec<&'static str> {
        vec![
            "国庆快乐 - 最常用的节日祝福",
            "祖国万岁 - 祝福祖国繁荣昌盛",
            "国家繁荣昌盛 - 祝愿国家兴旺发达",
            "人民幸福安康 - 祝愿人民生活幸福",
            "国泰民安 - 祝愿国家安定人民安乐",
            "节日快乐 - 通用节日祝福",
        ]
    }
}

impl Rule for NationalDayRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("national_day")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "国庆节礼仪",
            &[
                ("节日意义", &self.significance()),
                ("公务礼仪", &self.official_etiquette()),
                ("民间庆祝", &self.public_celebrations()),
                ("爱国礼仪", &self.patriotic_etiquette()),
                ("礼仪规范", &self.etiquette_rules()),
                ("祝福用语", &self.greetings()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_national_day_rules() {
        let rules = NationalDayRules::new();
        assert_eq!(rules.metadata().name, "国庆节礼仪");
        assert!(!rules.explain().is_empty());
        assert!(rules.significance().len() >= 5);
        assert!(rules.official_etiquette().len() >= 5);
        assert!(rules.public_celebrations().len() >= 5);
        assert!(rules.patriotic_etiquette().len() >= 5);
    }
}
