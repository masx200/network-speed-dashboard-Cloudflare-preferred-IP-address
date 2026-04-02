# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/11 03:42:21
- **数据来源**: connectivity_results-20251211-034221.json
- **总测试数**: 472
- **失败测试数**: 174
- **成功测试数**: 298
- **失败率**: 36.86%
- **平均延迟**: 85.45ms
- **最小延迟**: 36ms
- **最大延迟**: 778ms

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 171 次 (98.3%)
- **连接超时: I/O超时**: 3 次 (1.7%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (171 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 470  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 471  | cf.877774.xyz    | cf.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout  |
| 472  | ct.877774.xyz    | ct.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 171 次 (98.3%)
- **连接超时**: 3 次 (1.7%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 174 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 171 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：dnschecker.org (3次),
ashton.ns.cloudflare.com (3次), 456.cloudflare.182682.xyz
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 357 |
cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare
| | 138 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare
| | 263 | cu.877774.xyz | 104.26.4.118 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 291 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 40 | cloudflare |
| 224 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare | | 225 |
ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare | | 251 | time.is |
172.67.68.157 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare | | 377 |
moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 41 | cloudflare
| | 106 | steamdb.info | 104.20.34.212 | IPv4 | h3 | ✅ 成功 | 42 | cloudflare |
| 231 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h3 | ✅ 成功 | 43 |
cloudflare | | 352 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅
成功 | 43 | cloudflare | | 434 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4
| h3 | ✅ 成功 | 43 | cloudflare | | 447 | yx-auto.pages.dev | 104.21.9.230 |
IPv4 | h3 | ✅ 成功 | 43 | cloudflare | | 220 | 4444.cloudflare.182682.xyz | 104.21.80.180 |
IPv4 | h3 | ✅ 成功 | 44 | cloudflare | | 289 | cmcc.877774.xyz | 104.16.148.4 |
IPv4 | h3 | ✅ 成功 | 44 | cloudflare | | 356 | 172.64.159.6 | 172.64.159.6 |
IPv4 | h3 | ✅ 成功 | 44 | cloudflare | | 214 | www.ipget.net | 104.21.15.212 |
IPv4 | h3 | ✅ 成功 | 45 | cloudflare | | 286 | cmcc.877774.xyz | 104.16.148.1 |
IPv4 | h3 | ✅ 成功 | 45 | cloudflare | | 294 | cmcc.877774.xyz | 104.16.148.9 |
IPv4 | h3 | ✅ 成功 | 45 | cloudflare | | 421 | 172.67.106.26 | 172.67.106.26 |
IPv4 | h3 | ✅ 成功 | 45 | cloudflare | | 273 | fbi.gov | 104.16.149.244 | IPv4
| h3 | ✅ 成功 | 46 | cloudflare | | 293 | cmcc.877774.xyz | 104.16.148.8 | IPv4
| h3 | ✅ 成功 | 46 | cloudflare | | 366 | uriah.ns.cloudflare.com |
172.64.35.194 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare | | 218 | www.csgo.com |
195.85.59.161 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare | | 95 | asia.877774.xyz |
104.17.139.62 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare | | 97 |
cf.zhetengsha.eu.org | 172.64.152.241 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare |
| 307 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 318 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅ 成功 | 50 |
cloudflare | | 432 | www.okcupid.com | 104.18.160.63 | IPv4 | h3 | ✅ 成功 | 50
| cloudflare | | 198 | japan.com | 104.26.5.60 | IPv4 | h3 | ✅ 成功 | 51 |
cloudflare | | 327 | 104.17.68.85 | 104.17.68.85 | IPv4 | h3 | ✅ 成功 | 51 |
cloudflare | | 33 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h3 | ✅
成功 | 52 | cloudflare | | 249 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅
成功 | 52 | cloudflare | | 308 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 |
✅ 成功 | 52 | cloudflare | | 309 | sullivan.ns.cloudflare.com | 108.162.195.161
| IPv4 | h3 | ✅ 成功 | 52 | cloudflare | | 359 | cris.ns.cloudflare.com |
172.64.35.202 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare | | 422 | www.4444.cloudflare.182682.xyz |
162.159.153.2 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare | | 132 | 104.19.223.58 |
104.19.223.58 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare | | 261 | cu.877774.xyz |
104.26.4.116 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare | | 305 | cmcc.877774.xyz |
104.16.149.7 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare | | 196 | japan.com |
172.67.70.92 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare | | 226 | ip.sb |
104.26.13.31 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare | | 92 | 104.19.175.123 |
104.19.175.123 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare | | 142 | na.877774.xyz |
104.18.38.235 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare | | 162 |
huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 56 |
cloudflare | | 16 | bestcf.030101.xyz | 104.16.156.236 | IPv4 | h3 | ✅ 成功 |
57 | cloudflare | | 146 | ifconfig.co | 104.21.54.91 | IPv4 | h3 | ✅ 成功 | 57
| cloudflare | | 157 | ae8a9c24-83de.masx200.ddns-ip.net | 172.67.157.182 | IPv4
| h3 | ✅ 成功 | 57 | cloudflare | | 287 | cmcc.877774.xyz | 104.16.148.2 | IPv4
| h3 | ✅ 成功 | 57 | cloudflare | | 301 | cmcc.877774.xyz | 104.16.149.3 | IPv4
| h3 | ✅ 成功 | 57 | cloudflare | | 460 | damien.ns.cloudflare.com |
108.162.195.168 | IPv4 | h3 | ✅ 成功 | 58 | cloudflare | | 9 | www.wto.org |
104.18.41.190 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare | | 414 | 104.26.13.31 |
104.26.13.31 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare | | 128 | cf.0sm.com |
172.67.187.145 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare | | 391 | 172.67.110.232
| 172.67.110.232 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare | | 7 |
cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare | |
331 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare | |
136 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 63 | cloudflare | |
20 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare | |
164 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅ 成功 | 64 |
cloudflare | | 184 | www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 |
64 | cloudflare | | 371 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功
| 64 | cloudflare | | 430 | www.okcupid.com | 104.17.48.63 | IPv4 | h3 | ✅ 成功
| 64 | cloudflare | | 30 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅
成功 | 65 | cloudflare | | 54 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅
成功 | 65 | cloudflare | | 243 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h3 |
✅ 成功 | 65 | cloudflare | | 288 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 |
✅ 成功 | 65 | cloudflare | | 292 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 |
✅ 成功 | 65 | cloudflare | | 296 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3
| ✅ 成功 | 65 | cloudflare | | 8 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅
成功 | 66 | cloudflare | | 23 | dnschecker.org | 104.26.7.89 | IPv4 | h3 | ✅
成功 | 66 | cloudflare | | 202 | 104.16.61.163 | 104.16.61.163 | IPv4 | h3 | ✅
成功 | 66 | cloudflare | | 31 | www.ipchicken.com | 172.67.68.101 | IPv4 | h3 |
✅ 成功 | 67 | cloudflare | | 170 | www.digitalocean.com | 104.19.173.68 | IPv4
| h3 | ✅ 成功 | 67 | cloudflare | | 204 | yx-auto.pages.dev | 172.66.44.144 |
IPv4 | h3 | ✅ 成功 | 67 | cloudflare | | 278 | 104.16.223.179 | 104.16.223.179
| IPv4 | h3 | ✅ 成功 | 67 | cloudflare | | 299 | cmcc.877774.xyz | 104.16.149.1
| IPv4 | h3 | ✅ 成功 | 67 | cloudflare | | 336 | shopify.com | 23.227.38.33 |
IPv4 | h3 | ✅ 成功 | 67 | cloudflare | | 410 | pranab.ns.cloudflare.com |
172.64.35.199 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare | | 101 | 198.62.62.4 |
198.62.62.4 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare | | 168 | www.7749tv.com |
172.67.101.77 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare | | 169 |
www.digitalocean.com | 104.19.174.68 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare | |
279 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare | | 337
| toy-people.com | 104.26.2.36 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare | | 436 |
otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅ 成功 | 68 | cloudflare |
| 462 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅ 成功 | 68 |
cloudflare | | 19 | www.pcmag.com | 104.16.21.118 | IPv4 | h3 | ✅ 成功 | 69 |
cloudflare | | 137 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 69 |
cloudflare | | 143 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 | 69 |
cloudflare | | 145 | ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅ 成功 | 69 |
cloudflare | | 179 | www.hugedomains.com | 104.26.7.37 | IPv4 | h3 | ✅ 成功 |
69 | cloudflare | | 260 | cu.877774.xyz | 104.26.4.115 | IPv4 | h3 | ✅ 成功 |
69 | cloudflare | | 303 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 |
69 | cloudflare | | 311 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h3
| ✅ 成功 | 69 | cloudflare | | 50 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 |
✅ 成功 | 70 | cloudflare | | 93 | asia.877774.xyz | 104.16.211.153 | IPv4 | h3
| ✅ 成功 | 70 | cloudflare | | 94 | asia.877774.xyz | 104.17.142.146 | IPv4 |
h3 | ✅ 成功 | 70 | cloudflare | | 140 | ct.877774.xyz | 172.64.229.185 | IPv4 |
h3 | ✅ 成功 | 70 | cloudflare | | 213 | 172.67.243.218 | 172.67.243.218 | IPv4
| h3 | ✅ 成功 | 70 | cloudflare | | 256 | cu.877774.xyz | 104.26.4.111 | IPv4 |
h3 | ✅ 成功 | 70 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 26 条记录
- **快 (50-100ms)**: 74 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 171 次

### 按协议统计

- **none**: 174 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
