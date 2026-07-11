//! 中国藏书礼仪 - 传统藏书文化的礼仪规范
//!
//! 涵盖藏书、借书、读书、护书等传统礼仪。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ChineseBookCollectionRules,
    name: "中国藏书礼仪",
    desc: "传统藏书文化的礼仪规范",
    origin: "中国",
    tags: ["社交", "藏书", "文化", "礼仪"]
}

impl ChineseBookCollectionRules {
    /// 藏书室布置
    pub fn library_arrangement(&self) -> Vec<&'static str> {
        vec![
            "藏书室宜干燥 - 书室应干燥通风",
            "书架宜结实 - 书架应结实稳固",
            "宜分类摆放 - 书籍宜分类摆放",
            "宜编号登记 - 书籍宜编号登记造册",
            "宜防潮防虫 - 书室应防潮防虫",
            "光线不宜直射 - 书室光线不宜直射书籍",
        ]
    }

    /// 藏书选购礼仪
    pub fn book_selection(&self) -> Vec<&'static str> {
        vec![
            "宜购正版书 - 宜购买正版书籍",
            "宜选善本 - 宜选择善本精本",
            "宜辨真伪 - 应辨别书籍真伪",
            "宜察品相 - 应考察书籍品相",
            "不宜贪多 - 藏书不宜贪多不求甚解",
            "宜有重点 - 藏书宜有重点方向",
        ]
    }

    /// 藏书保养礼仪
    pub fn book_maintenance(&self) -> Vec<&'static str> {
        vec![
            "宜定期检查 - 应定期检查书籍状态",
            "宜翻阅通风 - 应定期翻阅通风",
            "宜防霉防蛀 - 应防止霉变虫蛀",
            "宜修补破损 - 应及时修补破损书籍",
            "宜装裱古籍 - 古籍宜装裱保护",
            "宜保持整洁 - 书籍应保持整洁",
        ]
    }

    /// 借书礼仪
    pub fn lending_etiquette(&self) -> Vec<&'static str> {
        vec![
            "借书宜登记 - 借书应登记造册",
            "宜按时归还 - 应按时归还书籍",
            "宜爱护书籍 - 借阅应爱护书籍",
            "不宜涂画 - 不宜在书上涂画",
            "不宜折页 - 不宜折书页",
            "归还宜完好 - 归还书籍应完好无损",
        ]
    }

    /// 还书礼仪
    pub fn returning_etiquette(&self) -> Vec<&'static str> {
        vec![
            "还书宜及时 - 应及时归还书籍",
            "宜检查书籍 - 归还前应检查书籍完好",
            "宜致谢意 - 归还时应表示感谢",
            "破损宜说明 - 若有破损应说明并赔偿",
            "宜包装妥当 - 归还珍贵书籍宜包装妥当",
            "不宜拖欠 - 不宜拖延归还时间",
        ]
    }

    /// 读书礼仪
    pub fn reading_etiquette(&self) -> Vec<&'static str> {
        vec![
            "读书宜静心 - 读书应静心专注",
            "宜恭敬书籍 - 应恭敬对待书籍",
            "宜洗手持书 - 拿书前宜洗手",
            "不宜边吃边读 - 不宜边吃东西边读书",
            "宜端正坐姿 - 读书应端正坐姿",
            "宜细读深思 - 应细致阅读深入思考",
        ]
    }

    /// 批注礼仪
    pub fn annotation_etiquette(&self) -> Vec<&'static str> {
        vec![
            "批注宜谨慎 - 批注应谨慎",
            "宜用铅笔批注 - 宜用铅笔批注便于修改",
            "不宜在珍本批注 - 不宜在珍贵书籍上批注",
            "批注宜简洁 - 批注应简洁明了",
            "不宜过度批注 - 不宜过度批注影响阅读",
            "批注宜有意义 - 批注应有实质意义",
        ]
    }

    /// 藏书赠送礼仪
    pub fn gift_etiquette(&self) -> Vec<&'static str> {
        vec![
            "赠书宜慎重 - 赠送书籍应慎重选择",
            "宜了解喜好 - 应了解对方喜好",
            "宜检查品相 - 赠送前应检查书籍品相",
            "宜包装精美 - 宜精美包装",
            "宜附赠言 - 可附赠言寄语",
            "不宜赠破损书 - 不宜赠送破损书籍",
        ]
    }

    /// 藏书传承礼仪
    pub fn inheritance_etiquette(&self) -> Vec<&'static str> {
        vec![
            "宜传承后代 - 藏书宜传承后代",
            "宜教导爱护 - 应教导后代爱护书籍",
            "宜记录藏书史 - 宜记录藏书历史",
            "宜珍视祖传书 - 应珍视祖传书籍",
            "宜捐赠公共 - 可捐赠公共图书馆",
            "不宜随意处置 - 不宜随意处置珍贵藏书",
        ]
    }

    /// 藏书禁忌
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "不宜随意丢弃 - 不宜随意丢弃书籍",
            "不宜焚烧书籍 - 不宜焚烧书籍",
            "不宜践踏书籍 - 不宜践踏书籍",
            "不宜污损书籍 - 不宜污损书籍",
            "不宜剪裁书籍 - 不宜剪裁书籍",
            "不宜私藏禁书 - 不宜私藏禁书",
        ]
    }

    /// 藏书名家
    pub fn famous_collectors(&self) -> Vec<&'static str> {
        vec![
            "范钦 - 天一阁创始人，藏书传承四百余年",
            "黄丕烈 - 清代藏书大家，专收宋元善本",
            "瞿绍基 - 铁琴铜剑楼主人，藏书世家",
            "陆心源 - 宋楼主人，藏书丰富",
            "叶德辉 - 观古堂主人，藏书目录名家",
            "张元济 - 商务印书馆创始人，古籍保护",
        ]
    }

    /// 藏书楼名称
    pub fn famous_libraries(&self) -> Vec<&'static str> {
        vec![
            "天一阁 - 宁波天一阁，现存最古老藏书楼",
            "文渊阁 - 皇家藏书阁，四库全书珍藏",
            "海源阁 - 山东藏书楼，藏书丰富",
            "铁琴铜剑楼 - 常熟藏书楼，历史悠久",
            "皕宋楼 - 陆心源藏书楼，宋本丰富",
            "嘉业堂 - 刘承干藏书楼，近代著名",
        ]
    }

    /// 藏书格言
    pub fn collection_proverbs(&self) -> Vec<&'static str> {
        vec![
            "书中自有黄金屋，书中自有颜如玉",
            "读书破万卷，下笔如有神",
            "书山有路勤为径，学海无涯苦作舟",
            "藏书万卷可教子，遗金满籯常作灾",
            "书非借不能读也",
            "开卷有益",
        ]
    }
}

impl Rule for ChineseBookCollectionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_book_collection")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中国藏书礼仪",
            &[
                ("藏书室布置", &self.library_arrangement()),
                ("选购礼仪", &self.book_selection()),
                ("保养礼仪", &self.book_maintenance()),
                ("借书礼仪", &self.lending_etiquette()),
                ("还书礼仪", &self.returning_etiquette()),
                ("读书礼仪", &self.reading_etiquette()),
                ("批注礼仪", &self.annotation_etiquette()),
                ("赠送礼仪", &self.gift_etiquette()),
                ("传承礼仪", &self.inheritance_etiquette()),
                ("禁忌事项", &self.taboos()),
                ("藏书名家", &self.famous_collectors()),
                ("藏书楼", &self.famous_libraries()),
                ("藏书格言", &self.collection_proverbs()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_collection_rules_basic() {
        let rules = ChineseBookCollectionRules::new();
        assert_eq!(rules.metadata().name, "中国藏书礼仪");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_library_arrangement() {
        let rules = ChineseBookCollectionRules::new();
        let arr = rules.library_arrangement();
        assert!(arr.iter().any(|a| a.contains("藏书室")));
        assert!(arr.len() >= 6);
    }

    #[test]
    fn test_lending_etiquette() {
        let rules = ChineseBookCollectionRules::new();
        let lending = rules.lending_etiquette();
        assert!(lending.iter().any(|l| l.contains("借书")));
        assert!(lending.len() >= 6);
    }

    #[test]
    fn test_famous_libraries() {
        let rules = ChineseBookCollectionRules::new();
        let libs = rules.famous_libraries();
        assert!(libs.iter().any(|l| l.contains("天一阁")));
        assert!(libs.iter().any(|l| l.contains("文渊阁")));
        assert!(libs.len() >= 6);
    }
}
