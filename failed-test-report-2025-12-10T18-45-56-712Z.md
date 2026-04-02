# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/10 18:45:56
- **数据来源**: connectivity_results-20251210-184556.json
- **总测试数**: 481
- **失败测试数**: 178
- **成功测试数**: 303
- **失败率**: 37.01%
- **平均延迟**: 140.06ms
- **最小延迟**: 40ms
- **最大延迟**: 890ms

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 171 次 (96.1%)
- **连接超时: I/O超时**: 7 次 (3.9%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (171 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (7 次测试)

| 序号 | 主机/域名        | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ---------------- | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 472  | cfip.xxxxxxxx.tk | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 476  | 111.171.108.67   | 111.171.108.67  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout  |
| 477  | 175.212.207.13   | 175.212.207.13  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout  |
| 478  | cf.877774.xyz    | cf.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout   |
| 479  | ct.877774.xyz    | ct.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout   |
| 480  | 119.194.220.146  | 119.194.220.146 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout |
| 481  | 121.188.182.190  | 121.188.182.190 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 171 次 (96.1%)
- **连接超时**: 7 次 (3.9%)

#### 错误模式分析

**超时集中度分析**: 共有 7 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 178 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 7 次，IPv6失败 171 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：ashton.ns.cloudflare.com (3次),
iplocation.io (3次), lewis.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 327 |
icook.hk | 172.67.161.104 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare | | 118 |
104.16.45.84 | 104.16.45.84 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare | | 143 |
www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare | | 359 |
otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare
| | 419 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 457 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare
| | 212 | time.is | 172.67.68.157 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare | |
414 | www.ipchicken.com | 172.67.68.101 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare
| | 108 | japan.com | 172.67.70.92 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare | |
365 | trevor.ns.cloudflare.com | 4444.cloudflare.182682.xyz | IPv4 | h3 | ✅ 成功 | 60 |
cloudflare | | 254 | singapore.com | 104.26.13.140 | IPv4 | h3 | ✅ 成功 | 61 |
cloudflare | | 440 | asia.877774.xyz | 104.17.142.146 | IPv4 | h3 | ✅ 成功 | 61
| cloudflare | | 423 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 62
| cloudflare | | 31 | iplocation.io | 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 64 |
cloudflare | | 153 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h3 | ✅
成功 | 64 | cloudflare | | 348 | www.okcupid.com | 104.17.48.63 | IPv4 | h3 | ✅
成功 | 67 | cloudflare | | 467 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 |
✅ 成功 | 67 | cloudflare | | 14 | steamdb.info | 104.20.34.212 | IPv4 | h3 | ✅
成功 | 68 | cloudflare | | 32 | iplocation.io | 172.67.70.100 | IPv4 | h3 | ✅
成功 | 68 | cloudflare | | 413 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 |
✅ 成功 | 68 | cloudflare | | 95 | comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅
成功 | 69 | cloudflare | | 171 | cu.877774.xyz | 104.26.4.111 | IPv4 | h3 | ✅
成功 | 70 | cloudflare | | 253 | 104.17.68.85 | 104.17.68.85 | IPv4 | h3 | ✅
成功 | 70 | cloudflare | | 304 | julio.ns.cloudflare.com | 108.162.195.209 |
IPv4 | h3 | ✅ 成功 | 70 | cloudflare | | 459 | www.udemy.com | 104.16.142.237 |
IPv4 | h3 | ✅ 成功 | 70 | cloudflare | | 145 | ip.sb | 104.26.13.31 | IPv4 | h3
| ✅ 成功 | 71 | cloudflare | | 170 | cu.877774.xyz | 104.26.4.119 | IPv4 | h3 |
✅ 成功 | 71 | cloudflare | | 106 | japan.com | 104.26.4.60 | IPv4 | h3 | ✅
成功 | 72 | cloudflare | | 203 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 |
✅ 成功 | 72 | cloudflare | | 430 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功
| 73 | cloudflare | | 6 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅ 成功 | 74 |
cloudflare | | 135 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 74 |
cloudflare | | 241 | www.gov.ua | 104.21.23.72 | IPv4 | h3 | ✅ 成功 | 74 |
cloudflare | | 447 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 75 |
cloudflare | | 408 | dnschecker.org | 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 79 |
cloudflare | | 41 | ae8a9c24-83de.masx200.ddns-ip.net | 104.21.14.41 | IPv4 | h3
| ✅ 成功 | 81 | cloudflare | | 416 | www.pcmag.com | 104.16.20.118 | IPv4 | h3
| ✅ 成功 | 81 | cloudflare | | 164 | cu.877774.xyz | 104.26.4.113 | IPv4 | h3 |
✅ 成功 | 82 | cloudflare | | 427 | gamer.com.tw | 104.18.2.197 | IPv4 | h3 | ✅
成功 | 82 | cloudflare | | 349 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4
| h3 | ✅ 成功 | 83 | cloudflare | | 91 | yx-auto.pages.dev | 104.21.6.60 | IPv4
| h3 | ✅ 成功 | 84 | cloudflare | | 202 | cmcc.877774.xyz | 104.16.148.12 |
IPv4 | h3 | ✅ 成功 | 84 | cloudflare | | 303 | www.visa.com.hk | 104.18.21.69 |
IPv4 | h3 | ✅ 成功 | 84 | cloudflare | | 306 | julio.ns.cloudflare.com |
172.64.35.209 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare | | 87 | na.877774.xyz |
104.19.74.233 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare | | 163 | 103.160.204.59 |
103.160.204.59 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare | | 443 | cf.090227.xyz |
104.18.42.98 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare | | 270 |
craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare
| | 383 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 90 |
cloudflare | | 79 | cf.zhetengsha.eu.org | 172.64.145.158 | IPv4 | h3 | ✅ 成功
| 93 | cloudflare | | 191 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h3 | ✅ 成功
| 93 | cloudflare | | 229 | 104.16.223.179 | 104.16.223.179 | IPv4 | h3 | ✅
成功 | 93 | cloudflare | | 336 | wilson.ns.cloudflare.com | 108.162.195.110 |
IPv4 | h3 | ✅ 成功 | 93 | cloudflare | | 470 | cfip.xxxxxxxx.tk |
104.16.241.229 | IPv4 | h3 | ✅ 成功 | 93 | cloudflare | | 267 | shopify.com |
23.227.38.33 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare | | 58 | cf.0sm.com |
172.67.187.145 | IPv4 | h3 | ✅ 成功 | 96 | cloudflare | | 23 |
ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h3 | ✅ 成功 | 97 |
cloudflare | | 56 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 97 |
cloudflare | | 187 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 97
| cloudflare | | 82 | www.digitalocean.com | 104.19.173.68 | IPv4 | h3 | ✅ 成功
| 99 | cloudflare | | 206 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功
| 99 | cloudflare | | 25 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h3
| ✅ 成功 | 100 | cloudflare | | 159 | braden.ns.cloudflare.com | 172.64.35.169
| IPv4 | h3 | ✅ 成功 | 100 | cloudflare | | 245 | benedict.ns.cloudflare.com |
108.162.195.205 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare | | 392 | 104.18.254.88
| 104.18.254.88 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare | | 331 |
pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h3 | ✅ 成功 | 101 |
cloudflare | | 44 | ifconfig.co | 104.21.54.91 | IPv4 | h3 | ✅ 成功 | 102 |
cloudflare | | 128 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 102
| cloudflare | | 180 | 4444.cloudflare.182682.xyz | 172.67.152.183 | IPv4 | h3 | ✅ 成功 |
102 | cloudflare | | 207 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功
| 102 | cloudflare | | 387 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 |
h3 | ✅ 成功 | 102 | cloudflare | | 448 | cloudflare-ip.mofashi.ltd |
172.67.155.172 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare | | 261 | toy-people.com
| 104.26.2.36 | IPv4 | h3 | ✅ 成功 | 103 | cloudflare | | 330 |
pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h3 | ✅ 成功 | 103 |
cloudflare | | 165 | cu.877774.xyz | 104.26.4.114 | IPv4 | h3 | ✅ 成功 | 104 |
cloudflare | | 247 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅
成功 | 104 | cloudflare | | 71 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3
| ✅ 成功 | 105 | cloudflare | | 119 | bowen.ns.cloudflare.com | 108.162.195.83
| IPv4 | h3 | ✅ 成功 | 105 | cloudflare | | 142 | 172.67.243.218 |
172.67.243.218 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare | | 441 |
asia.877774.xyz | 104.16.211.153 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare | |
463 | zread.ai | 172.67.202.78 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare | | 284
| uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h3 | ✅ 成功 | 106 |
cloudflare | | 375 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功
| 106 | cloudflare | | 388 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 |
h3 | ✅ 成功 | 106 | cloudflare | | 444 | cf.090227.xyz | 172.64.145.158 | IPv4
| h3 | ✅ 成功 | 106 | cloudflare | | 471 | cfip.xxxxxxxx.tk | 104.27.21.118 |
IPv4 | h3 | ✅ 成功 | 106 | cloudflare | | 11 | www.wto.org | 104.18.41.190 |
IPv4 | h3 | ✅ 成功 | 107 | cloudflare | | 126 | ct.877774.xyz | 172.64.229.44 |
IPv4 | h3 | ✅ 成功 | 107 | cloudflare | | 260 | 172.67.75.172 | 172.67.75.172 |
IPv4 | h3 | ✅ 成功 | 107 | cloudflare | | 384 | 104.18.78.214 | 104.18.78.214 |
IPv4 | h3 | ✅ 成功 | 107 | cloudflare | | 439 | 172.64.229.249 | 172.64.229.249
| IPv4 | h3 | ✅ 成功 | 108 | cloudflare | | 3 | 172.64.40.9 | 172.64.40.9 |
IPv4 | h3 | ✅ 成功 | 109 | cloudflare | | 185 | cmcc.877774.xyz | 104.16.149.8
| IPv4 | h3 | ✅ 成功 | 109 | cloudflare | | 195 | cmcc.877774.xyz |
104.16.148.5 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare | | 380 | icook.tw |
104.20.28.74 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare | | 112 | 172.64.156.195 |
172.64.156.195 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare | | 115 |
yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare | |
344 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare
| | 350 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h3 | ✅ 成功 | 111 |
cloudflare | | 399 | 456.cloudflare.182682.xyz | 104.26.9.160 | IPv4 | h3 | ✅
成功 | 111 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 1 条记录
- **快 (50-100ms)**: 60 条记录
- **正常 (100-200ms)**: 39 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 7 次
- **IPv6 失败**: 171 次

### 按协议统计

- **none**: 178 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
