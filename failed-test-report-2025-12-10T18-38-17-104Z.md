# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/10 18:38:17
- **数据来源**: connectivity_results-20251210-183817.json
- **总测试数**: 485
- **失败测试数**: 181
- **成功测试数**: 304
- **失败率**: 37.32%
- **平均延迟**: 147.19ms
- **最小延迟**: 60ms
- **最大延迟**: 677ms

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 172 次 (95.0%)
- **连接超时: I/O超时**: 9 次 (5.0%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (172 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (9 次测试)

| 序号 | 主机/域名        | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ---------------- | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 377  | 115.22.115.218   | 115.22.115.218  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 115.22.115.218:443: i/o timeout  |
| 409  | 52.76.110.129    | 52.76.110.129   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 52.76.110.129:443: i/o timeout   |
| 417  | 121.188.182.190  | 121.188.182.190 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout |
| 477  | cfip.xxxxxxxx.tk | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 481  | 111.171.108.67   | 111.171.108.67  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout  |
| 482  | 175.212.207.13   | 175.212.207.13  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout  |
| 483  | cf.877774.xyz    | cf.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout   |
| 484  | ct.877774.xyz    | ct.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout   |
| 485  | 119.194.220.146  | 119.194.220.146 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 172 次 (95.0%)
- **连接超时**: 9 次 (5.0%)

#### 错误模式分析

**超时集中度分析**: 共有 9 次超时，主要集中在IP段 115.22（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 181 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 9 次，IPv6失败 172 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：456.cloudflare.182682.xyz (3次),
ashton.ns.cloudflare.com (3次), dnschecker.org
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 176 |
comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare | | 170 |
www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare | |
147 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare | | 35
| 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare | |
296 | fbi.gov | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare | | 297 |
fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare | | 344 |
craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare
| | 319 | 172.64.146.16 | 172.64.146.16 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare
| | 425 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 76 |
cloudflare | | 454 | icook.tw | 104.20.28.74 | IPv4 | h3 | ✅ 成功 | 78 |
cloudflare | | 460 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h3 | ✅
成功 | 78 | cloudflare | | 474 | www.4444.cloudflare.182682.xyz | 162.159.152.2 | IPv4 | h3 | ✅
成功 | 82 | cloudflare | | 478 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h3 |
✅ 成功 | 82 | cloudflare | | 156 | www.hugedomains.com | 104.26.7.37 | IPv4 |
h3 | ✅ 成功 | 83 | cloudflare | | 446 | yx-auto.pages.dev | 104.21.9.230 | IPv4
| h3 | ✅ 成功 | 83 | cloudflare | | 137 | ifconfig.co | 104.21.54.91 | IPv4 |
h3 | ✅ 成功 | 86 | cloudflare | | 442 | trevor.ns.cloudflare.com |
172.64.35.154 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare | | 334 | 172.67.75.172 |
172.67.75.172 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare | | 244 | www.csgo.com |
195.85.59.161 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare | | 77 | cf.090227.xyz |
104.18.43.174 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare | | 361 |
uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h3 | ✅ 成功 | 95 |
cloudflare | | 22 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 97 |
cloudflare | | 359 | eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 97 |
cloudflare | | 116 | iplocation.io | 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 100 |
cloudflare | | 278 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 |
✅ 成功 | 101 | cloudflare | | 310 | 104.16.223.179 | 104.16.223.179 | IPv4 | h3
| ✅ 成功 | 101 | cloudflare | | 367 | www.glassdoor.com | 104.16.25.46 | IPv4 |
h3 | ✅ 成功 | 101 | cloudflare | | 135 | 162.159.36.104 | 162.159.36.104 | IPv4
| h3 | ✅ 成功 | 102 | cloudflare | | 248 | ip.sb | 172.67.75.172 | IPv4 | h3 |
✅ 成功 | 102 | cloudflare | | 39 | 172.64.148.15 | 172.64.148.15 | IPv4 | h3 |
✅ 成功 | 103 | cloudflare | | 256 | cu.877774.xyz | 104.26.4.115 | IPv4 | h3 |
✅ 成功 | 103 | cloudflare | | 14 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3
| ✅ 成功 | 104 | cloudflare | | 226 | cmcc.877774.xyz | 104.16.148.8 | IPv4 |
h3 | ✅ 成功 | 104 | cloudflare | | 94 | ct.877774.xyz | 172.64.229.44 | IPv4 |
h3 | ✅ 成功 | 105 | cloudflare | | 213 | 4444.cloudflare.182682.xyz | 172.67.152.183 | IPv4
| h3 | ✅ 成功 | 105 | cloudflare | | 433 | www.okcupid.com | 104.16.144.63 |
IPv4 | h3 | ✅ 成功 | 105 | cloudflare | | 85 | stock.hostmonit.com |
104.21.7.193 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare | | 289 | time.is |
104.26.13.54 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare | | 114 | iplocation.io |
104.26.10.222 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare | | 203 | 104.16.45.84 |
104.16.45.84 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare | | 294 | 172.64.49.165 |
172.64.49.165 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare | | 306 |
sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h3 | ✅ 成功 | 107 |
cloudflare | | 351 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅
成功 | 107 | cloudflare | | 356 | 172.64.159.6 | 172.64.159.6 | IPv4 | h3 | ✅
成功 | 107 | cloudflare | | 5 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅
成功 | 108 | cloudflare | | 61 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 |
✅ 成功 | 108 | cloudflare | | 224 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3
| ✅ 成功 | 109 | cloudflare | | 399 | icook.hk | 172.67.161.104 | IPv4 | h3 |
✅ 成功 | 109 | cloudflare | | 38 | 172.64.38.15 | 172.64.38.15 | IPv4 | h3 | ✅
成功 | 110 | cloudflare | | 169 | www.whatismyip.com | 104.26.12.23 | IPv4 | h3
| ✅ 成功 | 110 | cloudflare | | 229 | cmcc.877774.xyz | 104.16.148.11 | IPv4 |
h3 | ✅ 成功 | 110 | cloudflare | | 300 | 172.64.153.172 | 172.64.153.172 | IPv4
| h3 | ✅ 成功 | 110 | cloudflare | | 285 | 108.162.198.54 | 108.162.198.54 |
IPv4 | h3 | ✅ 成功 | 111 | cloudflare | | 394 | rustam.ns.cloudflare.com |
162.159.44.148 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare | | 37 |
cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare | |
214 | 4444.cloudflare.182682.xyz | 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare | |
204 | 104.16.61.163 | 104.16.61.163 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare | |
326 | 172.67.79.211 | 172.67.79.211 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare | |
58 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 195 | 172.64.156.195 | 172.64.156.195 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare
| | 287 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 419 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare | | 47
| 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare | | 268 |
dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 115 |
cloudflare | | 315 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅
成功 | 115 | cloudflare | | 349 | decker.ns.cloudflare.com | 108.162.195.155 |
IPv4 | h3 | ✅ 成功 | 115 | cloudflare | | 441 | trevor.ns.cloudflare.com |
162.159.44.154 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare | | 476 |
cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare | |
162 | yx-auto.pages.dev | 172.67.134.139 | IPv4 | h3 | ✅ 成功 | 116 |
cloudflare | | 86 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h3 | ✅ 成功 |
117 | cloudflare | | 241 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功
| 117 | cloudflare | | 245 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 |
117 | cloudflare | | 321 | www.gov.ua | 172.67.209.127 | IPv4 | h3 | ✅ 成功 |
117 | cloudflare | | 328 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功 |
117 | cloudflare | | 391 | 172.64.147.73 | 172.64.147.73 | IPv4 | h3 | ✅ 成功 |
117 | cloudflare | | 412 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h3
| ✅ 成功 | 117 | cloudflare | | 12 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3
| ✅ 成功 | 118 | cloudflare | | 15 | www.ipchicken.com | 172.67.68.101 | IPv4 |
h3 | ✅ 成功 | 118 | cloudflare | | 59 | asia.877774.xyz | 104.17.142.146 | IPv4
| h3 | ✅ 成功 | 118 | cloudflare | | 102 | 198.62.62.4 | 198.62.62.4 | IPv4 |
h3 | ✅ 成功 | 118 | cloudflare | | 121 | ae8a9c24-83de.masx200.ddns-ip.net |
104.21.14.41 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare | | 247 | ip.sb |
104.26.13.31 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare | | 258 | cu.877774.xyz |
104.26.4.117 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare | | 355 | 172.64.33.67 |
172.64.33.67 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare | | 369 |
moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h3 | ✅ 成功 | 118 |
cloudflare | | 473 | www.4444.cloudflare.182682.xyz | 162.159.153.2 | IPv4 | h3 | ✅ 成功 | 118 |
cloudflare | | 130 | na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅ 成功 | 119 |
cloudflare | | 185 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 |
119 | cloudflare | | 196 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3
| ✅ 成功 | 119 | cloudflare | | 304 | sullivan.ns.cloudflare.com |
108.162.195.161 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare | | 413 |
wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 119 |
cloudflare | | 40 | ip.gs | 104.21.14.176 | IPv4 | h3 | ✅ 成功 | 120 |
cloudflare | | 239 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 120
| cloudflare | | 242 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 |
120 | cloudflare | | 48 | 172.64.41.88 | 172.64.41.88 | IPv4 | h3 | ✅ 成功 |
121 | cloudflare | | 128 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 |
121 | cloudflare | | 254 | cu.877774.xyz | 104.26.4.113 | IPv4 | h3 | ✅ 成功 |
121 | cloudflare | | 18 | 456.cloudflare.182682.xyz | 172.67.75.208 | IPv4 | h3
| ✅ 成功 | 122 | cloudflare | | 63 | www.udemy.com | 104.16.143.237 | IPv4 | h3
| ✅ 成功 | 122 | cloudflare | | 104 | cf.zhetengsha.eu.org | 172.64.144.82 |
IPv4 | h3 | ✅ 成功 | 122 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 23 条记录
- **正常 (100-200ms)**: 77 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 9 次
- **IPv6 失败**: 172 次

### 按协议统计

- **none**: 181 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
