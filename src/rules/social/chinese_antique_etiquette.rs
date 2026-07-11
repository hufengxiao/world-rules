//! 中国收藏礼仪 - 传统收藏文化的礼仪规范
//!
//! 涵盖古玩收藏、鉴赏、交易、保管等传统礼仪。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ChineseAntiqueEtiquetteRules,
    name: "中国收藏礼仪",
    desc: "传统收藏文化的礼仪规范",
    origin: "中国",
    tags: ["社交", "收藏", "文化", "礼仪"]
}

impl ChineseAntiqueEtiquetteRules {
    /// 收藏理念
    pub fn collection_philosophy(&self) -> Vec<&'static str> {
        vec![
            "收藏宜修身养性 - 收藏应以修身养性为目的",
            "不宜唯利是图 - 不宜仅以获利为目的",
            "宜深入研究 - 应深入研究藏品知识",
            "宜传承文化 - 收藏应传承传统文化",
            "宜理性收藏 - 应理性收藏，不宜盲目",
            "宜量力而行 - 应量力而行，不宜过度投入",
        ]
    }

    /// 选购礼仪
    pub fn purchase_etiquette(&self) -> Vec<&'static str> {
        vec![
            "选购宜慎重 - 选购应慎重考察",
            "宜辨真伪 - 应仔细辨别真伪",
            "宜察品相 - 应考察藏品品相",
            "宜了解行情 - 应了解市场行情",
            "不宜冲动购买 - 不宜冲动购买",
            "宜请专家鉴定 - 重要藏品宜请专家鉴定",
        ]
    }

    /// 鉴赏礼仪
    pub fn appreciation_etiquette(&self) -> Vec<&'static str> {
        vec![
            "鉴赏宜静心 - 鉴赏应静心专注",
            "宜细观慢品 - 应细致观察品味",
            "宜学习相关知识 - 应学习相关知识",
            "不宜妄加评论 - 不宜妄加评论",
            "宜尊重他人见解 - 应尊重他人见解",
            "宜谦虚请教 - 应谦虚向专家请教",
        ]
    }

    /// 藏品保管礼仪
    pub fn storage_etiquette(&self) -> Vec<&'static str> {
        vec![
            "宜妥善保管 - 应妥善保管藏品",
            "宜分类存放 - 应分类存放",
            "宜防潮防尘 - 应防潮防尘",
            "宜定期检查 - 应定期检查藏品状态",
            "宜记录登记 - 应记录登记藏品信息",
            "宜保险重要藏品 - 重要藏品宜保险",
        ]
    }

    /// 藏品展示礼仪
    pub fn display_etiquette(&self) -> Vec<&'static str> {
        vec![
            "展示宜精心布置 - 应精心布置展示",
            "宜光线适度 - 展示光线应适度",
            "宜说明介绍 - 应有说明介绍",
            "不宜过度展示 - 不宜过度展示珍贵藏品",
            "宜保护藏品安全 - 应保护藏品安全",
            "宜有序陈列 - 应有序陈列",
        ]
    }

    /// 藏品交易礼仪
    pub fn trading_etiquette(&self) -> Vec<&'static str> {
        vec![
            "交易宜诚信 - 交易应诚信为本",
            "宜如实说明 - 应如实说明藏品情况",
            "不宜隐瞒瑕疵 - 不宜隐瞒瑕疵",
            "宜合理定价 - 应合理定价",
            "宜签订协议 - 重要交易宜签订协议",
            "不宜欺诈 - 不宜欺诈欺骗",
        ]
    }

    /// 藏品赠送礼仪
    pub fn gift_etiquette(&self) -> Vec<&'static str> {
        vec![
            "赠藏品宜慎重 - 赠送应慎重选择",
            "宜了解对方喜好 - 应了解对方喜好",
            "宜包装妥当 - 应妥善包装",
            "宜附说明介绍 - 宜附说明介绍",
            "不宜赠赝品 - 不宜赠送赝品",
            "宜珍视赠品 - 收赠品应珍视",
        ]
    }

    /// 藏品传承礼仪
    pub fn inheritance_etiquette(&self) -> Vec<&'static str> {
        vec![
            "宜传承后代 - 藏品宜传承后代",
            "宜教导鉴赏知识 - 应教导后代鉴赏知识",
            "宜记录传承历史 - 宜记录传承历史",
            "宜妥善交接 - 应妥善交接",
            "宜捐赠公共机构 - 可捐赠公共机构",
            "不宜随意处置 - 不宜随意处置珍贵藏品",
        ]
    }

    /// 藏品禁忌
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "不宜收藏盗墓品 - 不宜收藏盗墓出土文物",
            "不宜收藏违禁品 - 不宜收藏违禁物品",
            "不宜故意损毁 - 不宜故意损毁藏品",
            "不宜造假售假 - 不宜造假售假",
            "不宜夸大价值 - 不宜夸大藏品价值",
            "不宜强买强卖 - 不宜强买强卖",
        ]
    }

    /// 收藏品类
    pub fn collection_types(&self) -> Vec<&'static str> {
        vec![
            "瓷器 - 中国瓷器，历史悠久，精品众多",
            "书画 - 中国书画，艺术价值极高",
            "玉器 - 中国玉器，文化内涵丰富",
            "青铜器 - 古代青铜器，历史价值极高",
            "古籍善本 - 古籍善本，文献价值重要",
            "杂项 - 其他各类收藏品",
        ]
    }

    /// 收藏名家
    pub fn famous_collectors(&self) -> Vec<&'static str> {
        vec![
            "项元汴 - 明代收藏大家，藏品丰富",
            "安岐 - 清代收藏家，书画收藏丰富",
            "张伯驹 - 近代收藏家，捐献国宝",
            "王世襄 - 近代收藏家，明式家具名家",
            "马未都 - 当代收藏家，观复博物馆创始人",
            "李敖 - 台湾收藏家，古籍收藏丰富",
        ]
    }

    /// 藏品鉴定要点
    pub fn identification_points(&self) -> Vec<&'static str> {
        vec![
            "材质鉴定 - 应鉴定材质是否相符",
            "工艺鉴定 - 应鉴定工艺时代特征",
            "风格鉴定 - 应鉴定风格特征",
            "款识鉴定 - 应鉴定款识真伪",
            "包浆鉴定 - 应鉴定包浆自然程度",
            "来源鉴定 - 应鉴定来源流传有序",
        ]
    }

    /// 收藏格言
    pub fn collection_proverbs(&self) -> Vec<&'static str> {
        vec![
            "收藏之道，贵在识真",
            "宁可错过，不可错买",
            "藏而不研，等于无藏",
            "物以稀为贵",
            "眼力是收藏的根本",
            "收藏是一种文化修养",
        ]
    }
}

impl Rule for ChineseAntiqueEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_antique_etiquette")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中国收藏礼仪",
            &[
                ("收藏理念", &self.collection_philosophy()),
                ("选购礼仪", &self.purchase_etiquette()),
                ("鉴赏礼仪", &self.appreciation_etiquette()),
                ("保管礼仪", &self.storage_etiquette()),
                ("展示礼仪", &self.display_etiquette()),
                ("交易礼仪", &self.trading_etiquette()),
                ("赠送礼仪", &self.gift_etiquette()),
                ("传承礼仪", &self.inheritance_etiquette()),
                ("禁忌事项", &self.taboos()),
                ("收藏品类", &self.collection_types()),
                ("收藏名家", &self.famous_collectors()),
                ("鉴定要点", &self.identification_points()),
                ("收藏格言", &self.collection_proverbs()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_antique_rules_basic() {
        let rules = ChineseAntiqueEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "中国收藏礼仪");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_collection_philosophy() {
        let rules = ChineseAntiqueEtiquetteRules::new();
        let philosophy = rules.collection_philosophy();
        assert!(philosophy.iter().any(|p| p.contains("修身")));
        assert!(philosophy.len() >= 6);
    }

    #[test]
    fn test_appreciation_etiquette() {
        let rules = ChineseAntiqueEtiquetteRules::new();
        let appreciation = rules.appreciation_etiquette();
        assert!(appreciation.iter().any(|a| a.contains("鉴赏")));
        assert!(appreciation.len() >= 6);
    }

    #[test]
    fn test_collection_types() {
        let rules = ChineseAntiqueEtiquetteRules::new();
        let types = rules.collection_types();
        assert!(types.iter().any(|t| t.contains("瓷器")));
        assert!(types.iter().any(|t| t.contains("书画")));
        assert!(types.len() >= 6);
    }
}