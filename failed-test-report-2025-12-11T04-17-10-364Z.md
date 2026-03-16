# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/11 04:17:10
- **数据来源**: connectivity_results-20251211-041710.json
- **总测试数**: 486
- **失败测试数**: 177
- **成功测试数**: 309
- **失败率**: 36.42%
- **平均延迟**: 103.21ms
- **最小延迟**: 43ms
- **最大延迟**: 735ms

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 172 次 (97.2%)
- **连接超时: I/O超时**: 4 次 (2.3%)
- **连接超时: 上下文超时**: 1 次 (0.6%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (172 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (4 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 472  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 473  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 479  | cfip.xxxxxxxx.tk | 104.20.255.53  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.20.255.53:443: i/o timeout  |
| 484  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |

#### 连接超时: 上下文超时 (1 次测试)

| 序号 | 主机/域名      | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                            |
| ---- | -------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------------------------------------------------- |
| 471  | 34.143.159.175 | 34.143.159.175 | IPv4   | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 172 次 (97.2%)
- **连接超时**: 5 次 (2.8%)

#### 错误模式分析

**超时集中度分析**: 共有 4 次超时，主要集中在IP段 172.67（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 176 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 5 次，IPv6失败 172 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：dnschecker.org (3次),
ashton.ns.cloudflare.com (3次), iplocation.io
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 385 |
icook.hk | 104.21.90.210 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare | | 279 |
cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare | | 302
| benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 | ✅ 成功 | 50 |
cloudflare | | 273 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 | ✅ 成功 | 51 |
cloudflare | | 174 | comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 53 |
cloudflare | | 372 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅
成功 | 54 | cloudflare | | 381 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4
| h3 | ✅ 成功 | 54 | cloudflare | | 299 | cu.877774.xyz | 104.26.4.119 | IPv4 |
h3 | ✅ 成功 | 61 | cloudflare | | 460 | 456.cloudflare.182682.xyz |
104.26.9.160 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare | | 91 | iplocation.io |
104.26.11.222 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare | | 238 | xn--b6gac.eu.org
| 104.21.90.78 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare | | 70 |
stock.hostmonit.com | 104.21.7.193 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare | |
386 | icook.hk | 172.67.161.104 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare | | 456
| bestcf.030101.xyz | 104.19.149.213 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare | |
129 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare | |
322 | shopify.com | 23.227.38.33 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare | | 207
| www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare | | 420 |
cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare |
| 60 | zread.ai | 172.67.202.78 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare | | 82 |
steamdb.info | 172.66.175.250 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare | | 304 |
benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅ 成功 | 74 |
cloudflare | | 379 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅
成功 | 74 | cloudflare | | 416 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 |
✅ 成功 | 74 | cloudflare | | 444 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3
| ✅ 成功 | 74 | cloudflare | | 475 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 |
h3 | ✅ 成功 | 75 | cloudflare | | 109 | ifconfig.co | 104.21.54.91 | IPv4 | h3
| ✅ 成功 | 76 | cloudflare | | 140 | huxley.ns.cloudflare.com | 108.162.195.188
| IPv4 | h3 | ✅ 成功 | 76 | cloudflare | | 154 | cf.zhetengsha.eu.org |
104.18.42.98 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare | | 186 | 172.64.156.195 |
172.64.156.195 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare | | 378 | 172.67.110.232
| 172.67.110.232 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare | | 478 |
cfip.xxxxxxxx.tk | 104.17.127.110 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare | | 69
| cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare | |
245 | time.is | 104.26.12.54 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare | | 320 |
104.17.68.85 | 104.17.68.85 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare | | 467 |
www.visa.cn | 162.159.153.2 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare | | 481 |
cfip.xxxxxxxx.tk | 190.93.246.67 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare | | 45
| cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare | | 90 |
iplocation.io | 104.26.10.222 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare | | 98 |
ae8a9c24-83de.masx200.ddns-ip.net | 104.21.14.41 | IPv4 | h3 | ✅ 成功 | 78 |
cloudflare | | 126 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 78 |
cloudflare | | 159 | www.digitalocean.com | 104.19.174.68 | IPv4 | h3 | ✅ 成功
| 78 | cloudflare | | 477 | cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h3 | ✅
成功 | 78 | cloudflare | | 271 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h3 | ✅
成功 | 79 | cloudflare | | 296 | cu.877774.xyz | 104.26.4.116 | IPv4 | h3 | ✅
成功 | 79 | cloudflare | | 330 | toy-people.com | 172.67.72.18 | IPv4 | h3 | ✅
成功 | 79 | cloudflare | | 338 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4
| h3 | ✅ 成功 | 79 | cloudflare | | 367 | eur.877774.xyz | 104.21.47.209 | IPv4
| h3 | ✅ 成功 | 79 | cloudflare | | 441 | yx-auto.pages.dev | 172.67.161.98 |
IPv4 | h3 | ✅ 成功 | 79 | cloudflare | | 12 | gamer.com.tw | 104.18.2.197 |
IPv4 | h3 | ✅ 成功 | 80 | cloudflare | | 13 | gamer.com.tw | 104.18.3.197 |
IPv4 | h3 | ✅ 成功 | 80 | cloudflare | | 59 | 172.64.40.9 | 172.64.40.9 | IPv4
| h3 | ✅ 成功 | 80 | cloudflare | | 92 | iplocation.io | 172.67.70.100 | IPv4 |
h3 | ✅ 成功 | 80 | cloudflare | | 252 | 172.64.153.172 | 172.64.153.172 | IPv4
| h3 | ✅ 成功 | 80 | cloudflare | | 297 | cu.877774.xyz | 104.26.4.117 | IPv4 |
h3 | ✅ 成功 | 80 | cloudflare | | 321 | 172.67.75.172 | 172.67.75.172 | IPv4 |
h3 | ✅ 成功 | 80 | cloudflare | | 482 | cfip.xxxxxxxx.tk | 104.16.232.223 |
IPv4 | h3 | ✅ 成功 | 80 | cloudflare | | 19 | ip.gs | 172.67.160.28 | IPv4 | h3
| ✅ 成功 | 81 | cloudflare | | 24 | www.ipchicken.com | 104.26.7.112 | IPv4 |
h3 | ✅ 成功 | 81 | cloudflare | | 237 | xn--b6gac.eu.org | 172.67.153.253 |
IPv4 | h3 | ✅ 成功 | 81 | cloudflare | | 430 | trevor.ns.cloudflare.com |
108.162.195.154 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare | | 437 |
whatismyipaddress.com | 104.19.223.79 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare |
| 446 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare | | 33
| 172.64.38.15 | 172.64.38.15 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare | | 96 |
www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare | | 114 |
lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h3 | ✅ 成功 | 82 | cloudflare
| | 121 | freeyx.cloudflare88.eu.org | 141.101.120.10 | IPv4 | h3 | ✅ 成功 | 82
| cloudflare | | 189 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h3 | ✅
成功 | 82 | cloudflare | | 264 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 |
✅ 成功 | 82 | cloudflare | | 436 | whatismyipaddress.com | 104.19.222.79 | IPv4
| h3 | ✅ 成功 | 82 | cloudflare | | 89 | 172.64.154.18 | 172.64.154.18 | IPv4 |
h3 | ✅ 成功 | 83 | cloudflare | | 120 | freeyx.cloudflare88.eu.org |
141.101.120.246 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare | | 187 |
bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare
| | 224 | 103.160.204.59 | 103.160.204.59 | IPv4 | h3 | ✅ 成功 | 83 |
cloudflare | | 246 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 | 83 |
cloudflare | | 276 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 83
| cloudflare | | 344 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h3 |
✅ 成功 | 83 | cloudflare | | 424 | otto.ns.cloudflare.com | 108.162.195.135 |
IPv4 | h3 | ✅ 成功 | 83 | cloudflare | | 440 | yx-auto.pages.dev | 104.21.9.230
| IPv4 | h3 | ✅ 成功 | 83 | cloudflare | | 275 | cmcc.877774.xyz | 104.16.148.9
| IPv4 | h3 | ✅ 成功 | 84 | cloudflare | | 292 | cu.877774.xyz | 104.26.4.112 |
IPv4 | h3 | ✅ 成功 | 84 | cloudflare | | 43 | 172.64.41.88 | 172.64.41.88 |
IPv4 | h3 | ✅ 成功 | 85 | cloudflare | | 44 | cf.877774.xyz | 172.64.146.66 |
IPv4 | h3 | ✅ 成功 | 85 | cloudflare | | 65 | ipinfo.in | 104.21.21.129 | IPv4
| h3 | ✅ 成功 | 85 | cloudflare | | 86 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3
| ✅ 成功 | 85 | cloudflare | | 153 | na.877774.xyz | 104.19.74.233 | IPv4 | h3
| ✅ 成功 | 85 | cloudflare | | 233 | abdullah.ns.cloudflare.com | 172.64.35.203
| IPv4 | h3 | ✅ 成功 | 85 | cloudflare | | 291 | ipv4.ip.sb | 172.67.75.172 |
IPv4 | h3 | ✅ 成功 | 85 | cloudflare | | 358 | www.glassdoor.com | 104.17.64.70
| IPv4 | h3 | ✅ 成功 | 85 | cloudflare | | 368 | eur.877774.xyz | 104.21.26.150
| IPv4 | h3 | ✅ 成功 | 85 | cloudflare | | 391 | rustam.ns.cloudflare.com |
172.64.35.148 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare | | 408 | 172.67.106.26 |
172.67.106.26 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare | | 414 | www.okcupid.com
| 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare | | 415 |
www.okcupid.com | 104.17.48.63 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare | | 457 |
bestcf.030101.xyz | 104.19.153.123 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare | |
97 | www.4chan.org | 104.16.229.229 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare | |
215 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h3 | ✅
成功 | 86 | cloudflare | | 29 | dnschecker.org | 172.67.73.216 | IPv4 | h3 | ✅
成功 | 87 | cloudflare | | 112 | lewis.ns.cloudflare.com | 108.162.195.159 |
IPv4 | h3 | ✅ 成功 | 87 | cloudflare | | 127 | ct.877774.xyz | 172.64.229.174 |
IPv4 | h3 | ✅ 成功 | 87 | cloudflare | | 193 | 104.16.61.163 | 104.16.61.163 |
IPv4 | h3 | ✅ 成功 | 87 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 2 条记录
- **快 (50-100ms)**: 98 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 5 次
- **IPv6 失败**: 172 次

### 按协议统计

- **none**: 176 次失败
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
