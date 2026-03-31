# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 03:47:52
- **数据来源**: connectivity_results-20251212-034752.json
- **总测试数**: 471
- **失败测试数**: 177
- **成功测试数**: 294
- **失败率**: 37.58%
- **平均延迟**: 118.26ms
- **最小延迟**: 49ms
- **最大延迟**: 728ms

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 173 次 (97.7%)
- **连接超时: I/O超时**: 4 次 (2.3%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (173 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (4 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 456  | cf.877774.xyz    | cf.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout  |
| 457  | ct.877774.xyz    | ct.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout  |
| 458  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 465  | cfip.xxxxxxxx.tk | 104.20.255.53  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.20.255.53:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 173 次 (97.7%)
- **连接超时**: 4 次 (2.3%)

#### 错误模式分析

**超时集中度分析**: 共有 4 次超时，主要集中在IP段 172.67（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 177 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 4 次，IPv6失败 173 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：www.whatismyip.com (3次),
trevor.ns.cloudflare.com (3次), wilson.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 430 |
abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 | ✅ 成功 | 49 |
cloudflare | | 325 | dnschecker.org | 104.26.7.89 | IPv4 | h3 | ✅ 成功 | 50 |
cloudflare | | 161 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅ 成功 | 51 |
cloudflare | | 309 | 456.cloudflare.182682.xyz | 104.26.9.160 | IPv4 | h3 | ✅
成功 | 55 | cloudflare | | 444 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 |
✅ 成功 | 56 | cloudflare | | 468 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 |
h3 | ✅ 成功 | 58 | cloudflare | | 109 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3
| ✅ 成功 | 59 | cloudflare | | 169 | cmcc.877774.xyz | 104.16.148.11 | IPv4 |
h3 | ✅ 成功 | 64 | cloudflare | | 142 | na.877774.xyz | 104.18.187.25 | IPv4 |
h3 | ✅ 成功 | 65 | cloudflare | | 375 | www.udemy.com | 104.16.142.237 | IPv4 |
h3 | ✅ 成功 | 65 | cloudflare | | 340 | julio.ns.cloudflare.com |
108.162.195.209 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare | | 26 |
trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare
| | 83 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare
| | 321 | cf.090227.xyz | 172.64.144.82 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare
| | 389 | www.digitalocean.com | 104.19.174.68 | IPv4 | h3 | ✅ 成功 | 70 |
cloudflare | | 182 | cf.877771.xyz | 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 71 |
cloudflare | | 432 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h3 | ✅
成功 | 71 | cloudflare | | 168 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 |
✅ 成功 | 72 | cloudflare | | 371 | yx-auto.pages.dev | 172.67.161.98 | IPv4 |
h3 | ✅ 成功 | 72 | cloudflare | | 44 | www.gov.ua | 104.21.23.72 | IPv4 | h3 |
✅ 成功 | 73 | cloudflare | | 285 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3
| ✅ 成功 | 73 | cloudflare | | 393 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3
| ✅ 成功 | 74 | cloudflare | | 16 | www.whatismyip.com | 172.67.69.129 | IPv4 |
h3 | ✅ 成功 | 77 | cloudflare | | 124 | cu.877774.xyz | 104.26.4.117 | IPv4 |
h3 | ✅ 成功 | 77 | cloudflare | | 330 | ashton.ns.cloudflare.com |
108.162.195.173 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare | | 349 |
ae8a9c24-83de.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h3 | ✅ 成功 | 77 |
cloudflare | | 436 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h3 | ✅ 成功 |
77 | cloudflare | | 3 | comicabc.com | 172.67.174.21 | IPv4 | h3 | ✅ 成功 | 78
| cloudflare | | 77 | cf.0sm.com | 104.21.7.133 | IPv4 | h3 | ✅ 成功 | 78 |
cloudflare | | 156 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 78
| cloudflare | | 354 | 104.18.14.76 | 104.18.14.76 | IPv4 | h3 | ✅ 成功 | 78 |
cloudflare | | 172 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h3 | ✅ 成功 | 79 |
cloudflare | | 175 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功 | 79 |
cloudflare | | 203 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 79 |
cloudflare | | 232 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h3 | ✅ 成功 |
79 | cloudflare | | 298 | 162.159.36.104 | 162.159.36.104 | IPv4 | h3 | ✅ 成功
| 79 | cloudflare | | 129 | yx-auto.pages.dev | 104.21.6.60 | IPv4 | h3 | ✅
成功 | 80 | cloudflare | | 252 | time.is | 172.67.68.157 | IPv4 | h3 | ✅ 成功 |
80 | cloudflare | | 279 | ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 80 |
cloudflare | | 464 | cfip.xxxxxxxx.tk | 104.18.228.35 | IPv4 | h3 | ✅ 成功 | 80
| cloudflare | | 74 | icook.hk | 172.67.161.104 | IPv4 | h3 | ✅ 成功 | 81 |
cloudflare | | 163 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h3 | ✅ 成功 | 81 |
cloudflare | | 461 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h3 | ✅ 成功 |
81 | cloudflare | | 395 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 82
| cloudflare | | 408 | 104.26.13.31 | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 82 |
cloudflare | | 448 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 82 |
cloudflare | | 460 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h3 | ✅ 成功 |
82 | cloudflare | | 32 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 |
✅ 成功 | 83 | cloudflare | | 40 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅
成功 | 83 | cloudflare | | 217 | asia.877774.xyz | 104.16.211.153 | IPv4 | h3 |
✅ 成功 | 83 | cloudflare | | 399 | otto.ns.cloudflare.com | 172.64.35.135 |
IPv4 | h3 | ✅ 成功 | 83 | cloudflare | | 423 | 104.19.223.58 | 104.19.223.58 |
IPv4 | h3 | ✅ 成功 | 83 | cloudflare | | 64 | 103.160.204.59 | 103.160.204.59 |
IPv4 | h3 | ✅ 成功 | 84 | cloudflare | | 278 | ip.sb | 104.26.12.31 | IPv4 | h3
| ✅ 成功 | 84 | cloudflare | | 348 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 |
✅ 成功 | 84 | cloudflare | | 446 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3
| ✅ 成功 | 84 | cloudflare | | 130 | yx-auto.pages.dev | 172.67.134.139 | IPv4
| h3 | ✅ 成功 | 85 | cloudflare | | 197 | dylan.ns.cloudflare.com |
108.162.195.187 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare | | 6 | www.ipget.net |
172.67.207.26 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare | | 104 | www.okcupid.com
| 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare | | 180 |
cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare | | 253 |
time.is | 104.26.12.54 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare | | 316 |
whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare |
| 379 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare | | 2
| comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare | | 98 |
sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h3 | ✅ 成功 | 88 |
cloudflare | | 126 | cu.877774.xyz | 104.26.4.119 | IPv4 | h3 | ✅ 成功 | 88 |
cloudflare | | 310 | 456.cloudflare.182682.xyz | 104.26.8.160 | IPv4 | h3 | ✅
成功 | 88 | cloudflare | | 396 | 172.64.35.24 | 172.64.35.24 | IPv4 | h3 | ✅
成功 | 88 | cloudflare | | 15 | www.whatismyip.com | 104.26.12.23 | IPv4 | h3 |
✅ 成功 | 89 | cloudflare | | 240 | bowen.ns.cloudflare.com | 108.162.195.83 |
IPv4 | h3 | ✅ 成功 | 89 | cloudflare | | 305 | ip.gs | 104.21.14.176 | IPv4 |
h3 | ✅ 成功 | 89 | cloudflare | | 66 | 104.18.254.88 | 104.18.254.88 | IPv4 |
h3 | ✅ 成功 | 90 | cloudflare | | 155 | cmcc.877774.xyz | 104.16.149.10 | IPv4
| h3 | ✅ 成功 | 90 | cloudflare | | 276 | saas.sin.fan | 162.159.36.20 | IPv4 |
h3 | ✅ 成功 | 90 | cloudflare | | 140 | na.877774.xyz | 104.19.74.233 | IPv4 |
h3 | ✅ 成功 | 91 | cloudflare | | 219 | asia.877774.xyz | 104.17.142.146 | IPv4
| h3 | ✅ 成功 | 91 | cloudflare | | 246 | moura.ns.cloudflare.com |
108.162.195.217 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare | | 326 | dnschecker.org
| 172.67.73.216 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare | | 364 |
yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 | 91 | cloudflare | |
390 | www.digitalocean.com | 104.19.173.68 | IPv4 | h3 | ✅ 成功 | 91 |
cloudflare | | 7 | www.ipget.net | 104.21.15.212 | IPv4 | h3 | ✅ 成功 | 92 |
cloudflare | | 61 | 104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 92 |
cloudflare | | 81 | www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 92 |
cloudflare | | 94 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 |
92 | cloudflare | | 121 | cu.877774.xyz | 104.26.4.114 | IPv4 | h3 | ✅ 成功 |
92 | cloudflare | | 280 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 92 |
cloudflare | | 69 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅
成功 | 93 | cloudflare | | 185 | kyree.ns.cloudflare.com | 108.162.195.207 |
IPv4 | h3 | ✅ 成功 | 93 | cloudflare | | 394 | 172.67.79.211 | 172.67.79.211 |
IPv4 | h3 | ✅ 成功 | 93 | cloudflare | | 85 | ct.877774.xyz | 172.64.229.217 |
IPv4 | h3 | ✅ 成功 | 94 | cloudflare | | 141 | na.877774.xyz | 104.18.38.235 |
IPv4 | h3 | ✅ 成功 | 94 | cloudflare | | 336 |
local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h3 | ✅ 成功 |
94 | cloudflare | | 452 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅
成功 | 94 | cloudflare | | 417 | lewis.ns.cloudflare.com | 108.162.195.159 |
IPv4 | h3 | ✅ 成功 | 95 | cloudflare | | 51 | ipv4.ip.sb | 104.26.13.31 | IPv4
| h3 | ✅ 成功 | 96 | cloudflare | | 207 | bestcf.030101.xyz | 104.16.149.55 |
IPv4 | h3 | ✅ 成功 | 96 | cloudflare | | 380 | icook.tw | 104.20.28.74 | IPv4 |
h3 | ✅ 成功 | 96 | cloudflare | | 404 | ifconfig.co | 172.67.168.106 | IPv4 |
h3 | ✅ 成功 | 96 | cloudflare | | 419 | lewis.ns.cloudflare.com | 172.64.35.159
| IPv4 | h3 | ✅ 成功 | 96 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 1 条记录
- **快 (50-100ms)**: 99 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 4 次
- **IPv6 失败**: 173 次

### 按协议统计

- **none**: 177 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
