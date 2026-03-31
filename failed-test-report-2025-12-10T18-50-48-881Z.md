# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/10 18:50:48
- **数据来源**: connectivity_results-20251210-185048.json
- **总测试数**: 482
- **失败测试数**: 8
- **成功测试数**: 474
- **失败率**: 1.66%
- **平均延迟**: 168.39ms
- **最小延迟**: 132ms
- **最大延迟**: 1221ms

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 8 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (8 次测试)

| 序号 | 主机/域名                | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ------------------------ | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 7    | 111.171.108.67           | 111.171.108.67  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout  |
| 67   | 121.188.182.190          | 121.188.182.190 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout |
| 174  | 175.212.207.13           | 175.212.207.13  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout  |
| 340  | rustam.ns.cloudflare.com | 172.64.35.148   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.35.148:443: i/o timeout   |
| 344  | ct.877774.xyz            | ct.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout   |
| 345  | 119.194.220.146          | 119.194.220.146 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout |
| 374  | pranab.ns.cloudflare.com | 172.64.35.199   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.35.199:443: i/o timeout   |
| 454  | cfip.xxxxxxxx.tk         | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 8 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 8 次超时，主要集中在IP段 172.64（3
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 8 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 231 |
ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 132 | cloudflare | | 161 |
cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 134 |
cloudflare | | 78 | asia.877774.xyz | 104.17.139.62 | IPv4 | h2 | ✅ 成功 | 135
| cloudflare | | 154 | ifconfig.co | 2606:4700:3030::ac43:a86a | IPv6 | h2 | ✅
成功 | 135 | cloudflare | | 230 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅
成功 | 135 | cloudflare | | 19 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅
成功 | 136 | cloudflare | | 73 | cloudflare-ip.mofashi.ltd |
2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 136 | cloudflare | | 241 |
www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | h2 | ✅ 成功 | 136 | cloudflare
| | 18 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 137 | cloudflare |
| 51 | dnschecker.org | 172.67.73.216 | IPv4 | h2 | ✅ 成功 | 137 | cloudflare |
| 131 | ae8a9c24-83de.masx200.ddns-ip.net | 2606:4700:3030::6815:e29 | IPv6 | h2
| ✅ 成功 | 137 | cloudflare | | 229 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h2 |
✅ 成功 | 137 | cloudflare | | 262 | singapore.com | 104.26.13.140 | IPv4 | h2 |
✅ 成功 | 137 | cloudflare | | 355 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h2
| ✅ 成功 | 137 | cloudflare | | 10 | www.ipchicken.com | 104.26.7.112 | IPv4 |
h2 | ✅ 成功 | 138 | cloudflare | | 12 | www.ipchicken.com | 104.26.6.112 | IPv4
| h2 | ✅ 成功 | 138 | cloudflare | | 85 | tasteatlas.com | 2606:4700::6811:2569
| IPv6 | h2 | ✅ 成功 | 138 | cloudflare | | 219 | time.is | 172.67.68.157 |
IPv4 | h2 | ✅ 成功 | 138 | cloudflare | | 263 | singapore.com | 172.67.75.194 |
IPv4 | h2 | ✅ 成功 | 138 | cloudflare | | 356 | cmcc.877774.xyz |
104.16.149.244 | IPv4 | h2 | ✅ 成功 | 138 | cloudflare | | 24 | www.pcmag.com |
104.16.20.118 | IPv4 | h2 | ✅ 成功 | 139 | cloudflare | | 62 | ip.gs |
2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 139 | cloudflare | | 86 |
tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅ 成功 | 139 | cloudflare |
| 93 | saas.sin.fan | 162.159.36.20 | IPv4 | h2 | ✅ 成功 | 139 | cloudflare | |
102 | cf.zhetengsha.eu.org | 2606:4700:4407::ac40:9052 | IPv6 | h2 | ✅ 成功 |
139 | cloudflare | | 120 | na.877774.xyz | 104.18.38.235 | IPv4 | h2 | ✅ 成功 |
139 | cloudflare | | 209 | cu.877774.xyz | 104.26.4.119 | IPv4 | h2 | ✅ 成功 |
139 | cloudflare | | 222 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅
成功 | 139 | cloudflare | | 225 | fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功
| 139 | cloudflare | | 228 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅
成功 | 139 | cloudflare | | 29 | cf.877774.xyz | 172.64.146.66 | IPv4 | h2 | ✅
成功 | 140 | cloudflare | | 35 | 456.cloudflare.182682.xyz | 104.26.8.160 | IPv4
| h2 | ✅ 成功 | 140 | cloudflare | | 59 | ip.gs | 104.21.14.176 | IPv4 | h2 |
✅ 成功 | 140 | cloudflare | | 81 | freeyx.cloudflare88.eu.org |
2606:4700:3009:aa59:4b79:282c:480b:3b5f | IPv6 | h2 | ✅ 成功 | 140 | cloudflare
| | 112 | cf.0sm.com | 172.67.187.145 | IPv4 | h2 | ✅ 成功 | 140 | cloudflare |
| 236 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 140 | cloudflare |
| 240 | www.gov.ua | 172.67.209.127 | IPv4 | h2 | ✅ 成功 | 140 | cloudflare | |
267 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 140 | cloudflare | |
11 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 | ✅ 成功 | 141 | cloudflare
| | 26 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 141 |
cloudflare | | 84 | tasteatlas.com | 104.17.36.105 | IPv4 | h2 | ✅ 成功 | 141 |
cloudflare | | 95 | zread.ai | 172.67.202.78 | IPv4 | h2 | ✅ 成功 | 141 |
cloudflare | | 130 | ae8a9c24-83de.masx200.ddns-ip.net |
2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 141 | cloudflare | | 220 |
time.is | 104.26.13.54 | IPv4 | h2 | ✅ 成功 | 141 | cloudflare | | 224 |
[2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | h2 | ✅ 成功 |
141 | cloudflare | | 234 | 172.67.79.211 | 172.67.79.211 | IPv4 | h2 | ✅ 成功 |
141 | cloudflare | | 357 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h2 | ✅ 成功
| 141 | cloudflare | | 13 | [2606:4700:4408::18c5:3304] |
2606:4700:4408::18c5:3304 | IPv6 | h2 | ✅ 成功 | 142 | cloudflare | | 23 |
172.64.151.55 | 172.64.151.55 | IPv4 | h2 | ✅ 成功 | 142 | cloudflare | | 25 |
www.pcmag.com | 104.16.21.118 | IPv4 | h2 | ✅ 成功 | 142 | cloudflare | | 30 |
cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 142 |
cloudflare | | 32 | 172.67.181.209 | 172.67.181.209 | IPv4 | h2 | ✅ 成功 | 142
| cloudflare | | 61 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | h2 | ✅ 成功 |
142 | cloudflare | | 66 | cf.090227.xyz | 2a06:98c1:3108::6812:2a62 | IPv6 | h2
| ✅ 成功 | 142 | cloudflare | | 200 | 172.64.153.172 | 172.64.153.172 | IPv4 |
h2 | ✅ 成功 | 142 | cloudflare | | 261 | singapore.com | 104.26.12.140 | IPv4 |
h2 | ✅ 成功 | 142 | cloudflare | | 352 | cmcc.877774.xyz | 104.16.149.9 | IPv4
| h2 | ✅ 成功 | 142 | cloudflare | | 15 | bestcf.030101.xyz | 104.17.179.12 |
IPv4 | h2 | ✅ 成功 | 143 | cloudflare | | 17 | bestcf.030101.xyz |
2606:4700:0:cd:9b2d:c8ba:6717:5f84 | IPv6 | h2 | ✅ 成功 | 143 | cloudflare | |
20 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 143 |
cloudflare | | 34 | 456.cloudflare.182682.xyz | 172.67.75.208 | IPv4 | h2 | ✅
成功 | 143 | cloudflare | | 37 | 456.cloudflare.182682.xyz |
2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅ 成功 | 143 | cloudflare | | 69 |
www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 143 | cloudflare | | 90
| www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 143 | cloudflare
| | 179 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | h2 | ✅ 成功 |
143 | cloudflare | | 211 | cu.877774.xyz | 104.26.4.112 | IPv4 | h2 | ✅ 成功 |
143 | cloudflare | | 226 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 143
| cloudflare | | 9 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 |
IPv6 | h2 | ✅ 成功 | 144 | cloudflare | | 55 | dnschecker.org |
2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 144 | cloudflare | | 68 |
www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 144 | cloudflare | | 91
| www.udemy.com | 2606:4700::6810:8eed | IPv6 | h2 | ✅ 成功 | 144 | cloudflare
| | 99 | cf.zhetengsha.eu.org | 104.18.43.174 | IPv4 | h2 | ✅ 成功 | 144 |
cloudflare | | 147 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | h2 | ✅
成功 | 144 | cloudflare | | 148 | iplocation.io | 2606:4700:20::681a:ade | IPv6
| h2 | ✅ 成功 | 144 | cloudflare | | 205 | cu.877774.xyz | 104.26.4.115 | IPv4
| h2 | ✅ 成功 | 144 | cloudflare | | 207 | cu.877774.xyz | 104.26.4.117 | IPv4
| h2 | ✅ 成功 | 144 | cloudflare | | 353 | cmcc.877774.xyz | 104.16.149.10 |
IPv4 | h2 | ✅ 成功 | 144 | cloudflare | | 21 | www.wto.org |
2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 145 | cloudflare | | 28 |
cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 145 | cloudflare | | 104 |
ipinfo.in | 104.21.21.129 | IPv4 | h2 | ✅ 成功 | 145 | cloudflare | | 105 |
ipinfo.in | 172.67.198.203 | IPv4 | h2 | ✅ 成功 | 145 | cloudflare | | 111 |
stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2 | ✅ 成功 | 145 |
cloudflare | | 115 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅
成功 | 145 | cloudflare | | 127 | www.4chan.org | 104.16.228.229 | IPv4 | h2 |
✅ 成功 | 145 | cloudflare | | 138 | ct.877774.xyz | 172.64.229.174 | IPv4 | h2
| ✅ 成功 | 145 | cloudflare | | 157 | 103.160.204.59 | 103.160.204.59 | IPv4 |
h2 | ✅ 成功 | 145 | cloudflare | | 170 | lewis.ns.cloudflare.com |
2803:f800:50::6ca2:c39f | IPv6 | h2 | ✅ 成功 | 145 | cloudflare | | 171 |
lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | h2 | ✅ 成功 | 145 |
cloudflare | | 178 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 |
✅ 成功 | 145 | cloudflare | | 180 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功
| 145 | cloudflare | | 182 | ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 145 |
cloudflare | | 202 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 145
| cloudflare | | 233 | 104.18.39.196 | 104.18.39.196 | IPv4 | h2 | ✅ 成功 | 145
| cloudflare | | 54 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅
成功 | 146 | cloudflare | | 92 | saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅
成功 | 146 | cloudflare | | 125 | steamdb.info | 2606:4700:10::ac42:affa | IPv6
| h2 | ✅ 成功 | 146 | cloudflare | | 136 | ct.877774.xyz | 172.64.229.161 |
IPv4 | h2 | ✅ 成功 | 146 | cloudflare | | 137 | ct.877774.xyz | 172.64.229.173
| IPv4 | h2 | ✅ 成功 | 146 | cloudflare | | 145 | iplocation.io | 104.26.10.222
| IPv4 | h2 | ✅ 成功 | 146 | cloudflare | | 149 | iplocation.io |
2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 146 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 0 条记录
- **正常 (100-200ms)**: 100 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 8 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 8 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
