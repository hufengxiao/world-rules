#!/usr/bin/env python3
"""批量充实规则内容 - 第五批"""
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
    # E13: 生命科学
    ("science", "molecular_biology", [
        ("中心法则", [
            "DNA复制:DNA->DNA 以DNA为模板合成新DNA",
            "转录:DNA->RNA 以DNA为模板合成mRNA",
            "翻译:RNA->蛋白质 mRNA在核糖体上翻译为蛋白质",
            "逆转录:RNA->DNA 逆转录酶催化(病毒)",
        ]),
        ("基因表达调控", [
            "转录水平:启动子/增强子/转录因子",
            "转录后水平:mRNA剪接/修饰/稳定性",
            "翻译水平:核糖体结合/miRNA调控",
            "表观遗传:DNA甲基化/组蛋白修饰",
        ]),
        ("基因工程", [
            "限制性内切酶:识别特定DNA序列并切割",
            "DNA连接酶:连接DNA片段",
            "PCR:聚合酶链式反应扩增DNA",
            "CRISPR-Cas9:基因编辑技术",
        ]),
    ]),
    ("science", "immunology_detailed", [
        ("先天免疫", [
            "物理屏障:皮肤/黏膜/纤毛",
            "化学屏障:胃酸/溶菌酶/抗菌肽",
            "细胞:巨噬细胞/中性粒细胞/NK细胞",
            "炎症反应:红肿热痛",
        ]),
        ("适应性免疫", [
            "T细胞:细胞免疫(杀伤性T细胞/辅助性T细胞)",
            "B细胞:体液免疫(产生抗体)",
            "抗体:IgG/IgM/IgA/IgE/IgD五类",
            "免疫记忆:疫苗原理",
        ]),
        ("免疫相关疾病", [
            "过敏:免疫系统对无害物质过度反应",
            "自身免疫病:免疫系统攻击自身组织",
            "免疫缺陷:免疫系统功能不足",
            "免疫疗法:利用免疫系统治疗疾病",
        ]),
    ]),
    ("science", "pharmacology_detailed", [
        ("药代动力学", [
            "吸收:药物从给药部位进入血液循环",
            "分布:药物从血液分布到各组织",
            "代谢:药物在肝脏被代谢(主要CYP450酶)",
            "排泄:药物从体内排出(主要肾脏)",
            "半衰期:药物浓度降低一半的时间",
        ]),
        ("药效动力学", [
            "受体理论:药物与受体结合产生效应",
            "激动剂:激活受体产生效应",
            "拮抗剂:阻断受体不产生效应",
            "量效关系:剂量与效应的关系",
            "治疗窗口:有效剂量与中毒剂量之间的范围",
        ]),
        ("药物相互作用", [
            "药酶诱导:加速其他药物代谢",
            "药酶抑制:减慢其他药物代谢",
            "协同作用:两药合用效应增强",
            "拮抗作用:两药合用效应减弱",
        ]),
    ]),
    # E17: 社会法
    ("law", "children_rights", [
        ("基本权利", [
            "生存权:有权获得基本生活保障",
            "发展权:有权获得教育和发展机会",
            "受保护权:有权免受暴力和剥削",
            "参与权:有权表达意见和参与决策",
        ]),
        ("保护措施", [
            "家庭保护:父母有抚养教育义务",
            "学校保护:学校有安全教育义务",
            "社会保护:禁止使用童工",
            "网络保护:限制未成年人网络游戏时间",
            "司法保护:少年法庭/教育为主惩罚为辅",
        ]),
        ("国际公约", [
            "联合国儿童权利公约:最全面的儿童权利国际文件",
            "中国1992年加入该公约",
            "核心原则:儿童最佳利益原则",
            "禁止歧视:不因种族性别等受歧视",
        ]),
    ]),
    ("law", "privacy_rights", [
        ("隐私权内容", [
            "私人生活安宁:不受他人侵扰",
            "私人信息保密:个人数据受保护",
            "私人空间:住宅不受非法侵入",
            "通信自由:通信内容不受非法查看",
        ]),
        ("数据保护", [
            "GDPR:欧盟通用数据保护条例",
            "CCPA:加州消费者隐私法",
            "中国个人信息保护法:2021年实施",
            "核心原则:合法/正当/必要/诚信",
        ]),
        ("限制与例外", [
            "公共利益:国家安全/公共卫生",
            "知情同意:数据处理需获得同意",
            "匿名化:去除个人标识信息",
            "数据泄露通知:发生泄露需及时通知",
        ]),
    ]),
    # E7: 冬季运动
    ("sports", "f1_fia_detailed", [
        ("积分系统", [
            "正赛积分:第1名25分/第2名18分/第3名15分",
            "第4-10名:12/10/8/6/4/2/1分",
            "最快圈速:额外1分(需进入前10)",
            "冲刺赛积分:8/7/6/5/4/3/2/1",
        ]),
        ("技术规则", [
            "动力单元:1.6升V6涡轮增压+能量回收",
            "最高转速:15000rpm",
            "轮胎供应商:Pirelli(5种配方)",
            "燃油限制:110kg/比赛",
            "DRS:可调尾翼系统(减少空气阻力)",
        ]),
        ("比赛规则", [
            "排位赛:Q1/Q2/Q3三节淘汰制",
            "正赛:最短305公里或2小时",
            "安全车:事故时安全车带领",
            "红旗:严重事故时比赛暂停",
            "进站策略:至少使用两种不同配方轮胎",
        ]),
    ]),
    ("sports", "speed_skating_isu", [
        ("比赛规则", [
            "标准跑道:400米椭圆形冰道",
            "两人一组比赛分别在内外道",
            "每圈交换内外道确保公平",
            "以时间排名不是以对手为参照",
        ]),
        ("项目", [
            "500米:爆发力项目",
            "1000米:速度耐力项目",
            "1500米:中距离项目",
            "5000米/10000米:长距离项目",
            "团体追逐赛:3人一队",
            "集体出发:多人同时比赛",
        ]),
        ("装备", [
            "克莱普冰鞋:冰刀可活动提高蹬冰效率",
            "连体服:减少空气阻力",
            "头盔:安全保护",
            "护目镜:防风防冰屑",
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
