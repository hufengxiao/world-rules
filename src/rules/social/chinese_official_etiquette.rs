//! 中国传统仕途礼仪
//!
//! 传统官场礼仪是古代官员必须遵循的行为规范，
//! 涉及官职、升迁、交往、辞官等方面。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ChineseOfficialEtiquetteRules,
    name: "中国传统仕途礼仪",
    desc: "传统官场礼仪规范",
    origin: "中国",
    tags: ["社交", "礼仪", "官场", "传统"]
}

impl ChineseOfficialEtiquetteRules {
    /// 官职等级
    pub fn official_ranks(&self) -> Vec<&'static str> {
        vec![
            "正一品 - 太师、太傅、太保",
            "从一品 - 少师、少傅、少保",
            "正二品 - 太子太师、太子太傅",
            "从二品 - 各省巡抚、布政使",
            "正三品 - 太府寺卿、光禄寺卿",
            "从三品 - 光禄寺少卿、太仆寺卿",
            "正四品 - 通政使司副使、鸿胪寺卿",
            "从四品 - 翰林院侍读学士、侍讲学士",
        ]
    }

    /// 官服制度
    pub fn official_dress(&self) -> Vec<&'static str> {
        vec![
            "一品 - 仙鹤补子、红宝石顶戴",
            "二品 - 锦鸡补子、红珊瑚顶戴",
            "三品 - 孔雀补子、蓝宝石顶戴",
            "四品 - 云雁补子、青金石顶戴",
            "五品 - 白鹇补子、水晶顶戴",
            "六品 - 鹭鸶补子、砗磲顶戴",
            "七品 - 溪敕补子、素金顶戴",
            "八九品 - 黄鹂、鹌鹑补子、镂花金顶戴",
        ]
    }

    /// 朝见礼仪
    pub fn court_etiquette(&self) -> Vec<&'static str> {
        vec![
            "朝服整齐 - 穿戴正式朝服",
            "按时入宫 - 不可迟到早退",
            "分班站立 - 按品级高低排列",
            "行三跪九叩礼 - 面见皇帝大礼",
            "奏事规范 - 先跪奏，后起立",
            "赐座礼仪 - 皇帝赐座方可坐",
            "退出礼仪 - 面朝皇帝退下",
            "严禁私语 - 朝堂不可私语",
        ]
    }

    /// 官员交往
    pub fn official_interaction(&self) -> Vec<&'static str> {
        vec![
            "同僚礼仪 - 以礼相待，互相尊重",
            "上下级礼仪 - 下级服从上级",
            "拜见礼仪 - 投刺、递名帖",
            "迎送礼仪 - 官员上任、离任",
            "宴请礼仪 - 座次有讲究",
            "礼物往来 - 适度得体",
            "称谓礼仪 - 按官职称呼",
            "书信往来 - 用语得体",
        ]
    }

    /// 升迁礼仪
    pub fn promotion_etiquette(&self) -> Vec<&'static str> {
        vec![
            "接旨谢恩 - 跪接圣旨，三呼万岁",
            "拜别旧任 - 与同僚告别",
            "赴任准备 - 整理行装、交接",
            "接任仪式 - 正式交接印信",
            "拜见上司 - 新官拜见上级",
            "接风洗尘 - 同僚设宴欢迎",
            "谢恩奏折 - 上奏谢恩",
            "安民告示 - 张贴安民告示",
        ]
    }

    /// 辞官礼仪
    pub fn resignation_etiquette(&self) -> Vec<&'static str> {
        vec![
            "上辞呈 - 向皇帝或上级递交辞呈",
            "请求批准 - 等待批准方可离职",
            "交接工作 - 完成交接手续",
            "归还公物 - 印信、官服、文书",
            "告别同僚 - 与同僚道别",
            "回乡仪式 - 衣锦还乡或告老还乡",
            "谢恩上奏 - 上奏感谢皇恩",
            "隐退生活 - 归隐田园，不问政事",
        ]
    }

    /// 官场禁忌
    pub fn official_taboos(&self) -> Vec<&'static str> {
        vec![
            "不可结党营私",
            "不可贪污受贿",
            "不可越级上奏",
            "不可泄露机密",
            "不可怠慢上级",
            "不可欺压下属",
            "不可私通敌国",
            "不可徇私枉法",
        ]
    }

    /// 官员修养
    pub fn official_virtues(&self) -> Vec<&'static str> {
        vec![
            "清正廉洁 - 不贪不腐，两袖清风",
            "勤政爱民 - 勤于政事，爱护百姓",
            "公正无私 - 秉公办事，不徇私情",
            "谦虚谨慎 - 戒骄戒躁，谦虚待人",
            "克己奉公 - 约束自己，服务公家",
            "明辨是非 - 审理案件，明察秋毫",
            "知人善任 - 选拔人才，任人唯贤",
            "以身作则 - 身先士卒，为人表率",
        ]
    }
}

impl Rule for ChineseOfficialEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_official_etiquette")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中国传统仕途礼仪",
            &[
                ("官职等级", &self.official_ranks()),
                ("官服制度", &self.official_dress()),
                ("朝见礼仪", &self.court_etiquette()),
                ("官员交往", &self.official_interaction()),
                ("升迁礼仪", &self.promotion_etiquette()),
                ("辞官礼仪", &self.resignation_etiquette()),
                ("官场禁忌", &self.official_taboos()),
                ("官员修养", &self.official_virtues()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_official_etiquette_rules() {
        let rules = ChineseOfficialEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "中国传统仕途礼仪");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_official_ranks() {
        let rules = ChineseOfficialEtiquetteRules::new();
        let ranks = rules.official_ranks();
        assert!(ranks.iter().any(|r| r.contains("一品")));
        assert!(ranks.iter().any(|r| r.contains("四品")));
        assert!(ranks.len() >= 6);
    }

    #[test]
    fn test_official_dress() {
        let rules = ChineseOfficialEtiquetteRules::new();
        let dress = rules.official_dress();
        assert!(dress.iter().any(|d| d.contains("仙鹤")));
        assert!(dress.iter().any(|d| d.contains("孔雀")));
        assert!(dress.len() >= 6);
    }

    #[test]
    fn test_court_etiquette() {
        let rules = ChineseOfficialEtiquetteRules::new();
        let court = rules.court_etiquette();
        assert!(court.iter().any(|c| c.contains("跪")));
        assert!(court.iter().any(|c| c.contains("叩")));
        assert!(court.len() >= 6);
    }
}