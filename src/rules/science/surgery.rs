//! 外科学定律 - 研究手术治疗疾病的理论和实践
//!
//! 外科学以手术为主要治疗手段，涵盖各器官系统的外科疾病。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: SurgeryRules,
    name: "外科学定律",
    desc: "手术治疗疾病的基本理论和原则",
    origin: "医学",
    tags: ["科学", "医学", "外科"]
}

impl SurgeryRules {
    /// 无菌术
    pub fn aseptic_technique(&self) -> Vec<&'static str> {
        vec![
            "无菌原则: 防止微生物侵入手术区",
            "手术洗手: 肥皂水刷手、消毒液洗手",
            "穿无菌手术衣: 确保手术区域无菌",
            "戴无菌手套: 防止手术者手部细菌污染",
            "手术区消毒: 碘伏或碘酊酒精消毒皮肤",
            "铺无菌单: 隔离手术区与有菌区",
            "术中无菌操作: 严格区分无菌区与非无菌区",
            "器械传递: 无菌器械传递规范",
            "手术室管理: 通风、温度、湿度控制",
            "灭菌方法: 高压蒸汽灭菌、化学灭菌、等离子灭菌",
        ]
    }

    /// 外科感染
    pub fn surgical_infection(&self) -> Vec<&'static str> {
        vec![
            "疖: 单个毛囊及其周围组织的急性化脓性感染",
            "痈: 多个相邻毛囊及其周围组织的急性化脓性感染",
            "急性蜂窝织炎: 皮下疏松结缔组织急性弥漫性感染",
            "丹毒: 皮肤淋巴管的急性感染",
            "急性淋巴管炎: 淋巴管的急性感染",
            "急性淋巴结炎: 淋巴结的急性感染",
            "脓肿: 急性感染后组织坏死液化形成脓腔",
            "败血症: 病原菌侵入血液循环并繁殖",
            "破伤风: 破伤风梭菌产生外毒素引起",
            "气性坏疽: 产气荚膜梭菌引起的急性感染",
        ]
    }

    /// 创伤
    pub fn trauma(&self) -> Vec<&'static str> {
        vec![
            "创伤分类: 按皮肤完整性分开放性和闭合性",
            "创伤愈合: 一期愈合、二期愈合、痂下愈合",
            "清创术: 清除污染物和坏死组织",
            "创面处理: 清洁、消毒、包扎",
            "挤压综合征: 肌肉缺血坏死释放毒素",
            "多发伤: 同一致伤因素造成两处以上损伤",
            "复合伤: 两种以上致伤因素造成的损伤",
            "伤口感染: 细菌侵入伤口引起感染",
            "瘢痕形成: 创面愈合过程中纤维组织增生",
            "功能锻炼: 促进创伤后功能恢复",
        ]
    }

    /// 烧伤
    pub fn burns(&self) -> Vec<&'static str> {
        vec![
            "烧伤面积计算: 九分法、手掌法",
            "烧伤深度: I度、浅II度、深II度、III度",
            "烧伤严重程度: 轻度、中度、重度、特重",
            "烧伤休克期: 伤后48小时内液体渗出高峰",
            "烧伤感染期: 伤后3-7天感染风险最高",
            "烧伤创面处理: 清创、包扎、暴露疗法",
            "烧伤补液公式: 根据面积和体重计算",
            "烧伤后瘢痕: 增生性瘢痕、瘢痕疙瘩",
            "电烧伤: 电流通过人体造成的损伤",
            "化学烧伤: 酸碱等化学物质造成的损伤",
        ]
    }

    /// 麻醉
    pub fn anesthesia(&self) -> Vec<&'static str> {
        vec![
            "全身麻醉: 抑制中枢神经系统产生意识消失",
            "局部麻醉: 阻滞神经传导使局部无痛",
            "椎管内麻醉: 蛛网膜下腔或硬膜外腔阻滞",
            "神经阻滞: 阻滞神经干或神经丛",
            "表面麻醉: 局麻药涂布于黏膜表面",
            "麻醉前评估: ASA分级评估麻醉风险",
            "麻醉监测: 心电图、血压、血氧饱和度",
            "麻醉并发症: 恶心呕吐、喉痉挛、过敏反应",
            "麻醉复苏: 手术后患者意识恢复过程",
            "术后镇痛: 减轻术后疼痛提高舒适度",
        ]
    }

    /// 围手术期处理
    pub fn perioperative_management(&self) -> Vec<&'static str> {
        vec![
            "术前评估: 全面评估患者手术耐受力",
            "术前准备: 禁食、备皮、肠道准备",
            "术前用药: 镇静、抗胆碱药预防误吸",
            "术中监测: 生命体征、出血量监测",
            "术后观察: 监测生命体征和病情变化",
            "术后并发症: 出血、感染、切口裂开",
            "术后疼痛管理: 药物和非药物镇痛",
            "术后营养支持: 肠内营养和肠外营养",
            "早期活动: 促进胃肠功能恢复",
            "切口管理: 换药、拆线、观察愈合",
        ]
    }

    /// 普外科疾病
    pub fn general_surgery(&self) -> Vec<&'static str> {
        vec![
            "甲状腺疾病: 甲状腺功能亢进、甲状腺肿、甲状腺癌",
            "乳腺疾病: 乳腺增生、乳腺纤维腺瘤、乳腺癌",
            "腹外疝: 腹股沟疝、股疝、脐疝、切口疝",
            "阑尾炎: 急性和慢性阑尾炎",
            "肠梗阻: 机械性、动力性、血运性肠梗阻",
            "消化道穿孔: 胃十二指肠穿孔、肠穿孔",
            "胆囊疾病: 胆囊炎、胆囊结石、胆囊癌",
            "胰腺疾病: 胰腺炎、胰腺癌",
            "脾脏疾病: 脾破裂、脾功能亢进",
            "周围血管疾病: 静脉曲张、动脉闭塞症",
        ]
    }

    /// 神经外科疾病
    pub fn neurosurgery(&self) -> Vec<&'static str> {
        vec![
            "颅脑损伤: 脑震荡、脑挫裂伤、颅内血肿",
            "颅内压增高: 头痛、呕吐、视乳头水肿",
            "脑疝: 小脑幕切迹疝、枕骨大孔疝",
            "脑出血: 高血压性脑出血最常见",
            "蛛网膜下腔出血: 动脉瘤破裂最常见",
            "颅内肿瘤: 胶质瘤、脑膜瘤、垂体瘤",
            "椎管内肿瘤: 髓内肿瘤、髓外肿瘤",
            "脑血管疾病: 脑动脉瘤、脑血管畸形",
            "颅脑先天畸形: 脑积水、颅裂、脊柱裂",
            "功能神经外科: 帕金森病、癫痫的外科治疗",
        ]
    }

    /// 心胸外科疾病
    pub fn cardiothoracic_surgery(&self) -> Vec<&'static str> {
        vec![
            "胸部损伤: 肋骨骨折、气胸、血胸",
            "肺大疱: 肺组织内含气囊腔",
            "肺癌: 肺叶切除、全肺切除",
            "食管癌: 食管切除、食管重建",
            "纵隔肿瘤: 胸腺瘤、神经源性肿瘤",
            "心脏瓣膜病: 瓣膜置换术、瓣膜成形术",
            "冠心病: 冠状动脉旁路移植术",
            "先天性心脏病: 房间隔缺损、室间隔缺损",
            "主动脉疾病: 主动脉瘤切除、主动脉夹层手术",
            "胸腔镜手术: 微创胸外科手术",
        ]
    }

    /// 骨科疾病
    pub fn orthopedics(&self) -> Vec<&'static str> {
        vec![
            "骨折: 骨的完整性或连续性中断",
            "骨折愈合: 血肿机化期、原始骨痂期、骨痂改造期",
            "骨折治疗: 复位、固定、功能锻炼",
            "关节脱位: 关节面失去正常对合关系",
            "骨关节炎: 关节软骨退行性变",
            "脊柱疾病: 颈椎病、腰椎间盘突出症",
            "骨肿瘤: 骨软骨瘤、骨肉瘤",
            "运动损伤: 韧带损伤、半月板损伤",
            "手外伤: 手部皮肤、肌腱、神经、血管损伤",
            "断肢(指)再植: 显微外科技术重建肢体",
        ]
    }
}

impl Rule for SurgeryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("surgery")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "外科学定律",
            &[
                ("无菌术", &self.aseptic_technique()),
                ("外科感染", &self.surgical_infection()),
                ("创伤", &self.trauma()),
                ("烧伤", &self.burns()),
                ("麻醉", &self.anesthesia()),
                ("围手术期处理", &self.perioperative_management()),
                ("普外科疾病", &self.general_surgery()),
                ("神经外科疾病", &self.neurosurgery()),
                ("心胸外科疾病", &self.cardiothoracic_surgery()),
                ("骨科疾病", &self.orthopedics()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surgery_rules() {
        let rules = SurgeryRules::new();
        assert!(!rules.aseptic_technique().is_empty());
        assert!(!rules.surgical_infection().is_empty());
        assert!(!rules.general_surgery().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_surgery_metadata() {
        let rules = SurgeryRules::new();
        assert_eq!(rules.metadata().name, "外科学定律");
    }
}
