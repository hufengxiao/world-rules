//! 地质学详细规则
//!
//! 地质学研究地球的物质组成、内部结构、演化历史，
//! 包括岩石矿物、地质构造、地层学和地质过程。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 地质学详细规则集合
pub struct GeologyDetailedRules {
    metadata: RuleMetadata,
}

impl GeologyDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("地质学详细规则", "地质学基本定律和地质过程")
                .with_origin("地球科学")
                .with_tags(vec!["科学".into(), "地质".into(), "地球".into()]),
        }
    }

    /// 岩石类型规则
    pub fn rock_types_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("岩浆岩定律", "火成岩", "由岩浆冷却凝固形成的岩石"),
            ("沉积岩定律", "水成岩", "由沉积物压实胶结形成的岩石"),
            ("变质岩定律", "转化岩", "原有岩石在高温高压下变质"),
            ("花岗岩定律", "深成岩", "深成侵入岩的主要类型"),
            ("玄武岩定律", "喷出岩", "喷出岩的主要类型"),
            ("砂岩定律", "碎屑岩", "由砂粒胶结形成的沉积岩"),
            ("石灰岩定律", "化学岩", "由碳酸钙沉积形成的岩石"),
            ("片岩定律", "变质岩", "具有片状构造的变质岩"),
        ]
    }

    /// 矿物学规则
    pub fn mineralogy_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("矿物定义定律", "天然晶体", "天然形成的结晶固体"),
            ("矿物成分定律", "化学组成", "矿物具有一定的化学成分"),
            ("矿物结构定律", "晶体结构", "矿物内部原子排列方式"),
            ("矿物形态定律", "外部形状", "矿物晶体的外部形态"),
            ("矿物硬度定律", "摩氏硬度", "矿物抗划刻的能力度量"),
            ("矿物解理定律", "破裂面", "矿物沿特定方向破裂"),
            ("矿物光泽定律", "表面光泽", "矿物表面的反光特性"),
            ("矿物分类定律", "分类体系", "按化学成分分类矿物"),
        ]
    }

    /// 地质构造规则
    pub fn geological_structure_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("褶皱定律", "岩层弯曲", "岩层受力弯曲形成的构造"),
            ("断层定律", "岩层断裂", "岩层断裂位移形成的构造"),
            ("节理定律", "裂隙", "岩石中的裂隙构造"),
            ("背斜定律", "向上弯曲", "岩层向上弯曲的褶皱"),
            ("向斜定律", "向下弯曲", "岩层向下弯曲的褶皱"),
            ("正断层定律", "拉张断层", "上盘相对下降的断层"),
            ("逆断层定律", "挤压断层", "上盘相对上升的断层"),
            ("走滑断层定律", "水平位移", "两盘水平相对移动"),
        ]
    }

    /// 板块构造规则
    pub fn plate_tectonics_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("板块边界定律", "接触类型", "板块之间的边界类型"),
            ("离散边界定律", "分离边界", "板块分离形成新地壳"),
            ("汇聚边界定律", "碰撞边界", "板块碰撞地壳消减"),
            ("转换边界定律", "滑动边界", "板块水平滑动边界"),
            ("俯冲带定律", "地壳消减", "海洋板块俯冲消减"),
            ("造山带定律", "山脉形成", "板块碰撞形成山脉"),
            ("洋中脊定律", "海底扩张", "海底扩张中心"),
            ("裂谷定律", "大陆裂谷", "大陆板块分离形成裂谷"),
        ]
    }

    /// 地层学规则
    pub fn stratigraphy_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("地层叠覆定律", "垂直顺序", "老地层在下新地层在上"),
            ("地层原始连续定律", "横向连续", "地层原始沉积时横向连续"),
            ("地层原始水平定律", "水平沉积", "地层原始沉积时近水平"),
            ("化石层序定律", "生物演化", "化石反映地层时代顺序"),
            ("地层对比定律", "时代对比", "不同地区地层时代对比"),
            ("地层单位定律", "分类单位", "地层划分的时间单位"),
            ("沉积相定律", "沉积环境", "沉积物反映沉积环境"),
            ("不整合定律", "沉积间断", "地层间的沉积间断"),
        ]
    }

    /// 地质年代规则
    pub fn geological_time_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("地质年代定律", "时间划分", "地球历史的时间划分"),
            ("前寒武纪定律", "古老时期", "寒武纪之前的漫长时期"),
            ("古生代定律", "古老生物", "541-252百万年前的时代"),
            ("中生代定律", "中间生物", "252-66百万年前的时代"),
            ("新生代定律", "新近生物", "66百万年前至今"),
            ("纪划分定律", "次级单位", "代以下的地质年代单位"),
            ("世划分定律", "更次级", "纪以下的地质年代单位"),
            ("年代地层定律", "时间地层", "按地质年代划分地层"),
        ]
    }

    /// 地质作用规则
    pub fn geological_processes_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("风化作用定律", "岩石分解", "岩石在地表条件下分解"),
            ("侵蚀作用定律", "物质搬运", "风化产物被搬运移走"),
            ("沉积作用定律", "物质堆积", "搬运物质在低处堆积"),
            ("成岩作用定律", "岩石形成", "沉积物转化为沉积岩"),
            ("变质作用定律", "岩石转化", "岩石在高温高压下变化"),
            ("岩浆作用定律", "岩浆活动", "岩浆的形成和活动"),
            ("构造作用定律", "地壳运动", "地壳的运动和变形"),
            ("地震作用定律", "震动破坏", "地震对岩石的破坏作用"),
        ]
    }

    /// 矿产资源规则
    pub fn mineral_resources_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("金属矿产定律", "金属矿", "含金属元素的矿产"),
            ("非金属矿产定律", "非金属", "非金属元素矿产"),
            ("能源矿产定律", "能源矿", "煤、石油、天然气等"),
            ("矿床类型定律", "成因分类", "矿床按成因类型分类"),
            ("成矿作用定律", "矿床形成", "矿床的形成过程"),
            ("矿产勘探定律", "找矿方法", "矿产勘探的技术方法"),
            ("矿产开采定律", "采矿技术", "矿产开采的技术方法"),
            ("矿产资源定律", "资源评价", "矿产资源的评价方法"),
        ]
    }
}

impl Default for GeologyDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for GeologyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("geology_detailed")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_titled_sections(
            "地质学详细规则",
            &[
                ("岩石类型", &self.rock_types_rules()),
                ("矿物学", &self.mineralogy_rules()),
                ("地质构造", &self.geological_structure_rules()),
                ("板块构造", &self.plate_tectonics_rules()),
                ("地层学", &self.stratigraphy_rules()),
                ("地质年代", &self.geological_time_rules()),
                ("地质作用", &self.geological_processes_rules()),
                ("矿产资源", &self.mineral_resources_rules()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geology_detailed_rules() {
        let rules = GeologyDetailedRules::new();
        assert_eq!(rules.metadata().name, "地质学详细规则");
        assert_eq!(rules.rock_types_rules().len(), 8);
        assert_eq!(rules.mineralogy_rules().len(), 8);
        assert_eq!(rules.geological_structure_rules().len(), 8);
        assert_eq!(rules.plate_tectonics_rules().len(), 8);
        assert_eq!(rules.stratigraphy_rules().len(), 8);
        assert_eq!(rules.geological_time_rules().len(), 8);
        assert_eq!(rules.geological_processes_rules().len(), 8);
        assert_eq!(rules.mineral_resources_rules().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_geology_category() {
        let rules = GeologyDetailedRules::new();
        assert_eq!(rules.category().domain, "science");
        assert_eq!(rules.category().name, "geology_detailed");
    }

    #[test]
    fn test_geology_validate() {
        let rules = GeologyDetailedRules::new();
        let ctx = crate::rules::core::ValidateContext::default();
        assert!(rules.validate(&ctx).is_ok());
    }
}