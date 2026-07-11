//! 中国书法礼仪 - 传统书法文化的礼仪规范
//!
//! 书法是中国传统文化的瑰宝，书法礼仪涵盖书写、鉴赏、收藏等方面。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ChineseCalligraphyRules,
    name: "中国书法礼仪",
    desc: "传统书法文化的礼仪规范",
    origin: "中国",
    tags: ["社交", "书法", "文化", "艺术"]
}

impl ChineseCalligraphyRules {
    /// 书房布置礼仪
    pub fn study_arrangement(&self) -> Vec<&'static str> {
        vec![
            "书房宜清雅 - 书房应整洁有序，不宜杂乱",
            "案桌宜朝南 - 书桌朝向南方，采光充足",
            "挂画宜书法 - 墙上宜挂书法作品，增添雅韵",
            "置砚宜端砚 - 砚台宜选端砚、歙砚等名砚",
            "笔筒宜古朴 - 笔筒以竹木瓷质为佳",
            "案头宜整洁 - 书桌上不宜堆放杂物",
        ]
    }

    /// 执笔礼仪
    pub fn pen_holding_etiquette(&self) -> Vec<&'static str> {
        vec![
            "执笔贵端正 - 执笔姿势应端正，不宜歪斜",
            "五指齐力 - 拇指食指中指无名指小指协调用力",
            "指实掌虚 - 指尖紧握笔杆，掌心留有空间",
            "腕平掌竖 - 手腕平稳，手掌竖立",
            "笔正心正 - 执笔端正反映心正",
            "呼吸平稳 - 书写时呼吸应平稳自然",
        ]
    }

    /// 磨墨礼仪
    pub fn ink_grinding_etiquette(&self) -> Vec<&'static str> {
        vec![
            "磨墨贵从容 - 磨墨宜慢不宜快",
            "墨浓淡适中 - 墨汁浓淡应适中",
            "按顺时针磨 - 磨墨宜按顺时针方向",
            "墨不宜过厚 - 墨汁不宜过厚，影响流畅",
            "墨不宜过稀 - 墨汁不宜过稀，影响韵味",
            "墨毕宜洗砚 - 磨墨完毕应清洗砚台",
        ]
    }

    /// 书写规范
    pub fn writing_etiquette(&self) -> Vec<&'static str> {
        vec![
            "字宜端正 - 字迹应端正，不宜潦草",
            "笔顺规范 - 笔画顺序应符合规范",
            "布局得当 - 整体布局应均衡得当",
            "大小适宜 - 字体大小应均匀适宜",
            "间距均匀 - 字与字间距应均匀",
            "行气贯通 - 整行字应气韵贯通",
        ]
    }

    /// 落款礼仪
    pub fn signature_etiquette(&self) -> Vec<&'static str> {
        vec![
            "落款宜谦逊 - 落款用字应谦虚，不宜张扬",
            "时间宜准确 - 年月日应准确书写",
            "姓名宜雅称 - 可用雅号别称署名",
            "印章宜得体 - 印章位置大小应得体",
            "引首章 - 作品右上角可盖引首章",
            "压角章 - 作品左下角可盖压角章",
        ]
    }

    /// 赠送书法礼仪
    pub fn gift_etiquette(&self) -> Vec<&'static str> {
        vec![
            "赠前宜审视 - 赠送前应审视作品质量",
            "宜题上款 - 为他人书写宜题上款",
            "不宜擅署 - 不宜擅自署他人姓名",
            "装裱宜精良 - 赠送作品宜装裱精良",
            "宜配礼盒 - 作品宜配礼盒包装",
            "宜亲笔题赠 - 重要赠送宜亲笔题写",
        ]
    }

    /// 鉴赏礼仪
    pub fn appreciation_etiquette(&self) -> Vec<&'static str> {
        vec![
            "宜细观慢品 - 鉴赏应细致耐心",
            "宜先观整体 - 先看整体布局气势",
            "后观细节 - 再看笔画细节技巧",
            "宜静默欣赏 - 鉴赏时不宜喧哗",
            "宜得体称赞 - 赞语应得体，不宜浮夸",
            "不宜妄评 - 不懂不宜妄加评论",
        ]
    }

    /// 收藏礼仪
    pub fn collection_etiquette(&self) -> Vec<&'static str> {
        vec![
            "宜妥善保管 - 作品应妥善保管",
            "宜防潮防蛀 - 注意防潮防虫",
            "宜定期检查 - 定期检查作品状态",
            "宜装裱保存 - 未装裱作品宜及时装裱",
            "宜分类收藏 - 不同作者作品宜分类",
            "宜记录登记 - 收藏作品宜登记造册",
        ]
    }

    /// 禁忌事项
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "不宜嬉笑书写 - 书写时应庄重",
            "不宜酒后书写 - 酒后不宜作书",
            "不宜急躁书写 - 心情急躁不宜作书",
            "不宜污损作品 - 不得污损他人作品",
            "不宜擅改他人作品 - 不得擅自修改他人作品",
            "不宜评头论足 - 不宜对他人作品妄加批评",
        ]
    }

    /// 书法流派
    pub fn calligraphy_styles(&self) -> Vec<&'static str> {
        vec![
            "篆书 - 秦篆为正宗，古朴典雅",
            "隶书 - 汉隶成熟，蚕头燕尾",
            "楷书 - 唐楷为宗，端正端庄",
            "行书 - 王羲之为圣，流畅飘逸",
            "草书 - 张旭为狂，奔放豪迈",
            "魏碑 - 北魏碑刻，雄强朴拙",
        ]
    }

    /// 名家风范
    pub fn famous_calligraphers(&self) -> Vec<&'static str> {
        vec![
            "王羲之 - 书圣，兰亭序为天下第一行书",
            "颜真卿 - 颜体，端庄雄伟",
            "柳公权 - 柳体，骨力遒劲",
            "欧阳询 - 欧体，险劲严谨",
            "苏轼 - 宋四家之首，尚意书风",
            "赵孟頫 - 元代第一，复古书风",
        ]
    }
}

impl Rule for ChineseCalligraphyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_calligraphy")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中国书法礼仪",
            &[
                ("书房布置", &self.study_arrangement()),
                ("执笔礼仪", &self.pen_holding_etiquette()),
                ("磨墨礼仪", &self.ink_grinding_etiquette()),
                ("书写规范", &self.writing_etiquette()),
                ("落款礼仪", &self.signature_etiquette()),
                ("赠送礼仪", &self.gift_etiquette()),
                ("鉴赏礼仪", &self.appreciation_etiquette()),
                ("收藏礼仪", &self.collection_etiquette()),
                ("禁忌事项", &self.taboos()),
                ("书法流派", &self.calligraphy_styles()),
                ("名家风范", &self.famous_calligraphers()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calligraphy_rules_basic() {
        let rules = ChineseCalligraphyRules::new();
        assert_eq!(rules.metadata().name, "中国书法礼仪");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_study_arrangement() {
        let rules = ChineseCalligraphyRules::new();
        let arrangement = rules.study_arrangement();
        assert!(arrangement.iter().any(|a| a.contains("书房")));
        assert!(arrangement.len() >= 6);
    }

    #[test]
    fn test_pen_holding() {
        let rules = ChineseCalligraphyRules::new();
        let holding = rules.pen_holding_etiquette();
        assert!(holding.iter().any(|h| h.contains("执笔")));
        assert!(holding.len() >= 6);
    }

    #[test]
    fn test_writing_etiquette() {
        let rules = ChineseCalligraphyRules::new();
        let writing = rules.writing_etiquette();
        assert!(writing.iter().any(|w| w.contains("端正")));
        assert!(writing.len() >= 6);
    }

    #[test]
    fn test_calligraphy_styles() {
        let rules = ChineseCalligraphyRules::new();
        let styles = rules.calligraphy_styles();
        assert!(styles.iter().any(|s| s.contains("楷书")));
        assert!(styles.iter().any(|s| s.contains("行书")));
        assert!(styles.len() >= 6);
    }
}