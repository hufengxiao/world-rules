//! 中国棋类礼仪 - 传统棋艺活动的礼仪规范
//!
//! 涵盖象棋、围棋等棋类活动的对弈、观棋、待客等礼仪。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ChineseChessEtiquetteRules,
    name: "中国棋类礼仪",
    desc: "传统棋艺活动的礼仪规范",
    origin: "中国",
    tags: ["社交", "棋类", "文化", "礼仪"]
}

impl ChineseChessEtiquetteRules {
    /// 对弈准备礼仪
    pub fn preparation_etiquette(&self) -> Vec<&'static str> {
        vec![
            "棋桌宜平整 - 棋桌应平整稳定",
            "棋具宜整洁 - 棋盘棋子应干净整洁",
            "座位宜舒适 - 座位应舒适，不宜高低悬殊",
            "环境宜安静 - 对弈环境应安静无干扰",
            "光线宜适中 - 光线应充足但不刺眼",
            "茶水宜备好 - 可备茶水，不宜过量饮酒",
        ]
    }

    /// 落子礼仪
    pub fn move_etiquette(&self) -> Vec<&'static str> {
        vec![
            "落子贵轻稳 - 落子应轻稳，不宜重重敲击",
            "落子不悔 - 落子后不应悔棋",
            "思考从容 - 思考应从容，不宜急躁",
            "不宜催促 - 不宜催促对方落子",
            "落子位置准确 - 落子应准确放置于棋位",
            "移子宜慎重 - 移动棋子应慎重",
        ]
    }

    /// 对弈交谈礼仪
    pub fn conversation_etiquette(&self) -> Vec<&'static str> {
        vec![
            "对弈不宜多言 - 对弈过程中不宜多言",
            "胜不骄败不馁 - 赢不骄傲，输不气馁",
            "不宜讥讽 - 不宜讥讽对方失误",
            "宜谦虚请教 - 可谦虚请教高手",
            "不宜争执 - 不宜因棋局争执",
            "复盘宜友善 - 复盘讨论应友善",
        ]
    }

    /// 观棋礼仪
    pub fn watching_etiquette(&self) -> Vec<&'static str> {
        vec![
            "观棋不语真君子 - 观棋不应插话支招",
            "不宜指点 - 不宜指点棋手落子",
            "不宜喧哗 - 观棋不宜喧哗",
            "不宜干扰 - 不宜干扰棋手思考",
            "不宜靠棋桌太近 - 观棋不宜靠棋桌过近",
            "不宜随意走动 - 观棋不宜随意走动",
        ]
    }

    /// 棋室礼仪
    pub fn chess_room_etiquette(&self) -> Vec<&'static str> {
        vec![
            "进室宜安静 - 进入棋室应安静",
            "不宜高声交谈 - 棋室内不宜高声交谈",
            "不宜吸烟 - 棋室不宜吸烟",
            "宜爱护棋具 - 应爱护棋室棋具",
            "棋毕宜整理 - 对弈完毕应整理棋具",
            "宜遵守规矩 - 应遵守棋室规定",
        ]
    }

    /// 棋艺拜师礼仪
    pub fn teacher_etiquette(&self) -> Vec<&'static str> {
        vec![
            "拜师宜恭敬 - 拜师应恭敬郑重",
            "宜敬茶行礼 - 拜师宜敬茶行礼",
            "宜虚心学习 - 学习应虚心",
            "宜勤加练习 - 应勤加练习棋艺",
            "宜尊师重道 - 应尊重师长棋道",
            "不宜懈怠 - 学习不宜懈怠",
        ]
    }

    /// 棋赛礼仪
    pub fn competition_etiquette(&self) -> Vec<&'static str> {
        vec![
            "参赛宜准时 - 参赛应准时到场",
            "宜守比赛规则 - 应遵守比赛规则",
            "不宜违规 - 不宜违规操作",
            "宜尊重裁判 - 应尊重裁判判决",
            "宜尊重对手 - 应尊重对手",
            "赛后宜复盘交流 - 赛后宜复盘交流",
        ]
    }

    /// 棋类待客礼仪
    pub fn hospitality_etiquette(&self) -> Vec<&'static str> {
        vec![
            "邀棋宜礼貌 - 邀人对弈应礼貌",
            "宜备好棋具 - 应备好棋具",
            "宜设茶座 - 宜设茶座招待",
            "宜不强求 - 不宜强求他人对弈",
            "宜照顾客人水平 - 宜照顾客人棋艺水平",
            "棋毕宜奉茶 - 对弈完毕宜奉茶",
        ]
    }

    /// 棋品修养
    pub fn character_building(&self) -> Vec<&'static str> {
        vec![
            "棋品见人品 - 棋品反映人品",
            "宜以棋修身 - 通过棋艺修身养性",
            "宜静心养气 - 棋艺可静心养气",
            "宜磨练意志 - 棋艺可磨练意志",
            "宜培养耐心 - 棋艺培养耐心",
            "宜提高智慧 - 棋艺提高智慧",
        ]
    }

    /// 棋类禁忌
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "不宜边下边食 - 不宜边下棋边吃东西",
            "不宜酒后下棋 - 酒后不宜下棋",
            "不宜作弊 - 不宜作弊作弊",
            "不宜争执棋局 - 不宜因棋局争执",
            "不宜悔棋赖账 - 不宜悔棋赖账",
            "不宜扰乱对方 - 不宜扰乱对方思考",
        ]
    }

    /// 棋类种类
    pub fn chess_types(&self) -> Vec<&'static str> {
        vec![
            "象棋 - 中国象棋，历史悠久，雅俗共赏",
            "围棋 - 围棋，策略深邃，棋道高深",
            "五子棋 - 五子棋，规则简单，趣味性强",
            "军棋 - 军棋，军旅风格，趣味独特",
            "跳棋 - 跳棋，轻松休闲，适合聚会",
            "麻将 - 麻将，四人博弈，技巧丰富",
        ]
    }

    /// 棋谚棋理
    pub fn chess_proverbs(&self) -> Vec<&'static str> {
        vec![
            "观棋不语真君子，把酒多言是小人",
            "棋逢对手难藏幸，将遇良才好用功",
            "落子无悔大丈夫",
            "棋高一着，束手缚脚",
            "当局者迷，旁观者清",
            "一着不慎，满盘皆输",
        ]
    }
}

impl Rule for ChineseChessEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_chess_etiquette")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中国棋类礼仪",
            &[
                ("对弈准备", &self.preparation_etiquette()),
                ("落子礼仪", &self.move_etiquette()),
                ("交谈礼仪", &self.conversation_etiquette()),
                ("观棋礼仪", &self.watching_etiquette()),
                ("棋室礼仪", &self.chess_room_etiquette()),
                ("拜师礼仪", &self.teacher_etiquette()),
                ("棋赛礼仪", &self.competition_etiquette()),
                ("待客礼仪", &self.hospitality_etiquette()),
                ("棋品修养", &self.character_building()),
                ("棋类禁忌", &self.taboos()),
                ("棋类种类", &self.chess_types()),
                ("棋谚棋理", &self.chess_proverbs()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chess_etiquette_rules_basic() {
        let rules = ChineseChessEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "中国棋类礼仪");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_preparation_etiquette() {
        let rules = ChineseChessEtiquetteRules::new();
        let prep = rules.preparation_etiquette();
        assert!(prep.iter().any(|p| p.contains("棋桌")));
        assert!(prep.len() >= 6);
    }

    #[test]
    fn test_watching_etiquette() {
        let rules = ChineseChessEtiquetteRules::new();
        let watching = rules.watching_etiquette();
        assert!(watching.iter().any(|w| w.contains("观棋不语")));
        assert!(watching.len() >= 6);
    }

    #[test]
    fn test_chess_proverbs() {
        let rules = ChineseChessEtiquetteRules::new();
        let proverbs = rules.chess_proverbs();
        assert!(proverbs.iter().any(|p| p.contains("观棋不语")));
        assert!(proverbs.iter().any(|p| p.contains("落子无悔")));
        assert!(proverbs.len() >= 6);
    }
}
