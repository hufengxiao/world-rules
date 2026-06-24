#!/usr/bin/env python3
"""批量充实规则内容 - 第四批"""
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
    # E12: 数学
    ("science", "calculus", [
        ("微分", [
            "导数定义:f'(x)=lim[h->0](f(x+h)-f(x))/h",
            "基本导数:(x^n)’=n*x^(n-1), (sin x)’=cos x, (e^x)’=e^x",
            "链式法则:(f(g(x)))’=f'(g(x))*g’(x)",
            "乘积法则:(fg)’=f’g+fg'",
            "商法则:(f/g)’=(f’g-fg’)/g^2",
        ]),
        ("积分", [
            "不定积分:F(x)=int f(x)dx 表示F’(x)=f(x)",
            "基本积分:int x^n dx=x^(n+1)/(n+1)+C",
            "定积分:int_a^b f(x)dx=F(b)-F(a) 牛顿-莱布尼茨公式",
            "分部积分:int u dv=uv-int v du",
            "换元积分:int f(g(x))g’(x)dx=int f(u)du",
        ]),
        ("应用", [
            "极值:导数为零的点可能是极大或极小值",
            "二阶导数:f”>0极小值f”<0极大值",
            "曲线下面积:定积分的几何意义",
            "体积:旋转体体积=pi*int f(x)^2 dx",
            "弧长:int sqrt(1+(f’(x))^2) dx",
        ]),
    ]),
    ("science", "algorithms", [
        ("排序算法", [
            "冒泡排序:O(n^2)相邻比较交换",
            "快速排序:O(n log n)平均分治法选基准",
            "归并排序:O(n log n)稳定分治法",
            "堆排序:O(n log n)原地不稳定",
        ]),
        ("图算法", [
            "BFS广度优先搜索:O(V+E)队列实现",
            "DFS深度优先搜索:O(V+E)栈/递归实现",
            "Dijkstra最短路径:O((V+E)logV)非负权重",
            "Bellman-Ford:O(VE)可处理负权重",
            "Floyd-Warshall:O(V^3)所有点对最短路径",
        ]),
        ("动态规划", [
            "最优子结构:问题的最优解包含子问题的最优解",
            "重叠子问题:子问题被重复计算",
            "状态转移方程:定义子问题之间的关系",
            "背包问题:0-1背包/完全背包/多重背包",
            "最长公共子序列LCS",
        ]),
    ]),
    ("science", "cryptography_detailed", [
        ("对称加密", [
            "AES:高级加密标准分组长度128位密钥128/192/256位",
            "DES:数据加密标准已不安全密钥56位",
            "分组模式:ECB/CBC/CTR/GCM",
            "流密码:RC4(已不安全)/ChaCha20",
        ]),
        ("非对称加密", [
            "RSA:基于大数分解困难性密钥2048位以上",
            "ECC:椭圆曲线密码密钥更短安全性更高",
            "Diffie-Hellman:密钥交换协议",
            "数字签名:RSA签名/ECDSA签名",
        ]),
        ("哈希函数", [
            "SHA-256:输出256位抗碰撞",
            "SHA-3:Keccak算法",
            "MD5:已不安全可被碰撞",
            "HMAC:基于哈希的消息认证码",
            "密码哈希:bcrypt/scrypt/Argon2",
        ]),
    ]),
    # E16: 国际法
    ("law", "wto_law", [
        ("基本原则", [
            "最惠国待遇:给予一国的优惠必须无条件给予所有WTO成员",
            "国民待遇:进口商品与国内商品同等待遇",
            "透明度原则:贸易政策法规必须公开",
            "自由贸易原则:通过谈判降低关税和贸易壁垒",
        ]),
        ("争端解决", [
            "磋商:争端双方首先尝试协商解决",
            "专家组:磋商失败后设立专家组审理",
            "上诉机构:对专家组报告可以上诉",
            "执行:败诉方必须执行裁决否则面临报复",
        ]),
        ("主要协定", [
            "GATT:关税与贸易总协定(货物贸易)",
            "GATS:服务贸易总协定",
            "TRIPS:与贸易有关的知识产权协定",
            "SPS:卫生与植物卫生措施协定",
            "TBT:技术性贸易壁垒协定",
        ]),
    ]),
    ("law", "eu_gdpr", [
        ("基本原则", [
            "合法性公平性透明性:数据处理必须有合法基础",
            "目的限制:数据只能用于收集时声明的目的",
            "数据最小化:只收集必要的数据",
            "准确性:数据必须准确且及时更新",
            "存储限制:数据保留不超过必要时间",
            "完整性和保密性:确保数据安全",
        ]),
        ("数据主体权利", [
            "知情权:有权知道数据如何被处理",
            "访问权:有权获取自己的数据副本",
            "更正权:有权要求更正不准确的数据",
            "删除权:有权要求删除数据",
            "限制处理权:有权限制数据处理",
            "数据可携带权:有权以通用格式获取数据",
            "反对权:有权反对数据处理",
        ]),
        ("处罚", [
            "严重违规:最高2000万欧元或全球营业额4%",
            "一般违规:最高1000万欧元或全球营业额2%",
            "数据泄露:必须在72小时内通知监管机构",
        ]),
    ]),
    # E6: 水上运动
    ("sports", "surfing_wsl", [
        ("比赛规则", [
            "比赛时间:20-30分钟(根据浪况)",
            "每位选手最多冲25道浪",
            "取最好的两道浪得分相加",
            "满分10分每道浪(总分20分)",
        ]),
        ("评分标准", [
            "承诺:浪的难度和选择",
            "创新:创新性动作",
            "组合:动作的组合和流畅性",
            "速度力量和流畅性",
            "浪的大小和质量影响基础分",
        ]),
        ("优先权", [
            "最内侧选手有优先权(最靠近浪的破碎点)",
            "阻挡对手冲浪会被扣分",
            "选手必须在浪的正面冲浪",
            "两人同时冲一道浪时优先权选手得分",
        ]),
    ]),
    ("sports", "climbing_ifsc_detailed", [
        ("速度赛", [
            "标准赛道:15米高45度倾斜",
            "两人同时攀登相同赛道",
            "最快到达顶部者胜",
            "世界纪录:男子约5秒女子约6秒",
        ]),
        ("难度赛", [
            "先锋赛:选手在规定时间内攀登尽可能高的位置",
            "先锋赛时间:男子6分钟女子6分钟",
            "选手不能事先查看赛道",
            "高度越高排名越前",
        ]),
        ("攀石赛", [
            "选手在规定时间内尝试多条路线",
            "每条路线有多个得分点(Zone和Top)",
            "Top到达路线顶部得分最高",
            "Zone到达中间得分点得分次之",
            "以完成路线数和尝试次数排名",
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
