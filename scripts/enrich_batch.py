#!/usr/bin/env python3
"""批量充实规则内容"""
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
    ("games", "shogi", [
        ("棋盘与棋子", ["9x9棋盘双方各20枚棋子", "棋子:王将/玉将飞车角行金将银将桂马香车步兵", "棋子初始位置固定排列"]),
        ("走法", ["王将周围8格", "飞车横竖任意距离", "角行斜向任意距离", "金将周围6格", "银将前方和斜前方5格", "桂马前方两格+左右一格", "香车前方任意距离", "步兵前方一格"]),
        ("特殊规则", ["打入:吃掉的棋子可放回棋盘作为己方棋子", "升级:棋子进入敌方最后三排可升级(翻面)", "禁止打入步兵直接将死", "千日手:同一局面重复4次判和"]),
    ]),
    ("games", "chess_detailed", [
        ("棋子走法", ["王(King)周围8格每步一格", "后(Queen)横竖斜任意距离", "车(Rook)横竖任意距离", "象(Bishop)斜向任意距离", "马(Knight)L形2+1可跳过棋子", "兵(Pawn)前进一格首步可两格斜吃"]),
        ("特殊走法", ["王车易位:王向车方向移动两格车跳到王另一侧", "条件:王和车未移动过中间无棋子王未被将军", "吃过路兵:敌方兵首步走两格时可斜吃它", "兵升变:兵到达底线必须升级为后/车/象/马"]),
        ("胜负规则", ["将死(Checkmate):王被将军且无法逃脱", "逼和:无合法走法但未被将军", "三次重复局面和棋", "50步规则:50回合内无吃子无兵移动判和", "双方同意和棋", "时间耗尽判负"]),
    ]),
    ("games", "go_detailed", [
        ("基本规则", ["19x19棋盘也有9x9和13x13", "黑白双方轮流在交叉点落子", "棋子落下后不能移动除非被提", "气:棋子相邻的空交叉点", "无气的棋子被提走", "禁止自杀不能下无气的点除非能提对方"]),
        ("眼与活棋", ["眼:被己方棋子包围的空交叉点", "两个真眼的棋群是活棋不会被提", "假眼:可被对方破坏的眼"]),
        ("规则体系", ["中国规则:数子法活子+围空", "日本规则:数目法围空-提子", "贴目:黑方先行补偿白方中国7.5目日本6.5目", "终局:双方pass后计算领地", "劫争:禁止立即回提同一子"]),
    ]),
    ("sports", "football_league", [
        ("联赛结构", ["20支球队主客场双循环共38轮", "胜3分平1分负0分", "积分相同依次比较净胜球进球数相互战绩", "欧冠资格前4名直接进入小组赛", "降级最后3名降入英冠"]),
        ("VAR规则", ["视频助理裁判审查进球点球红牌认错人", "主裁判可查看场边监视器", "仅明显错误时介入"]),
        ("比赛规则", ["每场可换5人3次换人窗口", "加时赛淘汰赛平局后30分钟", "点球大战加时赛后仍平局时进行"]),
    ]),
    ("sports", "basketball_nba_detailed", [
        ("基本规则", ["5人对5人比赛4节各12分钟", "24秒进攻时限", "8秒过半场3秒区限制", "三分线NBA7.24米FIBA6.75米"]),
        ("选秀规则", ["NBA选秀两轮60个选秀权", "乐透抽签未进季后赛14支球队参与", "新秀合同首轮4年"]),
        ("工资帽", ["软工资帽限制球队薪资总额", "奢侈税超过奢侈税线需缴纳罚款", "伯德条款允许超工资帽续约自己的球员"]),
    ]),
    ("sports", "tennis_grand_slam", [
        ("四大满贯", ["澳网1月硬地", "法网5-6月红土", "温网6-7月草地", "美网8-9月硬地"]),
        ("比赛规则", ["男子单打五盘三胜制", "女子单打三盘两胜制", "每盘6局6-6时抢七", "发球每局轮换每分两次发球机会"]),
        ("特殊规则", ["鹰眼挑战每盘3次机会", "医疗暂停每盘一次3分钟", "温网要求全白着装"]),
    ]),
    ("social", "chinese_new_year", [
        ("时间与准备", ["农历正月初一除夕夜守岁", "腊月二十三小年祭灶", "贴春联福字窗花", "准备年货打扫房屋"]),
        ("传统习俗", ["年夜饭全家团聚菜品寓意吉祥", "守岁除夕夜不睡觉迎接新年", "拜年初一给长辈拜年说吉利话", "红包长辈给晚辈用新钞双数金额", "放鞭炮烟花驱邪迎新"]),
        ("禁忌", ["初一不扫地扫走财运", "不说不吉利的话", "红包不能当面拆开", "打碎碗要说碎碎平安"]),
    ]),
    ("social", "chinese_tea_ceremony", [
        ("茶具", ["茶壶紫砂壶最佳", "公道杯品茗杯", "盖碗万能茶具适合所有茶类"]),
        ("泡茶步骤", ["温壶温杯用热水冲洗茶具", "投茶3-5克/150ml", "洗茶第一泡倒掉不喝", "水温根据茶类调整绿茶80度红茶95度", "出汤控制浸泡时间"]),
        ("品茶礼仪", ["闻香先闻杯盖香再闻杯底香", "品饮小口慢品感受回甘", "扣指礼长辈倒茶时用手指轻扣桌面", "续茶主人应及时续茶"]),
    ]),
    ("social", "chinese_dining", [
        ("座次安排", ["主位面对门口主人或最尊贵的客人", "主宾主人右手边最重要的客人", "以右为尊以远为上"]),
        ("用餐礼仪", ["等主人或长辈先动筷", "不翻拣菜肴", "不把筷子插在饭上像祭祀", "喝汤不出声嘴中有食物不说话"]),
        ("敬酒礼仪", ["晚辈敬酒杯沿低于长辈杯沿", "敬酒时双手持杯", "先敬主宾再按顺序", "主人应先敬酒客人回敬"]),
    ]),
]

count = 0
for cat, name, sections in enrichments:
    ok = enrich_file(cat, name, sections)
    if ok:
        count += 1
        print(f"  Enriched: {cat}/{name}")
print(f"\nTotal enriched: {count}")
