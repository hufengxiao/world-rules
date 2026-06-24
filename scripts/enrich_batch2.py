#!/usr/bin/env python3
"""批量充实规则内容 - 第二批"""
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
    # E3: 麻将变体
    ("games", "mahjong_korean", [
        ("基本规则", ["韩国麻将使用简化规则", "没有风牌和箭牌只有万条筒", "每人发13张牌", "不使用花牌"]),
        ("特殊规则", ["只有条和筒两种花色时可胡", "韩国麻将简化了番种计算", "注重速度和简洁"]),
    ]),
    ("games", "mahjong_taiwanese_detailed", [
        ("基本规则", ["台湾麻将使用16张手牌", "使用144张牌含花牌", "每人发16张牌比一般麻将多3张"]),
        ("计分规则", ["以台数计算番值", "花牌可加台", "门清自摸加台", "各种特殊牌型有不同台数"]),
        ("特殊规则", ["连庄规则:庄家胡牌可连庄", "花牌补牌规则", "台数门槛通常为8台或16台"]),
    ]),
    ("games", "mahjong_hongkong", [
        ("基本规则", ["香港麻将使用鸡胡规则", "鸡胡即可胡牌无最低番数要求", "使用144张牌含花牌"]),
        ("番种", ["平胡:基本胡牌", "碰碰胡:全部刻子", "清一色:全同花色", "混一色:一种花色加字牌", "小三元:两种箭牌刻子加一种箭牌对子"]),
        ("计分规则", ["鸡胡1番", "碰碰胡2番", "清一色4番", "混一色2番", "自摸加1番"]),
    ]),
    ("games", "mahjong_japanese_detailed", [
        ("基本规则", ["日本麻将立直麻将", "使用136张牌无花牌", "每人发13张牌", "宝牌指示牌决定宝牌"]),
        ("立直规则", ["立直:听牌时宣布立直需支付1000点", "一发:立直后一圈内胡牌加1翻", "里宝牌:立直胡牌时翻开里宝牌"]),
        ("役满", ["大四喜:四种风牌刻子", "绿一色:全由条子23468组成", "九莲宝灯:同花色1112345678999加任意一张", "四暗刻:四个暗刻", "国士无双:十三幺"]),
    ]),
    # E5: 格斗运动
    ("sports", "mma_ufc", [
        ("比赛规则", ["比赛3回合每回合5分钟", "冠军赛5回合每回合5分钟", "回合间休息1分钟"]),
        ("得分规则", ["10分制:赢方10分输方9分或更少", "有效打击:拳腿膝", "摔跤:成功摔倒对手", "控制:地面控制时间", "降服:绞技关节技"]),
        ("犯规", ["插眼击裆咬人", "击打后脑勺脊椎", "抓扯头发抓笼网", "12点到6点肘击(向下肘击)", "处罚:扣分或取消资格"]),
        ("体重级别", ["蝇量级125磅", "雏量级135磅", "羽量级145磅", "轻量级155磅", "次中量级170磅", "中量级185磅", "轻重量级205磅", "重量级265磅"]),
    ]),
    ("sports", "boxing_wbc", [
        ("比赛规则", ["职业拳击12回合每回合3分钟", "回合间休息1分钟", "使用10盎司拳套(次中量级以上)", "裁判可终止比赛(RTKO)"]),
        ("得分规则", ["10分制:赢方10分输方9分", "3名边裁打分取多数", "击倒:读秒10秒内无法继续判KO", "技术击倒:裁判医生或角终止比赛"]),
        ("级别", ["迷你轻量级105磅", "轻蝇量级108磅", "蝇量级112磅", "雏量级118磅", "羽量级126磅", "轻量级135磅", "次中量级147磅", "中量级160磅", "轻重量级175磅", "重量级200磅以上"]),
    ]),
    ("sports", "bjj_ibjjf_detailed", [
        ("带位制度", ["白带:初学者", "蓝带:2年训练", "紫带:4年训练", "棕带:6年训练", "黑带:8年以上"]),
        ("比赛规则", ["比赛时间根据带位:白蓝5分钟紫棕6分钟黑10分钟", "得分:扫技2分摔倒3分过腿3分骑乘4分拿背4分", "优势:近似得分动作"]),
        ("降服", ["绞技:裸绞三角绞领绞", "关节技:十字固肩锁膝十字固", "拍垫认输:被降服时拍对手或垫子"]),
    ]),
    ("sports", "wushu_iwuf", [
        ("套路比赛", ["长拳:快速灵活多跳跃旋转", "南拳:刚猛有力发声助力", "太极:缓慢柔和连绵不断", "刀剑枪棍:器械套路"]),
        ("散打比赛", ["拳腿摔三种技术", "得分:拳1分腿2分摔3分", "每局2分钟共3局", "禁止:击打后脑裆部咽喉"]),
        ("评分规则", ["套路:动作质量演练水平难度分", "散打:有效打击得分", "裁判组:3-5名裁判打分"]),
    ]),
]

count = 0
for cat, name, sections in enrichments:
    ok = enrich_file(cat, name, sections)
    if ok:
        count += 1
        print(f"  Enriched: {cat}/{name}")
print(f"\nTotal enriched: {count}")
