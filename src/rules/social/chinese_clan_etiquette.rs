//! 中国传统宗族礼仪
//!
//! 宗族是中国传统社会的基础组织，宗族礼仪规范了家族内部的行为准则。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ChineseClanEtiquetteRules,
    name: "中国传统宗族礼仪",
    desc: "传统家族宗族礼仪规范",
    origin: "中国",
    tags: ["社交", "礼仪", "宗族", "传统"]
}

impl ChineseClanEtiquetteRules {
    /// 宗族组织
    pub fn clan_organization(&self) -> Vec<&'static str> {
        vec![
            "族长 - 宗族最高首领，由长辈或德高望重者担任",
            "房长 - 各房分支的首领",
            "宗子 - 长房长子，继承祖业",
            "族老 - 族中有威望的长者",
            "族正 - 协助族长处理族务",
            "族谱 - 记录宗族世系的文献",
            "祠堂 - 祭祀祖先、族议场所",
            "族田 - 宗族公有田产",
        ]
    }

    /// 家族辈分
    pub fn generation_order(&self) -> Vec<&'static str> {
        vec![
            "辈分排列 - 按字辈取名，明尊卑",
            "字辈命名 - 名字中含字辈字",
            "长幼有序 - 同辈按年龄排序",
            "嫡庶之别 - 嫡出高于庶出",
            "大宗小宗 - 长房为大宗，余为小宗",
            "昭穆制度 - 祖先排位的左右次序",
            "族谱记载 - 详细记录世系传承",
            "辈分称呼 - 按辈分称呼，不按年龄",
        ]
    }

    /// 祭祖礼仪
    pub fn ancestor_worship(&self) -> Vec<&'static str> {
        vec![
            "春秋二祭 - 清明、重阳祭祀",
            "祠堂祭祖 - 在祠堂举行",
            "全族参与 - 族人全体参加",
            "主祭人 - 族长或宗子主祭",
            "祭品准备 - 三牲、果品、香烛",
            "行礼顺序 - 按辈分高低依次祭拜",
            "读祝文 - 宣读祭文",
            "分胙 - 祭后分发祭肉",
        ]
    }

    /// 家族集会
    pub fn clan_gathering(&self) -> Vec<&'static str> {
        vec![
            "族会议事 - 讨论族中大事",
            "奖惩族众 - 表扬善行、惩罚恶行",
            "调解纠纷 - 处理族内矛盾",
            "济助贫困 - 帮助困难族人",
            "助学育才 - 资助族中子弟读书",
            "修谱续谱 - 定期更新族谱",
            "修缮祠堂 - 维护祠堂建筑",
            "祭祀活动 - 组织祭祖活动",
        ]
    }

    /// 家规家训
    pub fn family_rules(&self) -> Vec<&'static str> {
        vec![
            "孝敬父母 - 百善孝为先",
            "和睦兄弟 - 兄友弟恭",
            "尊敬长辈 - 不得冒犯",
            "勤俭持家 - 戒奢戒惰",
            "诚实守信 - 戒诈戒欺",
            "读书明理 - 重教兴学",
            "积德行善 - 济困扶危",
            "严守妇道 - 妇女规范",
        ]
    }

    /// 犯禁惩罚
    pub fn clan_punishment(&self) -> Vec<&'static str> {
        vec![
            "不孝父母 - 轻则责罚，重则逐出族",
            "败坏门风 - 记过、罚银",
            "偷盗赌博 - 责罚、赔偿",
            "通奸乱伦 - 逐出族谱",
            "犯上作乱 - 重罚或除名",
            "族内斗殴 - 调解、责罚",
            "不守妇道 - 轻则责骂，重则休书",
            "违法乱纪 - 送官究办",
        ]
    }

    /// 宗族义务
    pub fn clan_duties(&self) -> Vec<&'static str> {
        vec![
            "按时纳粮 - 缴纳族田租粮",
            "参加祭祀 - 按时参加祭祖",
            "服从族规 - 遵守族规家训",
            "互助互济 - 帮助困难族人",
            "保护族产 - 维护宗族财产",
            "培养子弟 - 重视教育培养",
            "维护名誉 - 维护宗族声誉",
            "传承族史 - 了解宗族历史",
        ]
    }

    /// 现代意义
    pub fn modern_significance(&self) -> Vec<&'static str> {
        vec![
            "传承优秀传统文化",
            "增强家族凝聚力",
            "弘扬孝道美德",
            "促进社会和谐",
            "激励后人奋进",
            "保存家族历史",
            "增进族人联系",
            "摒弃封建糟粕",
        ]
    }
}

impl Rule for ChineseClanEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_clan_etiquette")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中国传统宗族礼仪",
            &[
                ("宗族组织", &self.clan_organization()),
                ("家族辈分", &self.generation_order()),
                ("祭祖礼仪", &self.ancestor_worship()),
                ("家族集会", &self.clan_gathering()),
                ("家规家训", &self.family_rules()),
                ("犯禁惩罚", &self.clan_punishment()),
                ("宗族义务", &self.clan_duties()),
                ("现代意义", &self.modern_significance()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clan_etiquette_rules() {
        let rules = ChineseClanEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "中国传统宗族礼仪");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_clan_organization() {
        let rules = ChineseClanEtiquetteRules::new();
        let org = rules.clan_organization();
        assert!(org.iter().any(|o| o.contains("族长")));
        assert!(org.iter().any(|o| o.contains("祠堂")));
        assert!(org.len() >= 6);
    }

    #[test]
    fn test_generation_order() {
        let rules = ChineseClanEtiquetteRules::new();
        let gen = rules.generation_order();
        assert!(gen.iter().any(|g| g.contains("辈分")));
        assert!(gen.iter().any(|g| g.contains("嫡庶")));
        assert!(gen.len() >= 6);
    }

    #[test]
    fn test_family_rules() {
        let rules = ChineseClanEtiquetteRules::new();
        let rules_list = rules.family_rules();
        assert!(rules_list.iter().any(|r| r.contains("孝")));
        assert!(rules_list.iter().any(|r| r.contains("诚")));
        assert!(rules_list.len() >= 6);
    }
}
