# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/10 17:51:20
- **数据来源**: connectivity_results-20251210-175120.json
- **总测试数**: 489
- **失败测试数**: 182
- **成功测试数**: 307
- **失败率**: 37.22%
- **平均延迟**: 197.90ms
- **最小延迟**: 84ms
- **最大延迟**: 990ms

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 171 次 (94.0%)
- **连接超时: I/O超时**: 9 次 (4.9%)
- **DNS解析错误: 其他DNS错误**: 1 次 (0.5%)
- **连接超时: 上下文超时**: 1 次 (0.5%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (171 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (9 次测试)

| 序号 | 主机/域名        | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ---------------- | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 220  | 121.188.182.190  | 121.188.182.190 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout |
| 314  | 119.194.220.146  | 119.194.220.146 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout |
| 319  | 115.22.115.218   | 115.22.115.218  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 115.22.115.218:443: i/o timeout  |
| 346  | cfip.xxxxxxxx.tk | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 348  | 52.76.110.129    | 52.76.110.129   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 52.76.110.129:443: i/o timeout   |
| 402  | 175.212.207.13   | 175.212.207.13  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout  |
| 408  | 172.64.201.25    | 172.64.201.25   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout   |
| 431  | 111.171.108.67   | 111.171.108.67  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout  |
| 489  | 172.67.49.134    | 172.67.49.134   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout   |

#### DNS解析错误: 其他DNS错误 (1 次测试)

| 序号 | 主机/域名       | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                       |
| ---- | --------------- | --------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------------------------------------------------------------- |
| 5    | 168.138.184.172 | 168.138.184.172 | IPv4   | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": remote error: tls: unrecognized name |

#### 连接超时: 上下文超时 (1 次测试)

| 序号 | 主机/域名      | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                            |
| ---- | -------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------------------------------------------------- |
| 349  | 20.247.137.183 | 20.247.137.183 | IPv4   | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 171 次 (94.0%)
- **连接超时**: 10 次 (5.5%)
- **DNS解析错误**: 1 次 (0.5%)

#### 错误模式分析

**超时集中度分析**: 共有 9 次超时，主要集中在IP段 121.188（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 180 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 11 次，IPv6失败 171 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：singapore.com (3次), toy-people.com
(3次), japan.com (3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 411 |
freeyx.cloudflare88.eu.org | 141.101.121.255 | IPv4 | h3 | ✅ 成功 | 84 |
cloudflare | | 246 | www.okcupid.com | 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 86
| cloudflare | | 253 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功 |
94 | cloudflare | | 485 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功
| 94 | cloudflare | | 328 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅ 成功
| 96 | cloudflare | | 283 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3
| ✅ 成功 | 97 | cloudflare | | 469 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h3
| ✅ 成功 | 97 | cloudflare | | 483 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h3
| ✅ 成功 | 97 | cloudflare | | 486 | cmcc.877774.xyz | 104.16.148.11 | IPv4 |
h3 | ✅ 成功 | 97 | cloudflare | | 417 | stock.hostmonit.com | 172.67.187.251 |
IPv4 | h3 | ✅ 成功 | 100 | cloudflare | | 165 | www.csgo.com | 195.85.59.161 |
IPv4 | h3 | ✅ 成功 | 101 | cloudflare | | 166 | www.csgo.com | 195.85.59.95 |
IPv4 | h3 | ✅ 成功 | 101 | cloudflare | | 396 | braden.ns.cloudflare.com |
108.162.195.169 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare | | 247 |
www.okcupid.com | 104.18.160.63 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare | | 317
| www.4chan.org | 104.16.229.229 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare | |
365 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 420 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 | ✅ 成功 | 109
| cloudflare | | 61 | 172.64.33.67 | 172.64.33.67 | IPv4 | h3 | ✅ 成功 | 110 |
cloudflare | | 188 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅
成功 | 110 | cloudflare | | 462 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 |
✅ 成功 | 110 | cloudflare | | 58 | steamdb.info | 172.66.175.250 | IPv4 | h3 |
✅ 成功 | 111 | cloudflare | | 70 | 104.26.13.31 | 104.26.13.31 | IPv4 | h3 | ✅
成功 | 112 | cloudflare | | 177 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功 |
113 | cloudflare | | 241 | cu.877774.xyz | 104.26.4.114 | IPv4 | h3 | ✅ 成功 |
114 | cloudflare | | 427 | www.gov.ua | 104.21.23.72 | IPv4 | h3 | ✅ 成功 | 114
| cloudflare | | 333 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h3 |
✅ 成功 | 116 | cloudflare | | 40 | 162.159.39.118 | 162.159.39.118 | IPv4 | h3
| ✅ 成功 | 117 | cloudflare | | 361 | eur.877774.xyz | 104.21.47.209 | IPv4 |
h3 | ✅ 成功 | 118 | cloudflare | | 387 | www.digitalocean.com | 104.19.173.68 |
IPv4 | h3 | ✅ 成功 | 118 | cloudflare | | 367 | cf.090227.xyz | 172.66.47.179 |
IPv4 | h3 | ✅ 成功 | 119 | cloudflare | | 16 | fbi.gov | 104.16.148.244 | IPv4
| h3 | ✅ 成功 | 120 | cloudflare | | 249 | www.okcupid.com | 104.16.223.254 |
IPv4 | h3 | ✅ 成功 | 120 | cloudflare | | 356 | pranab.ns.cloudflare.com |
172.64.35.199 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare | | 379 | shopify.com |
23.227.38.33 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare | | 432 | 172.64.154.18 |
172.64.154.18 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare | | 84 |
cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare
| | 144 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h3 | ✅ 成功 | 122 |
cloudflare | | 239 | cu.877774.xyz | 104.26.4.112 | IPv4 | h3 | ✅ 成功 | 122 |
cloudflare | | 289 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 122 |
cloudflare | | 79 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 |
IPv4 | h3 | ✅ 成功 | 123 | cloudflare | | 428 | www.gov.ua | 172.67.209.127 |
IPv4 | h3 | ✅ 成功 | 123 | cloudflare | | 171 | uriah.ns.cloudflare.com |
108.162.195.194 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare | | 270 |
craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h3 | ✅ 成功 | 124 |
cloudflare | | 376 | cf.0sm.com | 172.67.187.145 | IPv4 | h3 | ✅ 成功 | 124 |
cloudflare | | 467 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 | 124
| cloudflare | | 90 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h3 | ✅
成功 | 125 | cloudflare | | 371 | yx-auto.pages.dev | 172.67.134.139 | IPv4 | h3
| ✅ 成功 | 125 | cloudflare | | 91 | cloudflare-ip.mofashi.ltd | 172.67.155.172
| IPv4 | h3 | ✅ 成功 | 127 | cloudflare | | 102 | ct.877774.xyz |
172.64.229.161 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare | | 403 | 172.64.146.16
| 172.64.146.16 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare | | 12 | icook.tw |
172.66.158.115 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare | | 225 |
www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare | |
231 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 263 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅ 成功 | 128 |
cloudflare | | 262 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h3 | ✅
成功 | 130 | cloudflare | | 447 | 456.cloudflare.182682.xyz | 104.26.8.160 |
IPv4 | h3 | ✅ 成功 | 130 | cloudflare | | 21 | palera.in | 172.67.157.122 |
IPv4 | h3 | ✅ 成功 | 131 | cloudflare | | 24 | 104.18.14.76 | 104.18.14.76 |
IPv4 | h3 | ✅ 成功 | 131 | cloudflare | | 77 | 104.16.45.84 | 104.16.45.84 |
IPv4 | h3 | ✅ 成功 | 131 | cloudflare | | 195 | dnschecker.org | 104.26.6.89 |
IPv4 | h3 | ✅ 成功 | 131 | cloudflare | | 443 | gamer.com.tw | 104.18.2.197 |
IPv4 | h3 | ✅ 成功 | 131 | cloudflare | | 433 | xn--b6gac.eu.org |
172.67.153.253 | IPv4 | h3 | ✅ 成功 | 132 | cloudflare | | 69 |
cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare | |
256 | ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare | | 437 |
bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 133 |
cloudflare | | 453 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h3 | ✅
成功 | 133 | cloudflare | | 25 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅
成功 | 134 | cloudflare | | 202 | www.visa.cn | 162.159.153.2 | IPv4 | h3 | ✅
成功 | 134 | cloudflare | | 366 | cf.090227.xyz | 172.66.44.77 | IPv4 | h3 | ✅
成功 | 134 | cloudflare | | 391 | sullivan.ns.cloudflare.com | 162.159.44.161 |
IPv4 | h3 | ✅ 成功 | 134 | cloudflare | | 95 | ct.877774.xyz | 172.64.229.173 |
IPv4 | h3 | ✅ 成功 | 135 | cloudflare | | 101 | ct.877774.xyz | 172.64.229.44 |
IPv4 | h3 | ✅ 成功 | 135 | cloudflare | | 351 | www.ipget.net | 104.21.15.212 |
IPv4 | h3 | ✅ 成功 | 135 | cloudflare | | 347 | cfip.xxxxxxxx.tk |
104.16.232.223 | IPv4 | h3 | ✅ 成功 | 136 | cloudflare | | 465 |
cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 | 136 | cloudflare | | 152
| www.hugedomains.com | 104.26.6.37 | IPv4 | h3 | ✅ 成功 | 137 | cloudflare | |
360 | eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 137 | cloudflare |
| 421 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h3 | ✅ 成功 | 137
| cloudflare | | 455 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h3 | ✅
成功 | 137 | cloudflare | | 470 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h3 |
✅ 成功 | 137 | cloudflare | | 162 | cf.zhetengsha.eu.org | 172.66.47.179 | IPv4
| h3 | ✅ 成功 | 138 | cloudflare | | 250 | www.okcupid.com | 104.16.144.63 |
IPv4 | h3 | ✅ 成功 | 138 | cloudflare | | 362 | eur.877774.xyz | 104.21.26.150
| IPv4 | h3 | ✅ 成功 | 138 | cloudflare | | 382 |
ae8a9c24-83de.masx200.ddns-ip.net | 104.21.14.41 | IPv4 | h3 | ✅ 成功 | 138 |
cloudflare | | 53 | www.udemy.com | 104.16.143.237 | IPv4 | h3 | ✅ 成功 | 139 |
cloudflare | | 78 | local-aria2-webui.masx200.ddns-ip.net | 104.21.14.41 | IPv4
| h3 | ✅ 成功 | 139 | cloudflare | | 138 | www.wto.org | 104.18.41.190 | IPv4 |
h3 | ✅ 成功 | 139 | cloudflare | | 233 | zread.ai | 104.21.76.240 | IPv4 | h3 |
✅ 成功 | 139 | cloudflare | | 364 | asia.877774.xyz | 104.16.211.153 | IPv4 |
h3 | ✅ 成功 | 139 | cloudflare | | 124 | dylan.ns.cloudflare.com |
108.162.195.187 | IPv4 | h3 | ✅ 成功 | 140 | cloudflare | | 237 | cu.877774.xyz
| 104.26.4.119 | IPv4 | h3 | ✅ 成功 | 140 | cloudflare | | 482 |
cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 | ✅ 成功 | 140 | cloudflare | | 34 |
toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 141 | cloudflare | | 49 |
ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅ 成功 | 141 | cloudflare | | 224 |
ipv4.ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 141 | cloudflare | | 343 |
cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h3 | ✅ 成功 | 141 | cloudflare | |
110 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 142 |
cloudflare | | 143 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h3 | ✅
成功 | 142 | cloudflare | | 160 | 162.159.36.104 | 162.159.36.104 | IPv4 | h3 |
✅ 成功 | 144 | cloudflare | | 340 | 172.64.148.15 | 172.64.148.15 | IPv4 | h3 |
✅ 成功 | 144 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 9 条记录
- **正常 (100-200ms)**: 91 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 11 次
- **IPv6 失败**: 171 次

### 按协议统计

- **none**: 180 次失败
- **h2**: 2 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
