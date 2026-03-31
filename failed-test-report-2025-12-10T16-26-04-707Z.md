# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/10 16:26:04
- **数据来源**: connectivity_results.json
- **总测试数**: 478
- **失败测试数**: 178
- **成功测试数**: 300
- **失败率**: 37.24%
- **平均延迟**: 164.55ms
- **最小延迟**: 44ms
- **最大延迟**: 1309ms

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 168 次 (94.4%)
- **连接超时: I/O超时**: 9 次 (5.1%)
- **连接超时: 上下文超时**: 1 次 (0.6%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (168 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (9 次测试)

| 序号 | 主机/域名        | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ---------------- | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 351  | 119.194.220.146  | 119.194.220.146 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout |
| 362  | ct.877774.xyz    | ct.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout   |
| 401  | cfip.xxxxxxxx.tk | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 402  | 52.76.110.129    | 52.76.110.129   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 52.76.110.129:443: i/o timeout   |
| 403  | 121.188.182.190  | 121.188.182.190 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout |
| 474  | 115.22.115.218   | 115.22.115.218  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 115.22.115.218:443: i/o timeout  |
| 476  | 111.171.108.67   | 111.171.108.67  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout  |
| 477  | cf.877774.xyz    | cf.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout   |
| 478  | 175.212.207.13   | 175.212.207.13  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout  |

#### 连接超时: 上下文超时 (1 次测试)

| 序号 | 主机/域名      | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                            |
| ---- | -------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------------------------------------------------- |
| 438  | 20.247.137.183 | 20.247.137.183 | IPv4   | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 168 次 (94.4%)
- **连接超时**: 10 次 (5.6%)

#### 错误模式分析

**超时集中度分析**: 共有 9 次超时，主要集中在IP段 119.194（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 177 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 10 次，IPv6失败 168 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：ashton.ns.cloudflare.com (3次),
braden.ns.cloudflare.com (3次), trevor.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 383 |
172.64.229.249 | 172.64.229.249 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare | | 466
| ct.877774.xyz | 172.64.229.217 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare | | 48
| www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare | | 455 |
benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅ 成功 | 58 |
cloudflare | | 335 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 60 |
cloudflare | | 417 | www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 60 |
cloudflare | | 447 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅
成功 | 60 | cloudflare | | 343 | www.wto.org | 104.18.41.190 | IPv4 | h3 | ✅
成功 | 66 | cloudflare | | 100 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4
| h3 | ✅ 成功 | 67 | cloudflare | | 418 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3
| ✅ 成功 | 69 | cloudflare | | 243 | 104.18.78.214 | 104.18.78.214 | IPv4 | h3
| ✅ 成功 | 70 | cloudflare | | 137 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h3 |
✅ 成功 | 71 | cloudflare | | 464 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 |
✅ 成功 | 72 | cloudflare | | 416 | www.4chan.org | 104.16.229.229 | IPv4 | h3 |
✅ 成功 | 76 | cloudflare | | 151 | 104.18.254.88 | 104.18.254.88 | IPv4 | h3 |
✅ 成功 | 77 | cloudflare | | 292 | local-aria2-webui.masx200.ddns-ip.net |
172.67.157.182 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare | | 359 |
freeyx.cloudflare88.eu.org | 141.101.120.229 | IPv4 | h3 | ✅ 成功 | 86 |
cloudflare | | 190 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 88 |
cloudflare | | 220 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 93
| cloudflare | | 144 | singapore.com | 104.26.13.140 | IPv4 | h3 | ✅ 成功 | 94
| cloudflare | | 216 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h3 | ✅ 成功 | 94
| cloudflare | | 391 | dnschecker.org | 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 94
| cloudflare | | 31 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅
成功 | 95 | cloudflare | | 121 | 104.19.223.58 | 104.19.223.58 | IPv4 | h3 | ✅
成功 | 97 | cloudflare | | 252 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅
成功 | 98 | cloudflare | | 225 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅
成功 | 99 | cloudflare | | 338 | japan.com | 104.26.5.60 | IPv4 | h3 | ✅ 成功 |
99 | cloudflare | | 411 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h3
| ✅ 成功 | 99 | cloudflare | | 88 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅
成功 | 100 | cloudflare | | 131 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 |
100 | cloudflare | | 154 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅ 成功 |
100 | cloudflare | | 325 | cf.877771.xyz | 172.67.152.183 | IPv4 | h3 | ✅ 成功
| 100 | cloudflare | | 376 | na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅ 成功
| 100 | cloudflare | | 146 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功
| 101 | cloudflare | | 195 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功
| 103 | cloudflare | | 395 | cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h3 | ✅
成功 | 103 | cloudflare | | 163 | abdullah.ns.cloudflare.com | 108.162.195.203 |
IPv4 | h3 | ✅ 成功 | 104 | cloudflare | | 224 | cmcc.877774.xyz | 104.16.148.2
| IPv4 | h3 | ✅ 成功 | 104 | cloudflare | | 445 | 172.64.41.88 | 172.64.41.88 |
IPv4 | h3 | ✅ 成功 | 104 | cloudflare | | 159 | www.visa.com.sg | 104.18.13.229
| IPv4 | h3 | ✅ 成功 | 105 | cloudflare | | 162 | 104.16.61.163 | 104.16.61.163
| IPv4 | h3 | ✅ 成功 | 105 | cloudflare | | 454 | benedict.ns.cloudflare.com |
162.159.44.205 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare | | 13 |
ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h3 | ✅ 成功 | 106 |
cloudflare | | 19 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h3 | ✅
成功 | 106 | cloudflare | | 211 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 |
✅ 成功 | 106 | cloudflare | | 258 | www.okcupid.com | 104.16.239.254 | IPv4 |
h3 | ✅ 成功 | 106 | cloudflare | | 412 | pranab.ns.cloudflare.com |
172.64.35.199 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare | | 55 |
www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare | |
247 | www.hugedomains.com | 104.26.7.37 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare
| | 97 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 108 |
cloudflare | | 136 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 108 |
cloudflare | | 219 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 108
| cloudflare | | 339 | japan.com | 104.26.4.60 | IPv4 | h3 | ✅ 成功 | 108 |
cloudflare | | 440 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h3 | ✅
成功 | 108 | cloudflare | | 57 | whatismyipaddress.com | 104.19.223.79 | IPv4 |
h3 | ✅ 成功 | 109 | cloudflare | | 140 | wilson.ns.cloudflare.com |
172.64.35.110 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare | | 463 | ct.877774.xyz |
172.64.229.174 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare | | 9 | 172.64.38.15 |
172.64.38.15 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare | | 305 |
cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h3 | ✅ 成功 | 110 |
cloudflare | | 54 | www.ipchicken.com | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 111
| cloudflare | | 217 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h3 | ✅ 成功 |
111 | cloudflare | | 231 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h3 | ✅ 成功
| 111 | cloudflare | | 296 | 162.159.38.89 | 162.159.38.89 | IPv4 | h3 | ✅ 成功
| 112 | cloudflare | | 303 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅
成功 | 112 | cloudflare | | 328 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅
成功 | 112 | cloudflare | | 446 | 172.64.33.67 | 172.64.33.67 | IPv4 | h3 | ✅
成功 | 112 | cloudflare | | 276 | cu.877774.xyz | 104.26.4.118 | IPv4 | h3 | ✅
成功 | 113 | cloudflare | | 332 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功 |
113 | cloudflare | | 186 | toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 |
114 | cloudflare | | 253 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 |
114 | cloudflare | | 45 | palera.in | 172.67.157.122 | IPv4 | h3 | ✅ 成功 | 115
| cloudflare | | 76 | www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 |
115 | cloudflare | | 130 | ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 115 |
cloudflare | | 135 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 115 |
cloudflare | | 441 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅
成功 | 115 | cloudflare | | 467 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 |
✅ 成功 | 115 | cloudflare | | 470 | lewis.ns.cloudflare.com | 172.64.35.159 |
IPv4 | h3 | ✅ 成功 | 115 | cloudflare | | 193 | 172.64.148.15 | 172.64.148.15 |
IPv4 | h3 | ✅ 成功 | 116 | cloudflare | | 277 | cu.877774.xyz | 104.26.4.119 |
IPv4 | h3 | ✅ 成功 | 116 | cloudflare | | 385 | comicabc.com | 172.67.174.21 |
IPv4 | h3 | ✅ 成功 | 116 | cloudflare | | 419 | bowen.ns.cloudflare.com |
108.162.195.83 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare | | 205 |
cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare
| | 379 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare
| | 145 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare
| | 171 | steamdb.info | 104.20.34.212 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare
| | 260 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare | |
267 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare | |
49 | www.visa.cn | 162.159.153.2 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare | |
198 | cf.zhetengsha.eu.org | 172.66.44.77 | IPv4 | h3 | ✅ 成功 | 119 |
cloudflare | | 222 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 |
119 | cloudflare | | 299 | 456.cloudflare.182682.xyz | 172.67.75.208 | IPv4 | h3
| ✅ 成功 | 119 | cloudflare | | 375 | asia.877774.xyz | 104.16.211.153 | IPv4 |
h3 | ✅ 成功 | 119 | cloudflare | | 221 | cmcc.877774.xyz | 104.16.149.12 | IPv4
| h3 | ✅ 成功 | 120 | cloudflare | | 227 | cmcc.877774.xyz | 104.16.148.5 |
IPv4 | h3 | ✅ 成功 | 120 | cloudflare | | 273 | cu.877774.xyz | 104.26.4.115 |
IPv4 | h3 | ✅ 成功 | 120 | cloudflare | | 25 | trevor.ns.cloudflare.com |
172.64.35.154 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare | | 259 | www.okcupid.com
| 104.17.48.63 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare | | 304 |
cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h3 | ✅ 成功 | 121 |
cloudflare | | 380 | www.ipget.net | 104.21.15.212 | IPv4 | h3 | ✅ 成功 | 121 |
cloudflare | | 388 | 103.160.204.59 | 103.160.204.59 | IPv4 | h3 | ✅ 成功 | 121
| cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 1 条记录
- **快 (50-100ms)**: 27 条记录
- **正常 (100-200ms)**: 72 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 10 次
- **IPv6 失败**: 168 次

### 按协议统计

- **none**: 177 次失败
- **h2**: 1 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
