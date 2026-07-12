//! 精神病学定律 - 研究精神疾病的诊断和治疗
//!
//! 精神病学涉及精神障碍的病因、发病机制、诊断和治疗。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: PsychiatryRules,
    name: "精神病学定律",
    desc: "精神障碍诊断和治疗的基本原则",
    origin: "医学",
    tags: ["科学", "医学", "精神病学"]
}

impl PsychiatryRules {
    /// 精神疾病诊断原则
    pub fn diagnostic_principles(&self) -> Vec<&'static str> {
        vec![
            "症状评估: 精神症状的识别和描述",
            "病史采集: 详细的精神病史和个人史",
            "精神检查: 对患者的精神状态进行系统检查",
            "诊断标准: ICD-10、DSM-5诊断标准应用",
            "鉴别诊断: 排除躯体疾病和其他精神障碍",
            "病程评估: 急性、亚急性、慢性病程判断",
            "严重程度: 轻、中、重度分级",
            "功能评估: 社会功能受损程度评定",
            "风险评估: 自杀、暴力等风险评估",
            "多轴诊断: 全面评估患者精神状态和相关因素",
        ]
    }

    /// 精神分裂症
    pub fn schizophrenia(&self) -> Vec<&'static str> {
        vec![
            "阳性症状: 幻觉、妄想、思维紊乱",
            "阴性症状: 情感淡漠、意志减退、社交退缩",
            "认知症状: 注意力、记忆力、执行功能障碍",
            "前驱期症状: 起病前的行为和情绪改变",
            "急性期: 精神症状明显加重",
            "残留期: 急性期后遗留症状",
            " Schneider一级症状: 思维插入、思维被夺等",
            "发病年龄: 多在青壮年起病",
            "病程特点: 多为慢性反复发作",
            "治疗原则: 抗精神病药物为主",
        ]
    }

    /// 情感障碍
    pub fn mood_disorders(&self) -> Vec<&'static str> {
        vec![
            "抑郁症: 情绪低落、兴趣减退、精力下降",
            "躁狂症: 情绪高涨、活动增多、言语增多",
            "双相障碍: 抑郁和躁狂交替发作",
            "持续性心境障碍: 情绪波动持续存在",
            "抑郁发作症状: 睡眠障碍、食欲改变、自杀观念",
            "躁狂发作症状: 夸大观念、冲动行为、睡眠需求减少",
            "快速循环型: 每年发作4次以上",
            "混合发作: 同时出现抑郁和躁狂症状",
            "季节性情感障碍: 与季节变化相关的情感障碍",
            "产后抑郁: 分娩后出现的抑郁症状",
        ]
    }

    /// 神经症
    pub fn neurotic_disorders(&self) -> Vec<&'static str> {
        vec![
            "焦虑症: 过度焦虑和担忧",
            "恐惧症: 对特定物体或情境的恐惧",
            "强迫症: 强迫思维和强迫行为",
            "躯体形式障碍: 过度关注躯体症状",
            "分离转换障碍: 心理因素引起的感觉运动障碍",
            "神经衰弱: 精神易兴奋和易疲劳",
            "广泛性焦虑障碍: 持续的焦虑和紧张",
            "惊恐障碍: 反复的惊恐发作",
            "社交恐惧症: 对社交情境的恐惧和回避",
            "特定恐惧症: 对特定物体的恐惧",
        ]
    }

    /// 器质性精神障碍
    pub fn organic_disorders(&self) -> Vec<&'static str> {
        vec![
            "谵妄: 急性起病的意识障碍和认知功能障碍",
            "痴呆: 慢性进行性认知功能减退",
            "遗忘综合征: 记忆障碍为主的综合征",
            "脑卒中后精神障碍: 脑血管病引起的精神症状",
            "脑外伤后精神障碍: 脑外伤引起的精神症状",
            "脑肿瘤精神症状: 脑肿瘤引起的精神障碍",
            "癫痫性精神障碍: 癫痫引起的精神症状",
            "内分泌疾病精神症状: 甲状腺等功能异常引起",
            "酒精所致精神障碍: 长期饮酒引起的精神障碍",
            "药物所致精神障碍: 药物滥用引起的精神症状",
        ]
    }

    /// 人格障碍
    pub fn personality_disorders(&self) -> Vec<&'static str> {
        vec![
            "偏执型人格障碍: 广泛怀疑和不信任",
            "分裂样人格障碍: 社交退缩和情感冷淡",
            "分裂型人格障碍: 思维和行为怪异",
            "反社会型人格障碍: 违反社会规范和道德",
            "边缘型人格障碍: 情绪不稳定和人际关系紊乱",
            "表演型人格障碍: 过度情绪化和寻求关注",
            "自恋型人格障碍: 夸大自我和缺乏同理心",
            "回避型人格障碍: 社交抑制和能力不足感",
            "依赖型人格障碍: 过度依赖他人",
            "强迫型人格障碍: 完美主义和固守规则",
        ]
    }

    /// 精神发育迟滞
    pub fn intellectual_disability(&self) -> Vec<&'static str> {
        vec![
            "轻度智力障碍: IQ 50-69，可接受教育",
            "中度智力障碍: IQ 35-49，可进行简单训练",
            "重度智力障碍: IQ 20-34，需要长期照顾",
            "极重度智力障碍: IQ <20，完全依赖照顾",
            "遗传因素: 染色体异常、基因突变",
            "环境因素: 围产期损伤、营养不良",
            "早期干预: 特殊教育和康复训练",
            "行为问题: 注意缺陷、冲动行为",
            "社交能力: 社交技能训练",
            "家庭支持: 家庭教育和心理支持",
        ]
    }

    /// 儿童青少年精神障碍
    pub fn child_psychiatric_disorders(&self) -> Vec<&'static str> {
        vec![
            "注意缺陷多动障碍: 注意力不集中、多动、冲动",
            "对立违抗障碍: 对成人权威的对抗行为",
            "品行障碍: 反复的违反社会规范行为",
            "抽动障碍: 不自主的运动或发声抽动",
            " Tourette综合征: 多发性运动和发声抽动",
            "分离焦虑症: 与依恋对象分离时的过度焦虑",
            "选择性缄默症: 特定情境下不能说话",
            "学校恐惧症: 对学校情境的恐惧和回避",
            "儿童抑郁症: 儿童期出现的抑郁症状",
            "儿童焦虑症: 儿童期出现的焦虑症状",
        ]
    }

    /// 精神药物治疗
    pub fn psychopharmacology(&self) -> Vec<&'static str> {
        vec![
            "抗精神病药: 多巴胺受体阻断，治疗精神病症状",
            "典型抗精神病药: 氯丙嗪、氟哌啶醇等",
            "非典型抗精神病药: 利培酮、奥氮平、喹硫平",
            "抗抑郁药: 增强中枢神经递质功能",
            "选择性5-羟色胺再摄取抑制剂: 氟西汀、帕罗西汀",
            "三环类抗抑郁药: 阿米替林、丙咪嗪",
            "心境稳定剂: 锂盐、丙戊酸钠治疗双相障碍",
            "抗焦虑药: 苯二氮䓬类减轻焦虑",
            "镇静催眠药: 诱导和维持睡眠",
            "药物副作用: 锥体外系反应、代谢紊乱等",
        ]
    }

    /// 心理治疗
    pub fn psychotherapy(&self) -> Vec<&'static str> {
        vec![
            "认知行为治疗: 改变不良认知和行为模式",
            "精神分析治疗: 探索潜意识冲突和早期经历",
            "支持性心理治疗: 提供情感支持和指导",
            "家庭治疗: 改善家庭关系和功能",
            "团体治疗: 在团体情境中进行心理治疗",
            "人际心理治疗: 改善人际关系问题",
            "行为治疗: 通过学习原理改变行为",
            "辩证行为治疗: 情绪调节和人际关系改善",
            "正念治疗: 提高对当下的关注和接纳",
            "艺术治疗: 通过艺术表达促进心理康复",
        ]
    }
}

impl Rule for PsychiatryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("psychiatry")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "精神病学定律",
            &[
                ("精神疾病诊断原则", &self.diagnostic_principles()),
                ("精神分裂症", &self.schizophrenia()),
                ("情感障碍", &self.mood_disorders()),
                ("神经症", &self.neurotic_disorders()),
                ("器质性精神障碍", &self.organic_disorders()),
                ("人格障碍", &self.personality_disorders()),
                ("精神发育迟滞", &self.intellectual_disability()),
                ("儿童青少年精神障碍", &self.child_psychiatric_disorders()),
                ("精神药物治疗", &self.psychopharmacology()),
                ("心理治疗", &self.psychotherapy()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_psychiatry_rules() {
        let rules = PsychiatryRules::new();
        assert!(!rules.diagnostic_principles().is_empty());
        assert!(!rules.schizophrenia().is_empty());
        assert!(!rules.mood_disorders().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_psychiatry_metadata() {
        let rules = PsychiatryRules::new();
        assert_eq!(rules.metadata().name, "精神病学定律");
    }
}
