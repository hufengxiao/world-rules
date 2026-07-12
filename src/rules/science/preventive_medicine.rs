//! 预防医学定律 - 研究疾病预防和健康促进
//!
//! 预防医学强调疾病预防、健康促进和公共卫生。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: PreventiveMedicineRules,
    name: "预防医学定律",
    desc: "疾病预防和健康促进的基本原则",
    origin: "医学",
    tags: ["科学", "医学", "预防医学"]
}

impl PreventiveMedicineRules {
    /// 三级预防
    pub fn three_level_prevention(&self) -> Vec<&'static str> {
        vec![
            "一级预防: 防止疾病发生，健康教育和生活方式干预",
            "二级预防: 早发现、早诊断、早治疗",
            "三级预防: 防止病情恶化，促进康复",
            "病因预防: 消除致病因素",
            "临床前期预防: 疾病早期筛查",
            "临床预防: 防止并发症和残疾",
            "健康促进: 增强健康意识和行为",
            "预防接种: 疫苗接种预防传染病",
            "定期体检: 发现早期疾病",
            "健康教育: 提高健康素养",
        ]
    }

    /// 流行病学方法
    pub fn epidemiology_methods(&self) -> Vec<&'static str> {
        vec![
            "发病率: 新发病例与人口数的比例",
            "患病率: 现患病例与人口数的比例",
            "死亡率: 死亡人数与人口数的比例",
            "病死率: 死亡人数与患病人数的比例",
            "相对危险度: 暴露组与非暴露组发病率比值",
            "归因危险度: 暴露组发病率减去非暴露组发病率",
            "队列研究: 前瞻性随访暴露组和对照组",
            "病例对照研究: 回顾性比较病例组与对照组",
            "横断面调查: 某时点人群健康状况调查",
            "随机对照试验: 评价干预效果的金标准",
        ]
    }

    /// 传染病预防
    pub fn infectious_disease_prevention(&self) -> Vec<&'static str> {
        vec![
            "疫苗接种: 预麻疹、乙肝、脊灰等",
            "传染病监测: 发现和控制疫情",
            "隔离措施: 控制传染源",
            "消毒灭菌: 切断传播途径",
            "个人防护: 保护易感人群",
            "疫情报告: 及时报告法定传染病",
            "流行病学调查: 查明传染来源和传播途径",
            "预防接种证: 儿童入学查验接种证",
            "旅行检疫: 防止传染病跨境传播",
            "计划免疫: 按程序完成疫苗接种",
        ]
    }

    /// 非传染性疾病预防
    pub fn noncommunicable_disease_prevention(&self) -> Vec<&'static str> {
        vec![
            "心血管疾病预防: 控制血压、血脂、血糖",
            "癌症预防: 戒烟、限酒、合理饮食",
            "糖尿病预防: 健康生活方式、定期筛查",
            "慢性呼吸系统疾病预防: 戒烟、避免空气污染",
            "高血压预防: 低盐饮食、控制体重",
            "脑卒中预防: 控制高血压、糖尿病",
            "骨质疏松预防: 补钙、运动、晒太阳",
            "精神疾病预防: 心理健康、压力管理",
            "意外伤害预防: 安全教育、环境改善",
            "视力保护: 预防近视、定期检查",
        ]
    }

    /// 职业病预防
    pub fn occupational_disease_prevention(&self) -> Vec<&'static str> {
        vec![
            "职业病定义: 职业活动中接触有害因素引起的疾病",
            "职业性尘肺: 长期吸入粉尘引起的肺部疾病",
            "职业性中毒: 化学物质引起的中毒",
            "职业性皮肤病: 接触有害物质引起的皮肤病",
            "职业性噪声聋: 期接触噪声引起的听力损失",
            "职业防护: 个人防护用品使用",
            "职业健康监护: 定期体检和健康监测",
            "工作环境改善: 控制有害因素",
            "职业健康教育: 提高防护意识",
            "职业病诊断: 依据国家职业病诊断标准",
        ]
    }

    /// 食品卫生与营养
    pub fn food_hygiene_nutrition(&self) -> Vec<&'static str> {
        vec![
            "食品安全: 无毒无害、符合营养要求",
            "食物中毒预防: 加强食品卫生管理",
            "营养平衡: 合理搭配各类食物",
            "膳食指南: 指导健康饮食行为",
            "食品安全标准: 国家食品安全标准",
            "食品添加剂: 合理使用食品添加剂",
            "食品保质期: 保证食品安全",
            "食品检测: 监测食品安全指标",
            "健康饮食: 少盐少油、多吃蔬果",
            "营养宣教: 提高营养健康意识",
        ]
    }

    /// 环境卫生
    pub fn environmental_health(&self) -> Vec<&'static str> {
        vec![
            "饮用水安全: 符合饮用水卫生标准",
            "空气污染控制: 减少大气污染物排放",
            "土壤污染防治: 防止土壤污染",
            "噪音控制: 控制环境噪音",
            "垃圾处理: 生活垃圾无害化处理",
            "环境监测: 监测环境污染物",
            "环境健康评估: 评估环境对健康的影响",
            "职业环境卫生: 改善工作场所环境",
            "居住环境卫生: 保证居住环境安全",
            "环境保护法规: 执行环境保护法律",
        ]
    }

    /// 学校卫生
    pub fn school_health(&self) -> Vec<&'static str> {
        vec![
            "学生体检: 定期体检发现问题",
            "近视预防: 正确用眼、户外活动",
            "龋齿预防: 口腔卫生、定期检查",
            "营养午餐: 保证学生营养需求",
            "体育锻炼: 增强体质、预防疾病",
            "心理健康教育: 促进心理健康",
            "安全教育: 防止意外伤害",
            "传染病防控: 学校传染病监测",
            "教室卫生: 保证教室环境安全",
            "作息规律: 保证睡眠时间",
        ]
    }

    /// 妇幼卫生
    pub fn maternal_child_health(&self) -> Vec<&'static str> {
        vec![
            "孕前保健: 孕前准备和咨询",
            "孕期保健: 定期产检、营养指导",
            "产时保健: 安全分娩",
            "产后保健: 产后恢复指导",
            "新生儿保健: 新生儿护理",
            "儿童保健: 定期体检、生长发育监测",
            "母乳喂养: 促进母乳喂养",
            "计划生育: 生育指导",
            "妇女病普查: 定期妇科检查",
            "儿童预防接种: 按程序完成疫苗接种",
        ]
    }

    /// 老年卫生
    pub fn elderly_health(&self) -> Vec<&'static str> {
        vec![
            "老年体检: 定期体检发现疾病",
            "慢性病管理: 管理高血压、糖尿病等",
            "跌倒预防: 防止老年人跌倒",
            "骨质疏松防治: 补钙、运动",
            "认知功能维护: 预防老年痴呆",
            "心理健康: 关注老年人心理需求",
            "合理用药: 避免药物不良反应",
            "居家安全: 改善居家环境",
            "社会支持: 提供老年人社会支持",
            "康复护理: 促进老年人功能恢复",
        ]
    }
}

impl Rule for PreventiveMedicineRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("preventive_medicine")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "预防医学定律",
            &[
                ("三级预防", &self.three_level_prevention()),
                ("流行病学方法", &self.epidemiology_methods()),
                ("传染病预防", &self.infectious_disease_prevention()),
                (
                    "非传染性疾病预防",
                    &self.noncommunicable_disease_prevention(),
                ),
                ("职业病预防", &self.occupational_disease_prevention()),
                ("食品卫生与营养", &self.food_hygiene_nutrition()),
                ("环境卫生", &self.environmental_health()),
                ("学校卫生", &self.school_health()),
                ("妇幼卫生", &self.maternal_child_health()),
                ("老年卫生", &self.elderly_health()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preventive_medicine_rules() {
        let rules = PreventiveMedicineRules::new();
        assert!(!rules.three_level_prevention().is_empty());
        assert!(!rules.epidemiology_methods().is_empty());
        assert!(!rules.infectious_disease_prevention().is_empty());
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_preventive_medicine_metadata() {
        let rules = PreventiveMedicineRules::new();
        assert_eq!(rules.metadata().name, "预防医学定律");
    }
}
