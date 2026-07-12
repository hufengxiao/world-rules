//! 急诊医学定律 - 研究急危重症的快速识别和处理
//!
//! 急诊医学强调快速诊断和紧急处理，挽救生命为首要目标。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: EmergencyMedicineRules,
    name: "急诊医学定律",
    desc: "急危重症快速识别和紧急处理的基本原则",
    origin: "医学",
    tags: ["科学", "医学", "急诊"]
}

impl EmergencyMedicineRules {
    /// 急诊评估原则
    pub fn emergency_assessment(&self) -> Vec<&'static str> {
        vec![
            "ABCDE评估: 气道、呼吸、循环、功能障碍、暴露",
            "生命体征: 体温、脉搏、呼吸、血压、血氧",
            "意识评估: AVPU或Glasgow昏迷评分",
            "快速体检: 头颈胸腹脊柱四肢检查",
            "急救优先: 立即处理危及生命的情况",
            "病史采集: 简要询问发病经过",
            "辅助检查: 血常规、心电图、影像检查",
            "风险分层: 区分高危、中危、低危患者",
            "分诊原则: 根据病情紧急程度安排就诊顺序",
            "再评估: 持续监测病情变化",
        ]
    }

    /// 心肺复苏
    pub fn cardiopulmonary_resuscitation(&self) -> Vec<&'static str> {
        vec![
            "心脏骤停识别: 意识丧失、呼吸停止、脉搏消失",
            "胸外按压: 100-120次/分，按压深度5-6cm",
            "开放气道: 头后仰、抬下颌法",
            "人工呼吸: 按压与呼吸比例30:2",
            "电除颤: 室颤或无脉室速时立即除颤",
            "高级生命支持: 气管插管、静脉给药",
            "复苏药物: 肾上腺素、阿托品、胺碘酮",
            "复苏顺序: C-A-B顺序优先按压",
            "终止复苏: 无恢复征象持续30分钟以上",
            "复苏后处理: 维持生命体征稳定",
        ]
    }

    /// 休克处理
    pub fn shock_management(&self) -> Vec<&'static str> {
        vec![
            "低血容量性休克: 补充血容量为主",
            "心源性休克: 增强心肌收缩力",
            "感染性休克: 抗感染和液体复苏",
            "过敏性休克: 肾上腺素为首选药物",
            "神经源性休克: 补液和血管活性药物",
            "休克征象: 低血压、心率快、皮肤湿冷",
            "液体复苏: 晶体液或胶体液快速补充",
            "血管活性药: 多巴胺、去甲肾上腺素",
            "监测指标: 尿量、中心静脉压、乳酸",
            "休克纠正: 血压恢复、组织灌注改善",
        ]
    }

    /// 急性中毒处理
    pub fn acute_poisoning(&self) -> Vec<&'static str> {
        vec![
            "洗胃: 口服中毒6小时内洗胃",
            "催吐: 不适用于意识障碍或腐蚀性毒物",
            "吸附: 活性炭吸附消化道毒物",
            "导泻: 促进毒物从肠道排出",
            "血液净化: 血液透析、血液灌流清除毒物",
            "特效解毒剂: 有机磷中毒用阿托品、解磷定",
            "支持治疗: 维持呼吸循环功能",
            "监测: 生命体征、毒物浓度",
            "预防并发症: 保护重要器官功能",
            "中毒信息: 查询毒物性质和处理方法",
        ]
    }

    /// 创伤急救
    pub fn trauma_emergency(&self) -> Vec<&'static str> {
        vec![
            "创伤评估: 按ABCDE顺序快速评估",
            "多发伤处理: 优先处理危及生命的损伤",
            "气道管理: 保持呼吸道通畅",
            "呼吸支持: 人工呼吸或机械通气",
            "循环支持: 止血、补液、输血",
            "颅脑损伤: 防止颅内压增高",
            "胸部损伤: 处理气胸、血胸",
            "腹部损伤: 排除内脏破裂出血",
            "脊柱损伤: 保护脊髓防止二次损伤",
            "骨折处理: 固定骨折防止移位",
        ]
    }

    /// 急性心血管事件
    pub fn acute_cardiovascular(&self) -> Vec<&'static str> {
        vec![
            "急性心肌梗死: 立即开通闭塞血管",
            "急性冠脉综合征: 抗血小板、抗凝治疗",
            "急性心力衰竭: 减轻心脏负荷、改善心功能",
            "恶性心律失常: 纠正心律、电除颤",
            "高血压急症: 控制血压、保护器官",
            "急性主动脉夹层: 控制血压心率",
            "急性肺栓塞: 抗凝或溶栓治疗",
            "深静脉血栓: 抗凝预防肺栓塞",
            "心包填塞: 紧急心包穿刺引流",
            "急性心源性休克: 血运重建或机械支持",
        ]
    }

    /// 急性神经系统事件
    pub fn acute_neurological(&self) -> Vec<&'static str> {
        vec![
            "急性脑卒中: 快速识别和溶栓评估",
            "缺血性卒中: 4.5小时内静脉溶栓",
            "出血性卒中: 控制血压、降低颅内压",
            "蛛网膜下腔出血: 防止再出血和脑血管痉挛",
            "癫痫持续状态: 控制癫痫发作",
            "颅内压增高: 降颅压、保护脑功能",
            "急性脑膜炎: 抗感染治疗",
            "急性脊髓损伤: 保护脊髓功能",
            "意识障碍: 病因诊断和对症处理",
            "头痛急诊: 排除严重疾病",
        ]
    }

    /// 急性呼吸系统事件
    pub fn acute_respiratory(&self) -> Vec<&'static str> {
        vec![
            "急性呼吸衰竭: 改善通气氧合",
            "急性呼吸窘迫综合征: 机械通气支持",
            "哮喘急性发作: 扩张支气管、氧疗",
            "急性肺水肿: 减轻肺水肿、改善氧合",
            "气胸: 紧急胸腔穿刺排气",
            "气道阻塞: 清除异物或气管插管",
            "急性肺栓塞: 抗凝或溶栓治疗",
            "吸入性肺炎: 抗感染和支持治疗",
            "急性呼吸窘迫: 快速评估和处理",
            "氧疗: 提高血氧饱和度",
        ]
    }

    /// 急腹症处理
    pub fn acute_abdomen(&self) -> Vec<&'static str> {
        vec![
            "急腹症评估: 病史、体检、辅助检查",
            "急性阑尾炎: 手术切除阑尾",
            "急性胆囊炎: 抗感染或手术治疗",
            "急性胰腺炎: 抑制胰腺分泌、支持治疗",
            "肠梗阻: 区分单纯性和绞窄性",
            "消化道穿孔: 紧急手术修补",
            "急性腹膜炎: 抗感染和手术",
            "急性胃扩张: 减压和支持治疗",
            "急性肠缺血: 紧急血管介入或手术",
            "腹部外伤: 排除内脏损伤",
        ]
    }

    /// 危重病监测
    pub fn critical_care_monitoring(&self) -> Vec<&'static str> {
        vec![
            "生命体征监测: 持续监测心率、血压、呼吸",
            "血氧监测: 血氧饱和度和血气分析",
            "心电图监测: 发现心律失常",
            "中心静脉压: 评估血容量状态",
            "有创动脉压: 准确监测血压变化",
            "呼吸功能监测: 呼吸频率、潮气量",
            "尿量监测: 评估肾功能和灌注",
            "神经系统监测: 意识状态、瞳孔变化",
            "实验室监测: 血常规、生化、凝血",
            "影像监测: 胸片、超声、CT",
        ]
    }
}

impl Rule for EmergencyMedicineRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("emergency_medicine")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "急诊医学定律",
            &[
                ("急诊评估原则", &self.emergency_assessment()),
                ("心肺复苏", &self.cardiopulmonary_resuscitation()),
                ("休克处理", &self.shock_management()),
                ("急性中毒处理", &self.acute_poisoning()),
                ("创伤急救", &self.trauma_emergency()),
                ("急性心血管事件", &self.acute_cardiovascular()),
                ("急性神经系统事件", &self.acute_neurological()),
                ("急性呼吸系统事件", &self.acute_respiratory()),
                ("急腹症处理", &self.acute_abdomen()),
                ("危重病监测", &self.critical_care_monitoring()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emergency_medicine_rules() {
        let rules = EmergencyMedicineRules::new();
        assert!(!rules.emergency_assessment().is_empty());
        assert!(!rules.cardiopulmonary_resuscitation().is_empty());
        assert!(!rules.shock_management().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_emergency_medicine_metadata() {
        let rules = EmergencyMedicineRules::new();
        assert_eq!(rules.metadata().name, "急诊医学定律");
    }
}
