#!/usr/bin/env python3
"""批量充实规则内容 - 第三批"""
import os, re

BASE = "D:/Projects/world-rules/src/rules"

def enrich_file(cat, name, sections):
    path = f"{BASE}/{cat}/{name}.rs"
    if not os.path.exists(path):
        return False
    with open(path, 'r', encoding='utf-8') as f:
        content = f.read()
    m = re.search(r'struct:\s+(\w+Rules)', content)
    if not m:
        return False
    struct_name = m.group(1)
    nm = re.search(r'name:\s+"([^"]+)"', content)
    dm = re.search(r'desc:\s+"([^"]+)"', content)
    om = re.search(r'origin:\s+"([^"]+)"', content)
    tm = re.search(r'tags:\s+\[([^\]]+)\]', content)
    if not all([nm, dm, om, tm]):
        return False
    display_name, desc, origin, tags = nm.group(1), dm.group(1), om.group(1), tm.group(1)
    meth = ""
    calls = ""
    for idx, (sn, items) in enumerate(sections):
        il = ", ".join('"' + i + '"' for i in items)
        meth += f"    pub fn section_{idx}(&self) -> Vec<&'static str> {{ vec![{il}] }}\n\n"
        calls += f'            ("{sn}", &self.section_{idx}()),\n'
    new_content = f'''//! {display_name}
use crate::rules::core::{{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext}};
use crate::simple_rule;
simple_rule! {{ struct: {struct_name}, name: "{display_name}", desc: "{desc}", origin: "{origin}", tags: [{tags}] }}
impl {struct_name} {{
{meth}}}
impl Rule for {struct_name} {{
    fn metadata(&self) -> &RuleMetadata {{ &self.metadata }}
    fn category(&self) -> RuleCategory {{ RuleCategory::{cat}("{name}") }}
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {{ Ok(true) }}
    fn explain(&self) -> String {{ crate::rules::core::format_rule_sections("{display_name}", &[{calls}]) }}
}}
#[cfg(test)]
mod tests {{ use super::*; #[test] fn test() {{ let r = {struct_name}::new(); assert!(!r.explain().is_empty()); }} }}
'''
    with open(path, 'w', encoding='utf-8') as f:
        f.write(new_content)
    return True

enrichments = [
    # E11: 物理
    ("science", "mechanics_classical", [
        ("牛顿三大定律", [
            "第一定律惯性定律:物体不受力时保持静止或匀速直线运动",
            "第二定律:F=ma 力等于质量乘以加速度",
            "第三定律:作用力与反作用力大小相等方向相反",
        ]),
        ("万有引力定律", [
            "F=GMm/r^2 两物体间引力与质量乘积成正比与距离平方成反比",
            "G=6.674x10^-11 N·m^2/kg^2 万有引力常数",
            "适用于质点或均匀球体",
        ]),
        ("功和能", [
            "功:W=Fs cos theta 力乘以位移乘以夹角余弦",
            "动能:Ek=1/2 mv^2",
            "势能:Ep=mgh 重力势能",
            "能量守恒定律:能量不能被创造或消灭只能转化",
        ]),
    ]),
    ("science", "quantum_mechanics_detailed", [
        ("基本原理", [
            "波粒二象性:微观粒子同时具有波动和粒子特性",
            "薛定谔方程:i*h_bar*dPsi/dt=H*Psi 描述量子态演化",
            "波函数Psi:描述粒子量子态的数学函数",
            "波函数模的平方给出粒子出现概率",
        ]),
        ("不确定性原理", [
            "海森堡不确定性原理:位置和动量不能同时精确测量",
            "Delta x * Delta p >= h_bar/2",
            "能量和时间的不确定性:Delta E * Delta t >= h_bar/2",
        ]),
        ("量子效应", [
            "量子隧穿:粒子可以穿越经典力学不允许的势垒",
            "量子纠缠:两个粒子状态关联测量一个立即影响另一个",
            "量子叠加:粒子可以同时处于多个状态",
            "观测导致波函数坍缩到确定状态",
        ]),
    ]),
    ("science", "thermodynamics_detailed", [
        ("四大定律", [
            "第零定律:若A与C热平衡B与C热平衡则A与B热平衡",
            "第一定律:内能变化=吸收热量-对外做功 dU=dQ-dW",
            "第二定律:热量不能自发从低温物体传到高温物体",
            "第三定律:绝对零度不可能达到",
        ]),
        ("熵", [
            "熵是系统无序程度的度量",
            "克劳修斯不等式:dQ/T <= 0 对于循环过程",
            "熵增原理:孤立系统的熵永不减少",
            "熵的统计解释:S=k_B * ln(W) 玻尔兹曼公式",
        ]),
        ("应用", [
            "卡诺热机:理想热机效率=1-T_cold/T_hot",
            "热力学第二定律的开尔文表述和克劳修斯表述等价",
            "自由能:Gibbs自由能G=H-TS判断反应方向",
        ]),
    ]),
    # E15: 中国法律
    ("law", "civil_code_general", [
        ("基本原则", [
            "平等原则:民事主体法律地位一律平等",
            "自愿原则:按照自己意愿设立变更终止民事法律关系",
            "公平原则:合理确定各方权利义务",
            "诚信原则:秉持诚实恪守承诺",
            "守法与公序良俗原则",
            "绿色原则:节约资源保护生态环境",
        ]),
        ("民事主体", [
            "自然人:从出生到死亡享有民事权利能力",
            "法人:营利法人/非营利法人/特别法人",
            "非法人组织:个人独资企业/合伙企业等",
            "民事行为能力:完全/限制/无民事行为能力",
            "8周岁以上18周岁以下为限制民事行为能力人",
        ]),
        ("民事法律行为", [
            "有效条件:行为人有相应行为能力/意思表示真实/不违反法律",
            "无效情形:违反强制性规定/违背公序良俗/恶意串通",
            "可撤销:重大误解/欺诈/胁迫/显失公平",
            "效力待定:限制行为能力人超出能力范围的行为",
        ]),
    ]),
    ("law", "criminal_law_general", [
        ("犯罪构成", [
            "犯罪客体:犯罪行为侵害的社会关系",
            "犯罪客观方面:危害行为/危害结果/因果关系",
            "犯罪主体:实施犯罪的人(自然人/单位)",
            "犯罪主观方面:故意或过失",
        ]),
        ("刑罚种类", [
            "主刑:管制(3月-2年)/拘役(1月-6月)/有期徒刑(6月-15年)/无期徒刑/死刑",
            "附加刑:罚金/剥夺政治权利/没收财产/驱逐出境",
            "附加刑可独立适用也可附加适用",
        ]),
        ("正当防卫", [
            "正当防卫:为保护合法权益制止不法侵害",
            "防卫过当:明显超过必要限度造成重大损害",
            "特殊防卫:对正在进行行凶杀人抢劫强奸绑架等暴力犯罪的防卫不存在防卫过当",
            "紧急避险:为保护合法权益不得已损害另一较小利益",
        ]),
    ]),
    # E18: 健康
    ("health", "heart_health", [
        ("风险因素", [
            "高血压:收缩压>=140或舒张压>=90mmHg",
            "高血脂:LDL-C>3.4mmol/L",
            "糖尿病:空腹血糖>=7.0mmol/L",
            "吸烟:心血管疾病的重要危险因素",
            "肥胖:BMI>=28",
            "家族史:直系亲属有心血管疾病",
        ]),
        ("预防措施", [
            "控制血压:目标<140/90mmHg",
            "控制血脂:LDL-C目标<2.6mmol/L(高危<1.8)",
            "控制血糖:HbA1c<7%",
            "戒烟:戒烟1年后心血管风险降低50%",
            "运动:每周150分钟中等强度有氧运动",
            "饮食:低盐(<6g/天)/低脂/多蔬果",
        ]),
        ("警示症状", [
            "胸痛:胸骨后压榨性疼痛持续>15分钟",
            "呼吸困难:活动后气短",
            "心悸:心跳不规则或过快",
            "晕厥:突然意识丧失",
            "发现症状立即拨打120急救电话",
        ]),
    ]),
    ("health", "diabetes_management", [
        ("诊断标准", [
            "空腹血糖>=7.0mmol/L",
            "餐后2小时血糖>=11.1mmol/L",
            "糖化血红蛋白HbA1c>=6.5%",
            "随机血糖>=11.1mmol/L伴典型症状",
        ]),
        ("饮食管理", [
            "控制总热量:根据体重和活动量计算",
            "碳水化合物占总热量45-60%",
            "选择低GI食物:全谷物/豆类/蔬菜",
            "定时定量:每天3餐规律进食",
            "限制含糖饮料和精制糖",
        ]),
        ("运动管理", [
            "每周至少150分钟中等强度有氧运动",
            "运动时间:餐后1小时最佳",
            "运动前后监测血糖",
            "避免空腹运动防止低血糖",
            "血糖>16.7mmol/L或有酮症时不宜运动",
        ]),
    ]),
    ("health", "first_aid", [
        ("心肺复苏CPR", [
            "确认安全环境",
            "判断意识:轻拍重唤",
            "拨打120急救电话",
            "胸外按压:双手交叠掌根按压胸骨下半段",
            "按压深度5-6cm频率100-120次/分钟",
            "30次按压后2次人工呼吸",
            "持续直到急救人员到达",
        ]),
        ("止血", [
            "直接压迫:用干净布料直接按压伤口",
            "抬高肢体:受伤肢体抬高于心脏",
            "止血带:四肢大出血时使用(每小时放松1次)",
            "填塞:深部伤口用干净布料填塞",
        ]),
        ("常见急救", [
            "烫伤:冲脱泡盖送(冷水冲洗15分钟以上)",
            "骨折:固定制动不要复位",
            "中毒:拨打120保留毒物样本",
            "溺水:先确保自身安全再施救",
            "触电:先断电源再施救",
        ]),
    ]),
]

count = 0
for cat, name, sections in enrichments:
    ok = enrich_file(cat, name, sections)
    if ok:
        count += 1
        print(f"  Enriched: {cat}/{name}")
print(f"\nTotal enriched: {count}")
