//! 中国建筑礼仪 - 传统建筑文化的礼仪规范
//!
//! 涵盖建筑选址、布局、营造、居住等传统礼仪。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ChineseArchitectureEtiquetteRules,
    name: "中国建筑礼仪",
    desc: "传统建筑文化的礼仪规范",
    origin: "中国",
    tags: ["社交", "建筑", "文化", "礼仪"]
}

impl ChineseArchitectureEtiquetteRules {
    /// 建筑选址礼仪
    pub fn site_selection(&self) -> Vec<&'static str> {
        vec![
            "选址宜审慎 - 选址应仔细考察地形环境",
            "宜向阳避阴 - 建筑宜向阳，不宜阴暗",
            "宜避开冲煞 - 不宜选址于道路直冲之处",
            "宜依山傍水 - 传统选址宜依山傍水",
            "宜避开墓地 - 不宜选址于墓地附近",
            "宜考察地质 - 应考察地质是否稳固",
        ]
    }

    /// 建筑布局礼仪
    pub fn layout_etiquette(&self) -> Vec<&'static str> {
        vec![
            "坐北朝南 - 传统建筑宜坐北朝南",
            "中轴对称 - 主要建筑沿中轴线对称布局",
            "前厅后堂 - 前为厅堂，后为居室",
            "左尊右卑 - 传统以东为尊，以西为卑",
            "庭院深深 - 多重庭院，层层深入",
            "宜有回廊 - 宜设回廊连接各建筑",
        ]
    }

    /// 门楼礼仪
    pub fn gate_etiquette(&self) -> Vec<&'static str> {
        vec![
            "门楼宜庄重 - 门楼应庄重大气",
            "门向宜正 - 大门方向宜端正",
            "门槛不宜过高 - 门槛高度应适中",
            "门前宜整洁 - 门前应保持整洁",
            "不宜正对路口 - 大门不宜正对路口",
            "门楣宜装饰 - 门楣可装饰吉祥图案",
        ]
    }

    /// 厅堂礼仪
    pub fn hall_etiquette(&self) -> Vec<&'static str> {
        vec![
            "厅堂宜宽敞 - 厅堂应宽敞明亮",
            "中堂宜设祖位 - 中堂宜设祖先神位",
            "座次有序 - 座次应按尊卑有序",
            "宜挂匾额 - 厅堂可悬挂匾额楹联",
            "宜摆设屏风 - 可摆设屏风分隔空间",
            "不宜堆杂物 - 厅堂不宜堆放杂物",
        ]
    }

    /// 营造礼仪
    pub fn construction_etiquette(&self) -> Vec<&'static str> {
        vec![
            "动土宜择吉日 - 动土应择吉日吉时",
            "宜祭拜土地 - 动土前宜祭拜土地神",
            "宜请工匠 - 宜请经验丰富工匠",
            "宜遵循规矩 - 应遵循传统营造规矩",
            "上梁宜择吉日 - 上梁应择吉日举行仪式",
            "落成宜祭祀 - 落成宜举行祭祀仪式",
        ]
    }

    /// 居住礼仪
    pub fn living_etiquette(&self) -> Vec<&'static str> {
        vec![
            "居室宜整洁 - 居室应保持整洁",
            "卧房宜安静 - 卧房应安静舒适",
            "厨房宜洁净 - 厨房应干净整洁",
            "书房宜清雅 - 书房应清雅安静",
            "庭院宜养护 - 庭院应养护打理",
            "宜保持通风 - 房屋应保持通风",
        ]
    }

    /// 房屋交易礼仪
    pub fn transaction_etiquette(&self) -> Vec<&'static str> {
        vec![
            "买卖宜诚信 - 房屋买卖应诚信",
            "宜签订契约 - 应签订正式契约",
            "宜请中人见证 - 宜请中人见证交易",
            "交割宜清楚 - 房屋交割应清楚",
            "宜支付定金 - 交易宜支付定金",
            "入宅宜择吉日 - 入宅应择吉日",
        ]
    }

    /// 乔迁礼仪
    pub fn moving_etiquette(&self) -> Vec<&'static str> {
        vec![
            "乔迁宜择吉日 - 乔迁应择吉日",
            "宜先安神位 - 入宅宜先安神位祖先位",
            "宜携带米粮 - 入宅应携带米粮象征富足",
            "宜开火做饭 - 入宅应开火做饭象征兴旺",
            "宜请亲友暖房 - 可请亲友暖房",
            "不宜空手入宅 - 入宅不宜空手",
        ]
    }

    /// 房屋禁忌
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "不宜建在坟墓旁 - 不宜建在墓地附近",
            "不宜正对庙宇 - 大门不宜正对庙宇",
            "不宜建在低洼处 - 不宜建在低洼积水处",
            "不宜横梁压顶 - 床位不宜在横梁下",
            "不宜门对门 - 房门不宜正对房门",
            "不宜尖角冲射 - 不宜有尖角冲射房屋",
        ]
    }

    /// 传统建筑类型
    pub fn building_types(&self) -> Vec<&'static str> {
        vec![
            "四合院 - 传统民居，四面房屋围合",
            "三进院落 - 大型民居，三重庭院",
            "徽派建筑 - 徽州风格，白墙黑瓦",
            "江南园林 - 苏州园林，小巧精致",
            "客家土楼 - 客家民居，圆形方形",
            "北方民居 - 北方风格，厚墙保温",
        ]
    }

    /// 建筑装饰
    pub fn building_decoration(&self) -> Vec<&'static str> {
        vec![
            "飞檐翘角 - 传统屋顶，飞檐翘角",
            "雕梁画栋 - 木雕彩绘，精美装饰",
            "瓦当滴水 - 屋檐瓦饰，造型精美",
            "门楣匾额 - 门楣悬挂匾额",
            "楹联对联 - 悬挂楹联对联",
            "石雕石刻 - 门前石狮石刻",
        ]
    }

    /// 建筑风水要点
    pub fn geomancy_points(&self) -> Vec<&'static str> {
        vec![
            "藏风聚气 - 建筑应能藏风聚气",
            "山水环抱 - 宜山水环抱有情",
            "明堂开阔 - 前方明堂应开阔",
            "后有靠山 - 后方应有靠山",
            "左青龙右白虎 - 左右形势宜和谐",
            "五行相生 - 建筑五行宜相生",
        ]
    }
}

impl Rule for ChineseArchitectureEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_architecture_etiquette")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中国建筑礼仪",
            &[
                ("选址礼仪", &self.site_selection()),
                ("布局礼仪", &self.layout_etiquette()),
                ("门楼礼仪", &self.gate_etiquette()),
                ("厅堂礼仪", &self.hall_etiquette()),
                ("营造礼仪", &self.construction_etiquette()),
                ("居住礼仪", &self.living_etiquette()),
                ("交易礼仪", &self.transaction_etiquette()),
                ("乔迁礼仪", &self.moving_etiquette()),
                ("禁忌事项", &self.taboos()),
                ("建筑类型", &self.building_types()),
                ("建筑装饰", &self.building_decoration()),
                ("风水要点", &self.geomancy_points()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_architecture_rules_basic() {
        let rules = ChineseArchitectureEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "中国建筑礼仪");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_site_selection() {
        let rules = ChineseArchitectureEtiquetteRules::new();
        let site = rules.site_selection();
        assert!(site.iter().any(|s| s.contains("选址")));
        assert!(site.len() >= 6);
    }

    #[test]
    fn test_layout_etiquette() {
        let rules = ChineseArchitectureEtiquetteRules::new();
        let layout = rules.layout_etiquette();
        assert!(layout.iter().any(|l| l.contains("坐北朝南")));
        assert!(layout.len() >= 6);
    }

    #[test]
    fn test_building_types() {
        let rules = ChineseArchitectureEtiquetteRules::new();
        let types = rules.building_types();
        assert!(types.iter().any(|t| t.contains("四合院")));
        assert!(types.iter().any(|t| t.contains("园林")));
        assert!(types.len() >= 6);
    }
}