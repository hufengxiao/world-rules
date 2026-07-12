//! 儿科学定律 - 研究儿童生长发育和疾病防治
//!
//! 儿科学研究从胎儿到青少年期的生长发育规律和疾病诊治。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: PediatricsRules,
    name: "儿科学定律",
    desc: "儿童生长发育和疾病防治的基本原则",
    origin: "医学",
    tags: ["科学", "医学", "儿科"]
}

impl PediatricsRules {
    /// 小儿生长发育
    pub fn growth_development(&self) -> Vec<&'static str> {
        vec![
            "体重: 出生3kg，1岁9kg，2岁12kg估算公式",
            "身高: 出生50cm，1岁75cm，2岁87cm",
            "头围: 出生34cm，1岁46cm，2岁48cm",
            "胸围: 出生小于头围，1岁时相等",
            "牙齿: 6个月萌出第一颗乳牙，2.5岁出齐20颗",
            "骨化中心: 腕部骨化中心数=年龄+1",
            "运动发育: 二抬四翻六会坐七滚八爬周会走",
            "语言发育: 发音、咿呀、单词、短句阶段",
            "青春期发育: 性征出现和体格快速增长",
            "生长发育评价: 百分位法、标准差法",
        ]
    }

    /// 小儿营养
    pub fn pediatric_nutrition(&self) -> Vec<&'static str> {
        vec![
            "母乳喂养: 6个月内纯母乳喂养最理想",
            "人工喂养: 无法母乳时使用配方奶",
            "混合喂养: 母乳不足时添加配方奶",
            "辅食添加: 6个月开始逐步添加辅食",
            "营养素需求: 蛋白质、脂肪、碳水化合物比例适宜",
            "维生素D: 预防佝偻病的重要营养素",
            "铁剂补充: 预防缺铁性贫血",
            "微量元素: 锌、碘、硒等对生长发育重要",
            "营养不良: 蛋白质-能量营养不良",
            "肥胖症: 能量摄入过多导致体重超标",
        ]
    }

    /// 新生儿疾病
    pub fn neonatal_diseases(&self) -> Vec<&'static str> {
        vec![
            "新生儿窒息: 出生时无自主呼吸或呼吸抑制",
            "新生儿呼吸窘迫综合征: 肺表面活性物质缺乏",
            "新生儿黄疸: 胆红素在体内积聚引起皮肤黄染",
            "新生儿溶血病: 母婴血型不合引起溶血",
            "新生儿败血症: 细菌侵入血液循环引起感染",
            "新生儿肺炎: 宫内、产时或产后感染引起",
            "新生儿颅内出血: 缺氧或产伤引起",
            "新生儿寒冷损伤综合征: 低体温和皮肤硬肿",
            "新生儿坏死性小肠结肠炎: 肠道缺血缺氧损伤",
            "新生儿低血糖: 血糖低于2.2mmol/L",
        ]
    }

    /// 营养性疾病
    pub fn nutritional_diseases(&self) -> Vec<&'static str> {
        vec![
            "佝偻病: 维生素D缺乏导致钙磷代谢紊乱",
            "维生素D缺乏性手足搐搦症: 低钙惊厥",
            "营养不良: 蛋白质和能量摄入不足",
            "缺铁性贫血: 铁摄入不足或丢失过多",
            "巨幼细胞贫血: 维生素B12或叶酸缺乏",
            "维生素A缺乏: 夜盲症、眼干燥症",
            "锌缺乏症: 食欲减退、生长发育迟缓",
            "碘缺乏病: 甲状腺肿、智力发育障碍",
            "肥胖症: 能量摄入超过消耗",
            "维生素中毒: 脂溶性维生素过量摄入",
        ]
    }

    /// 呼吸系统疾病
    pub fn respiratory_diseases(&self) -> Vec<&'static str> {
        vec![
            "急性上呼吸道感染: 最常见的小儿呼吸系统疾病",
            "急性支气管炎: 支气管黏膜急性炎症",
            "毛细支气管炎: 婴幼儿常见下呼吸道感染",
            "肺炎: 支气管肺炎最常见",
            "支气管哮喘: 慢性气道炎症和气道高反应性",
            "急性喉炎: 喉部黏膜急性炎症致喉梗阻",
            "呼吸道异物: 好发于幼儿，突发窒息",
            "先天性喉喘鸣: 喉软骨发育不良",
            "肺不张: 支气管阻塞或肺组织受压",
            "肺气肿: 肺泡过度充气膨胀",
        ]
    }

    /// 消化系统疾病
    pub fn digestive_diseases(&self) -> Vec<&'static str> {
        vec![
            "口炎: 鹅口疮、疱疹性口炎、溃疡性口炎",
            "胃食管反流: 下食管括约肌功能不全",
            "胃炎: 幽门螺杆菌感染最常见",
            "消化性溃疡: 胃和十二指肠溃疡",
            "小儿腹泻病: 多病原多因素引起",
            "急性坏死性肠炎: 肠道出血坏死性炎症",
            "肠套叠: 婴幼儿最常见的急腹症",
            "先天性巨结肠: 结肠神经节细胞缺如",
            "胆道闭锁: 肝内外胆管闭锁",
            "肝炎: 病毒性肝炎以甲型最常见",
        ]
    }

    /// 循环系统疾病
    pub fn cardiovascular_diseases(&self) -> Vec<&'static str> {
        vec![
            "先天性心脏病: 胎儿期心脏发育异常",
            "室间隔缺损: 最常见的先天性心脏病",
            "房间隔缺损: 房间隔发育异常",
            "动脉导管未闭: 出生后导管未闭合",
            "法洛四联症: 肺动脉狭窄、室缺、主动脉骑跨、右室肥厚",
            "病毒性心肌炎: 病毒感染引起心肌炎症",
            "充血性心力衰竭: 心输出量不能满足机体需要",
            "心律失常: 早搏、心动过速、传导阻滞",
            "感染性心内膜炎: 心内膜微生物感染",
            "川崎病: 全身中小动脉炎",
        ]
    }

    /// 泌尿系统疾病
    pub fn urinary_diseases(&self) -> Vec<&'static str> {
        vec![
            "急性肾小球肾炎: 链球菌感染后免疫反应",
            "肾病综合征: 大量蛋白尿、低蛋白血症、水肿",
            "泌尿道感染: 细菌侵入尿路引起炎症",
            "急性肾衰竭: 肾功能急剧下降",
            "溶血尿毒综合征: 微血管病性溶血性贫血",
            "肾小管酸中毒: 肾小管酸化功能障碍",
            " Bartter综合征: 肾小管离子转运异常",
            "遗尿症: 5岁以上睡眠中不自主排尿",
            "先天性肾病: 出生时或生后不久发病",
            "肾发育不良: 肾脏结构和功能异常",
        ]
    }

    /// 神经系统疾病
    pub fn neurological_diseases(&self) -> Vec<&'static str> {
        vec![
            "癫痫: 脑神经元异常放电引起发作",
            "脑性瘫痪: 出生前至生后1月内脑损伤",
            "化脓性脑膜炎: 细菌感染引起软脑膜炎症",
            "病毒性脑炎: 病毒感染引起脑实质炎症",
            "热性惊厥: 发热引起的惊厥发作",
            "智力障碍: 智力功能显著低于平均水平",
            "注意缺陷多动障碍: 注意力不集中、多动冲动",
            "抽动障碍: 不自主运动和发声抽动",
            "孤独症谱系障碍: 社交沟通障碍、刻板行为",
            "重症肌无力: 神经肌肉接头传递障碍",
        ]
    }

    /// 感染性疾病
    pub fn infectious_diseases(&self) -> Vec<&'static str> {
        vec![
            "麻疹: 麻疹病毒引起，口腔Koplik斑",
            "风疹: 风疹病毒引起，耳后淋巴结肿大",
            "幼儿急疹: 人疱疹病毒6型引起，热退疹出",
            "水痘: 水痘-带状疱疹病毒引起",
            "猩红热: A组β溶血性链球菌感染",
            "流行性腮腺炎: 腮腺炎病毒引起腮腺肿大",
            "手足口病: 肠道病毒引起，手、足、口皮疹",
            "百日咳: 百日咳鲍特菌引起阵发性痉挛性咳嗽",
            "细菌性痢疾: 志贺菌引起发热、腹痛、脓血便",
            "结核病: 结核分枝杆菌感染",
        ]
    }
}

impl Rule for PediatricsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("pediatrics")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "儿科学定律",
            &[
                ("小儿生长发育", &self.growth_development()),
                ("小儿营养", &self.pediatric_nutrition()),
                ("新生儿疾病", &self.neonatal_diseases()),
                ("营养性疾病", &self.nutritional_diseases()),
                ("呼吸系统疾病", &self.respiratory_diseases()),
                ("消化系统疾病", &self.digestive_diseases()),
                ("循环系统疾病", &self.cardiovascular_diseases()),
                ("泌尿系统疾病", &self.urinary_diseases()),
                ("神经系统疾病", &self.neurological_diseases()),
                ("感染性疾病", &self.infectious_diseases()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pediatrics_rules() {
        let rules = PediatricsRules::new();
        assert!(!rules.growth_development().is_empty());
        assert!(!rules.neonatal_diseases().is_empty());
        assert!(!rules.infectious_diseases().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_pediatrics_metadata() {
        let rules = PediatricsRules::new();
        assert_eq!(rules.metadata().name, "儿科学定律");
    }
}
