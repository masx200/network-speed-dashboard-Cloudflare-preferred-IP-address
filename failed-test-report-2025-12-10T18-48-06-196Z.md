# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/10 18:48:06
- **数据来源**: connectivity_results-20251210-184806.json
- **总测试数**: 483
- **失败测试数**: 179
- **成功测试数**: 304
- **失败率**: 37.06%
- **平均延迟**: 139.14ms
- **最小延迟**: 62ms
- **最大延迟**: 535ms

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 172 次 (96.1%)
- **连接超时: I/O超时**: 7 次 (3.9%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (172 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (7 次测试)

| 序号 | 主机/域名        | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ---------------- | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 472  | 121.188.182.190  | 121.188.182.190 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout |
| 477  | cfip.xxxxxxxx.tk | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 479  | 111.171.108.67   | 111.171.108.67  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout  |
| 480  | 175.212.207.13   | 175.212.207.13  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout  |
| 481  | cf.877774.xyz    | cf.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout   |
| 482  | ct.877774.xyz    | ct.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout   |
| 483  | 119.194.220.146  | 119.194.220.146 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 172 次 (96.1%)
- **连接超时**: 7 次 (3.9%)

#### 错误模式分析

**超时集中度分析**: 共有 7 次超时，主要集中在IP段 121.188（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 179 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 7 次，IPv6失败 172 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：dnschecker.org (3次),
ashton.ns.cloudflare.com (3次), 456.cloudflare.182682.xyz
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 110 |
steamdb.info | 172.66.175.250 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare | | 259 |
cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare | | 233
| cu.877774.xyz | 104.26.4.119 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare | | 362 |
eur.877774.xyz | 104.21.26.150 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare | | 51 |
172.64.229.249 | 172.64.229.249 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare | | 186
| www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare | |
458 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare | | 250
| dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h3 | ✅ 成功 | 70 |
cloudflare | | 443 | yx-auto.pages.dev | 104.21.9.230 | IPv4 | h3 | ✅ 成功 | 70
| cloudflare | | 263 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功 | 71
| cloudflare | | 355 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅
成功 | 75 | cloudflare | | 426 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4
| h3 | ✅ 成功 | 75 | cloudflare | | 377 | moura.ns.cloudflare.com |
108.162.195.217 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare | | 405 | icook.hk |
172.67.161.104 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare | | 433 | www.okcupid.com
| 104.17.48.63 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare | | 256 | cmcc.877774.xyz
| 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare | | 337 | toy-people.com
| 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare | | 187 |
www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare | |
453 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 78 |
cloudflare | | 301 | 172.64.153.172 | 172.64.153.172 | IPv4 | h3 | ✅ 成功 | 79
| cloudflare | | 238 | cu.877774.xyz | 104.26.4.115 | IPv4 | h3 | ✅ 成功 | 81 |
cloudflare | | 270 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 83
| cloudflare | | 203 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 |
85 | cloudflare | | 432 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功
| 85 | cloudflare | | 191 | 172.64.157.120 | 172.64.157.120 | IPv4 | h3 | ✅
成功 | 88 | cloudflare | | 294 | time.is | 104.26.12.54 | IPv4 | h3 | ✅ 成功 |
88 | cloudflare | | 237 | cu.877774.xyz | 104.26.4.114 | IPv4 | h3 | ✅ 成功 |
90 | cloudflare | | 302 | fbi.gov | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 90 |
cloudflare | | 196 | japan.com | 172.67.70.92 | IPv4 | h3 | ✅ 成功 | 92 |
cloudflare | | 165 | yx-auto.pages.dev | 104.21.6.60 | IPv4 | h3 | ✅ 成功 | 93
| cloudflare | | 57 | 172.64.41.88 | 172.64.41.88 | IPv4 | h3 | ✅ 成功 | 94 |
cloudflare | | 50 | 172.64.148.15 | 172.64.148.15 | IPv4 | h3 | ✅ 成功 | 95 |
cloudflare | | 140 | na.877774.xyz | 104.19.74.233 | IPv4 | h3 | ✅ 成功 | 95 |
cloudflare | | 291 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅ 成功 | 95 |
cloudflare | | 146 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 97 |
cloudflare | | 261 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅ 成功 | 97 |
cloudflare | | 275 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅ 成功 | 97 |
cloudflare | | 411 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅
成功 | 97 | cloudflare | | 31 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅
成功 | 99 | cloudflare | | 239 | cu.877774.xyz | 104.26.4.116 | IPv4 | h3 | ✅
成功 | 99 | cloudflare | | 321 | 172.64.146.16 | 172.64.146.16 | IPv4 | h3 | ✅
成功 | 99 | cloudflare | | 38 | www.ipchicken.com | 172.67.68.101 | IPv4 | h3 |
✅ 成功 | 100 | cloudflare | | 142 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3
| ✅ 成功 | 100 | cloudflare | | 386 | kyree.ns.cloudflare.com | 162.159.44.207
| IPv4 | h3 | ✅ 成功 | 100 | cloudflare | | 129 | www.4chan.org |
104.16.229.229 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare | | 204 |
yx-auto.pages.dev | 172.66.44.144 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare | |
218 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare | |
317 | www.gov.ua | 104.21.23.72 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare | | 447
| trevor.ns.cloudflare.com | 4444.cloudflare.182682.xyz | IPv4 | h3 | ✅ 成功 | 101 |
cloudflare | | 81 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 | ✅ 成功 | 102
| cloudflare | | 255 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h3 | ✅ 成功 |
102 | cloudflare | | 387 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3 |
✅ 成功 | 102 | cloudflare | | 473 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 |
h3 | ✅ 成功 | 102 | cloudflare | | 359 | 172.64.33.67 | 172.64.33.67 | IPv4 |
h3 | ✅ 成功 | 103 | cloudflare | | 9 | bestcf.030101.xyz | 104.17.179.12 | IPv4
| h3 | ✅ 成功 | 104 | cloudflare | | 104 | freeyx.cloudflare88.eu.org |
141.101.120.230 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare | | 115 | iplocation.io
| 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare | | 166 |
yx-auto.pages.dev | 172.67.134.139 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare | |
56 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare | |
303 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare | | 26 |
ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h3 | ✅ 成功 | 107 |
cloudflare | | 52 | ip.gs | 104.21.14.176 | IPv4 | h3 | ✅ 成功 | 107 |
cloudflare | | 95 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h3 | ✅ 成功 |
108 | cloudflare | | 103 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅ 成功
| 108 | cloudflare | | 265 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h3 | ✅
成功 | 108 | cloudflare | | 392 | julio.ns.cloudflare.com | 162.159.44.209 |
IPv4 | h3 | ✅ 成功 | 108 | cloudflare | | 436 | www.okcupid.com |
104.16.239.254 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare | | 113 | 172.64.154.18
| 172.64.154.18 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare | | 171 |
www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare | |
373 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare |
| 457 | icook.tw | 104.20.28.74 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare | | 20
| dnschecker.org | 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare | | 86
| 4444.cloudflare.182682.xyz | 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare | | 153
| ifconfig.co | 104.21.54.91 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare | | 207 |
bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 110 |
cloudflare | | 331 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 110 |
cloudflare | | 368 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h3 | ✅
成功 | 110 | cloudflare | | 39 | 456.cloudflare.182682.xyz | 172.67.75.208 |
IPv4 | h3 | ✅ 成功 | 111 | cloudflare | | 134 | cf.zhetengsha.eu.org |
104.18.35.15 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare | | 254 | cmcc.877774.xyz
| 104.16.148.8 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare | | 385 |
kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅ 成功 | 111 |
cloudflare | | 72 | www.udemy.com | 104.16.143.237 | IPv4 | h3 | ✅ 成功 | 112 |
cloudflare | | 273 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h3 | ✅ 成功 | 112
| cloudflare | | 366 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h3 |
✅ 成功 | 112 | cloudflare | | 18 | dnschecker.org | 104.26.6.89 | IPv4 | h3 |
✅ 成功 | 113 | cloudflare | | 258 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3
| ✅ 成功 | 113 | cloudflare | | 262 | cmcc.877774.xyz | 104.16.149.3 | IPv4 |
h3 | ✅ 成功 | 113 | cloudflare | | 148 | ct.877774.xyz | 172.64.229.195 | IPv4
| h3 | ✅ 成功 | 114 | cloudflare | | 224 | www.csgo.com | 195.85.59.161 | IPv4
| h3 | ✅ 成功 | 114 | cloudflare | | 12 | www.pcmag.com | 104.16.21.118 | IPv4
| h3 | ✅ 成功 | 115 | cloudflare | | 145 | ct.877774.xyz | 172.64.229.173 |
IPv4 | h3 | ✅ 成功 | 115 | cloudflare | | 417 | pranab.ns.cloudflare.com |
172.64.35.199 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare | | 428 |
cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare
| | 470 | www.4444.cloudflare.182682.xyz | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare |
| 215 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h3 | ✅
成功 | 116 | cloudflare | | 374 | www.visa.com.hk | 104.18.21.69 | IPv4 | h3 |
✅ 成功 | 116 | cloudflare | | 391 | julio.ns.cloudflare.com | 108.162.195.209 |
IPv4 | h3 | ✅ 成功 | 116 | cloudflare | | 423 | palera.in | 104.21.58.72 | IPv4
| h3 | ✅ 成功 | 116 | cloudflare | | 49 | gamer.com.tw | 104.18.3.197 | IPv4 |
h3 | ✅ 成功 | 117 | cloudflare | | 307 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3
| ✅ 成功 | 117 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 41 条记录
- **正常 (100-200ms)**: 59 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 7 次
- **IPv6 失败**: 172 次

### 按协议统计

- **none**: 179 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
