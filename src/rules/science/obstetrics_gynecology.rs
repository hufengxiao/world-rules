//! 妇产科学定律 - 研究女性生殖系统和妊娠分娩
//!
//! 妇产科学涵盖妇科疾病和产科妊娠分娩的诊治原则。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ObstetricsGynecologyRules,
    name: "妇产科学定律",
    desc: "女性生殖系统疾病和妊娠分娩的基本原则",
    origin: "医学",
    tags: ["科学", "医学", "妇产科"]
}

impl ObstetricsGynecologyRules {
    /// 妊娠生理
    pub fn pregnancy_physiology(&self) -> Vec<&'static str> {
        vec![
            "受精: 精子与卵子在输卵管壶腹部结合",
            "着床: 受精后6-7天植入子宫内膜",
            "胎盘: 胎儿与母体进行物质交换的器官",
            "脐带: 连接胎儿与胎盘的管状结构",
            "羊水: 保护胎儿、维持恒温恒压",
            "妊娠期母体变化: 心输出量增加、血容量增多",
            "预产期推算: 末次月经第一天后推280天",
            "妊娠分期: 早期(<14周)、中期(14-27周)、晚期(≥28周)",
            "胎动: 孕妇感觉胎儿活动，18-20周开始",
            "胎心率: 120-160次/分",
        ]
    }

    /// 妊娠诊断
    pub fn pregnancy_diagnosis(&self) -> Vec<&'static str> {
        vec![
            "早期妊娠症状: 停经、早孕反应、尿频",
            "妊娠试验: 尿或血HCG检测",
            "超声检查: 宫内孕囊、胎心搏动",
            "中晚期妊娠体征: 子宫增大、胎动、胎心",
            "胎产式: 胎儿纵轴与母体纵轴的关系",
            "胎先露: 最先进入骨盆入口的胎儿部分",
            "胎方位: 胎儿先露部指示点与母体骨盆的关系",
            "四步触诊法: 判断胎位和胎先露",
            "骨盆测量: 外测量和内测量评估骨盆大小",
            "高危妊娠筛查: 评估妊娠风险因素",
        ]
    }

    /// 正常分娩
    pub fn normal_delivery(&self) -> Vec<&'static str> {
        vec![
            "分娩动因: 内分泌调节、机械性刺激",
            "临产先兆: 假宫缩、胎儿下降感、见红",
            "临产诊断: 规律宫缩、宫口扩张、胎先露下降",
            "产程分期: 第一产程、第二产程、第三产程",
            "第一产程: 规律宫缩至宫口开全，初产妇11-12小时",
            "第二产程: 宫口开全至胎儿娩出，初产妇1-2小时",
            "第三产程: 胎儿娩出至胎盘娩出，5-15分钟",
            "产程图: 监测宫口扩张和胎先露下降",
            "分娩镇痛: 药物镇痛和非药物镇痛",
            "新生儿评估: Apgar评分评估新生儿状态",
        ]
    }

    /// 异常分娩
    pub fn abnormal_delivery(&self) -> Vec<&'static str> {
        vec![
            "产力异常: 子宫收缩乏力或过强",
            "产道异常: 骨产道或软产道异常",
            "胎位异常: 持续性枕后位、臀位、横位",
            "巨大儿: 出生体重≥4000g",
            "头盆不称: 胎头与骨盆大小不相适应",
            "产程延长: 活跃期延长、第二产程延长",
            "肩难产: 胎头娩出后胎肩嵌顿",
            "剖宫产: 手术结束分娩",
            "产钳助产: 产钳协助胎儿娩出",
            "胎头吸引: 负压吸引协助胎儿娩出",
        ]
    }

    /// 妊娠并发症
    pub fn pregnancy_complications(&self) -> Vec<&'static str> {
        vec![
            "流产: 妊娠不足28周胎儿体重不足1000g终止",
            "异位妊娠: 受精卵在子宫腔外着床",
            "妊娠期高血压疾病: 高血压、蛋白尿、水肿",
            "妊娠期糖尿病: 妊娠期首次发现的糖代谢异常",
            "前置胎盘: 胎盘附着于子宫下段或覆盖宫颈内口",
            "胎盘早剥: 正常位置胎盘在胎儿娩出前剥离",
            "胎膜早破: 临产前胎膜破裂",
            "羊水过多: 羊水量超过2000ml",
            "羊水过少: 羊水量少于300ml",
            "多胎妊娠: 一次妊娠同时有两个或以上胎儿",
        ]
    }

    /// 分娩期并发症
    pub fn delivery_complications(&self) -> Vec<&'static str> {
        vec![
            "产后出血: 胎儿娩出后24小时出血≥500ml",
            "子宫破裂: 子宫体部或下段破裂",
            "羊水栓塞: 羊水进入母体血循环引起栓塞",
            "脐带脱垂: 脐带脱出于胎先露下方",
            "脐带缠绕: 脐带缠绕胎儿颈部或肢体",
            "胎盘滞留: 胎儿娩出后30分钟胎盘未娩出",
            "子宫内翻: 子宫内膜面向外翻出",
            "软产道裂伤: 宫颈、阴道、会阴撕裂",
            "产科休克: 失血性休克、感染性休克",
            "产科DIC: 产科原因引起的弥散性血管内凝血",
        ]
    }

    /// 妇科疾病
    pub fn gynecological_diseases(&self) -> Vec<&'static str> {
        vec![
            "阴道炎: 细菌性、念珠菌性、滴虫性阴道炎",
            "宫颈炎: 急性和慢性宫颈炎症",
            "盆腔炎: 女性上生殖道感染性疾病",
            "子宫肌瘤: 子宫平滑肌细胞增生形成的肿瘤",
            "子宫内膜异位症: 子宫内膜组织出现在子宫腔以外",
            "子宫腺肌病: 子宫内膜侵入子宫肌层",
            "卵巢肿瘤: 卵巢良性或恶性肿瘤",
            "宫颈癌: 人乳头瘤病毒感染相关恶性肿瘤",
            "子宫内膜癌: 子宫内膜发生的恶性肿瘤",
            "卵巢癌: 卵巢恶性肿瘤，死亡率最高",
        ]
    }

    /// 月经与内分泌
    pub fn menstruation_endocrine(&self) -> Vec<&'static str> {
        vec![
            "月经周期: 平均28天，卵泡期、排卵期、黄体期",
            "排卵: 下次月经前14天左右",
            "功能失调性子宫出血: 无器质性病变的异常出血",
            "闭经: 原发性闭经和继发性闭经",
            "痛经: 月经期下腹疼痛",
            "经前期综合征: 月经前周期性出现症状",
            "多囊卵巢综合征: 排卵障碍、高雄激素血症",
            "围绝经期综合征: 卵巢功能衰退引起症状",
            "高泌乳素血症: 泌乳素分泌过多",
            "不孕症: 正常性生活1年未避孕未受孕",
        ]
    }

    /// 计划生育
    pub fn family_planning(&self) -> Vec<&'static str> {
        vec![
            "宫内节育器: 放置宫腔内避孕，安全有效",
            "口服避孕药: 复方短效、复方长效避孕药",
            "紧急避孕: 无保护性生活后72小时内",
            "屏障避孕: 避孕套、阴道隔膜",
            "安全期避孕: 避开排卵期性生活",
            "输卵管结扎: 永久性避孕方法",
            "输精管结扎: 男性永久性避孕方法",
            "人工流产: 早期妊娠终止方法",
            "药物流产: 米非司酮配伍米索前列醇",
            "中期引产: 妊娠中期终止方法",
        ]
    }

    /// 妇女保健
    pub fn women_health(&self) -> Vec<&'static str> {
        vec![
            "青春期保健: 月经初潮前后保健指导",
            "婚前保健: 婚前医学检查和咨询",
            "孕前保健: 孕前3个月开始保健准备",
            "孕期保健: 定期产检、营养指导、运动建议",
            "分娩期保健: 安全分娩、减轻疼痛",
            "产褥期保健: 产后42天内恢复保健",
            "哺乳期保健: 母乳喂养指导、乳房护理",
            "更年期保健: 激素补充、骨质疏松预防",
            "老年期保健: 定期体检、慢病管理",
            "妇女病普查: 宫颈癌筛查、乳腺检查",
        ]
    }
}

impl Rule for ObstetricsGynecologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("obstetrics_gynecology")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "妇产科学定律",
            &[
                ("妊娠生理", &self.pregnancy_physiology()),
                ("妊娠诊断", &self.pregnancy_diagnosis()),
                ("正常分娩", &self.normal_delivery()),
                ("异常分娩", &self.abnormal_delivery()),
                ("妊娠并发症", &self.pregnancy_complications()),
                ("分娩期并发症", &self.delivery_complications()),
                ("妇科疾病", &self.gynecological_diseases()),
                ("月经与内分泌", &self.menstruation_endocrine()),
                ("计划生育", &self.family_planning()),
                ("妇女保健", &self.women_health()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obstetrics_gynecology_rules() {
        let rules = ObstetricsGynecologyRules::new();
        assert!(!rules.pregnancy_physiology().is_empty());
        assert!(!rules.normal_delivery().is_empty());
        assert!(!rules.gynecological_diseases().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_obstetrics_gynecology_metadata() {
        let rules = ObstetricsGynecologyRules::new();
        assert_eq!(rules.metadata().name, "妇产科学定律");
    }
}
