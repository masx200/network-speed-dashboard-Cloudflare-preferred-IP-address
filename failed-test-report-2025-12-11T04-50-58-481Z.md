# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/11 04:50:58
- **数据来源**: connectivity_results-20251211-045058.json
- **总测试数**: 484
- **失败测试数**: 175
- **成功测试数**: 309
- **失败率**: 36.16%
- **平均延迟**: 155.32ms
- **最小延迟**: 53ms
- **最大延迟**: 1399ms

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 171 次 (97.7%)
- **连接超时: I/O超时**: 4 次 (2.3%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (171 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (4 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 470  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 471  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 480  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 481  | cfip.xxxxxxxx.tk | 104.20.255.53  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.20.255.53:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 171 次 (97.7%)
- **连接超时**: 4 次 (2.3%)

#### 错误模式分析

**超时集中度分析**: 共有 4 次超时，主要集中在IP段 172.67（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 175 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 4 次，IPv6失败 171 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：dnschecker.org (3次),
ashton.ns.cloudflare.com (3次), 456.cloudflare.182682.xyz
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 477 |
cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare | |
433 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 126 | cf.zhetengsha.eu.org | 104.18.35.15 | IPv4 | h3 | ✅ 成功 | 62 |
cloudflare | | 308 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 63
| cloudflare | | 212 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 |
IPv4 | h3 | ✅ 成功 | 68 | cloudflare | | 389 | www.visa.com.hk | 104.18.21.69 |
IPv4 | h3 | ✅ 成功 | 73 | cloudflare | | 282 | 172.64.146.16 | 172.64.146.16 |
IPv4 | h3 | ✅ 成功 | 74 | cloudflare | | 446 | yx-auto.pages.dev |
172.67.161.98 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare | | 106 | ct.877774.xyz |
172.64.229.174 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare | | 184 |
www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare | |
242 | cu.877774.xyz | 104.26.4.112 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare | |
345 | 104.17.79.11 | 104.17.79.11 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare | |
457 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 87 |
cloudflare | | 265 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h3 | ✅ 成功 |
88 | cloudflare | | 279 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 88 |
cloudflare | | 467 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 88 |
cloudflare | | 51 | gamer.com.tw | 104.18.2.197 | IPv4 | h3 | ✅ 成功 | 89 |
cloudflare | | 120 | iplocation.io | 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 90 |
cloudflare | | 166 | yx-auto.pages.dev | 172.67.134.139 | IPv4 | h3 | ✅ 成功 |
92 | cloudflare | | 314 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h3
| ✅ 成功 | 92 | cloudflare | | 89 | stock.hostmonit.com | 172.67.187.251 | IPv4
| h3 | ✅ 成功 | 93 | cloudflare | | 172 | www.hugedomains.com | 104.26.7.37 |
IPv4 | h3 | ✅ 成功 | 93 | cloudflare | | 283 | cmcc.877774.xyz | 104.16.148.1 |
IPv4 | h3 | ✅ 成功 | 93 | cloudflare | | 286 | cmcc.877774.xyz | 104.16.148.4 |
IPv4 | h3 | ✅ 成功 | 93 | cloudflare | | 56 | ip.gs | 172.67.160.28 | IPv4 | h3
| ✅ 成功 | 95 | cloudflare | | 261 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 |
✅ 成功 | 95 | cloudflare | | 300 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 |
✅ 成功 | 95 | cloudflare | | 309 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅
成功 | 95 | cloudflare | | 347 | craig.ns.cloudflare.com | 108.162.195.192 |
IPv4 | h3 | ✅ 成功 | 95 | cloudflare | | 363 | moura.ns.cloudflare.com |
108.162.195.217 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare | | 364 |
moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare
| | 376 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅ 成功 | 95 |
cloudflare | | 162 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅
成功 | 96 | cloudflare | | 320 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅
成功 | 96 | cloudflare | | 462 | damien.ns.cloudflare.com | 162.159.44.168 |
IPv4 | h3 | ✅ 成功 | 96 | cloudflare | | 84 | ipinfo.in | 104.21.21.129 | IPv4
| h3 | ✅ 成功 | 97 | cloudflare | | 257 | abdullah.ns.cloudflare.com |
172.64.35.203 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare | | 25 | www.ipchicken.com
| 172.67.68.101 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare | | 48 | 172.64.229.249
| 172.64.229.249 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare | | 121 | iplocation.io
| 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare | | 155 | 104.18.14.76 |
104.18.14.76 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare | | 196 | japan.com |
104.26.4.60 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare | | 318 | 172.67.79.211 |
172.67.79.211 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare | | 456 | 104.18.78.214 |
104.18.78.214 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare | | 248 | cf.877771.xyz |
104.21.80.180 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare | | 303 | cmcc.877774.xyz
| 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare | | 349 |
craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare
| | 76 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare | |
105 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 307 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare
| | 346 | 172.64.33.67 | 172.64.33.67 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 96 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare | |
147 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h3 | ✅ 成功 | 101 |
cloudflare | | 295 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 | ✅ 成功 |
101 | cloudflare | | 431 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅
成功 | 101 | cloudflare | | 93 | asia.877774.xyz | 104.17.142.146 | IPv4 | h3 |
✅ 成功 | 102 | cloudflare | | 115 | cloudflare-ip.mofashi.ltd | 172.67.155.172
| IPv4 | h3 | ✅ 成功 | 102 | cloudflare | | 137 | www.4chan.org |
104.16.228.229 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare | | 294 |
cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare | | 355
| decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅ 成功 | 102 |
cloudflare | | 408 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h3 | ✅
成功 | 102 | cloudflare | | 107 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 |
✅ 成功 | 103 | cloudflare | | 161 | huxley.ns.cloudflare.com | 162.159.44.188 |
IPv4 | h3 | ✅ 成功 | 103 | cloudflare | | 241 | cu.877774.xyz | 104.26.4.111 |
IPv4 | h3 | ✅ 成功 | 103 | cloudflare | | 247 | cf.877771.xyz | 172.67.152.183
| IPv4 | h3 | ✅ 成功 | 103 | cloudflare | | 268 | 172.64.49.165 | 172.64.49.165
| IPv4 | h3 | ✅ 成功 | 104 | cloudflare | | 71 | www.udemy.com | 104.16.142.237
| IPv4 | h3 | ✅ 成功 | 105 | cloudflare | | 321 | 172.67.110.232 |
172.67.110.232 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare | | 95 | asia.877774.xyz
| 104.17.139.62 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare | | 284 |
cmcc.877774.xyz | 104.16.148.2 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare | | 8 |
yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare | |
176 | www.7749tv.com | 172.67.101.77 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 336 | toy-people.com | 172.67.72.18 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 426 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare | |
441 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h3 | ✅ 成功 | 108 |
cloudflare | | 127 | cf.zhetengsha.eu.org | 172.64.152.241 | IPv4 | h3 | ✅ 成功
| 109 | cloudflare | | 197 | japan.com | 104.26.5.60 | IPv4 | h3 | ✅ 成功 | 109
| cloudflare | | 298 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 |
109 | cloudflare | | 304 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功
| 109 | cloudflare | | 310 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 |
109 | cloudflare | | 335 | toy-people.com | 104.26.2.36 | IPv4 | h3 | ✅ 成功 |
109 | cloudflare | | 292 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功
| 110 | cloudflare | | 319 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功
| 110 | cloudflare | | 392 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3
| ✅ 成功 | 110 | cloudflare | | 397 | rustam.ns.cloudflare.com | 162.159.44.148
| IPv4 | h3 | ✅ 成功 | 110 | cloudflare | | 483 | cfip.xxxxxxxx.tk |
198.41.214.141 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare | | 263 | 104.17.142.12
| 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare | | 302 |
cmcc.877774.xyz | 104.16.149.7 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare | | 403
| icook.hk | 104.21.90.210 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare | | 65 |
www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare | | 103
| ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare | | 111
| saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare | | 113 |
freeyx.cloudflare88.eu.org | 141.101.121.127 | IPv4 | h3 | ✅ 成功 | 112 |
cloudflare | | 171 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 |
112 | cloudflare | | 362 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅
成功 | 112 | cloudflare | | 98 | steamdb.info | 104.20.34.212 | IPv4 | h3 | ✅
成功 | 113 | cloudflare | | 104 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 |
✅ 成功 | 113 | cloudflare | | 132 | ae8a9c24-83de.masx200.ddns-ip.net |
172.67.157.182 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare | | 156 | 162.159.44.208
| 162.159.44.208 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare | | 311 | ipv4.ip.sb |
172.67.75.172 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 47 条记录
- **正常 (100-200ms)**: 53 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 4 次
- **IPv6 失败**: 171 次

### 按协议统计

- **none**: 175 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
