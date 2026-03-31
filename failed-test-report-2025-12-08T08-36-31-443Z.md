# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/8 16:36:31
- **数据来源**: connectivity_results.json
- **总测试数**: 486
- **失败测试数**: 184
- **成功测试数**: 302
- **失败率**: 37.86%
- **平均延迟**: 108.43ms
- **最小延迟**: 45ms
- **最大延迟**: 4832ms

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 173 次 (94.0%)
- **连接超时: I/O超时**: 9 次 (4.9%)
- **连接被拒绝: 通用连接拒绝**: 2 次 (1.1%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (173 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (9 次测试)

| 序号 | 主机/域名        | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ---------------- | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 363  | 119.194.220.146  | 119.194.220.146 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout |
| 375  | cfip.xxxxxxxx.tk | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 390  | ct.877774.xyz    | ct.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout   |
| 481  | 111.171.108.67   | 111.171.108.67  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout  |
| 482  | 175.212.207.13   | 175.212.207.13  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout  |
| 483  | 121.188.182.190  | 121.188.182.190 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout |
| 484  | 115.22.115.218   | 115.22.115.218  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 115.22.115.218:443: i/o timeout  |
| 485  | cf.877774.xyz    | cf.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout   |
| 486  | 52.76.110.129    | 52.76.110.129   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 52.76.110.129:443: i/o timeout   |

#### 连接被拒绝: 通用连接拒绝 (2 次测试)

| 序号 | 主机/域名       | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                  |
| ---- | --------------- | --------------- | ------ | ---- | ------ | -------- | ------ | --------------------------------------------------------- |
| 75   | 138.2.18.82     | 138.2.18.82     | IPv4   | none | N/A    | 0        | N/A    | dial tcp 138.2.18.82:443: connect: connection refused     |
| 82   | 222.105.131.225 | 222.105.131.225 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 222.105.131.225:443: connect: connection refused |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 173 次 (94.0%)
- **连接超时**: 9 次 (4.9%)
- **连接被拒绝**: 2 次 (1.1%)

#### 错误模式分析

**超时集中度分析**: 共有 9 次超时，主要集中在IP段 119.194（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 184 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 11 次，IPv6失败 173 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：bowen.ns.cloudflare.com (3次),
julio.ns.cloudflare.com (3次), 456.cloudflare.182682.xyz
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 147 |
104.16.223.179 | 104.16.223.179 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare | | 170
| 04c6cf21-1294-4fae-8bf8-715bbc897b60.masx200.netlib.re | 104.21.47.252 | IPv4
| h3 | ✅ 成功 | 45 | cloudflare | | 211 | ct.877774.xyz | 172.64.229.161 | IPv4
| h3 | ✅ 成功 | 45 | cloudflare | | 334 | 172.64.156.195 | 172.64.156.195 |
IPv4 | h3 | ✅ 成功 | 46 | cloudflare | | 369 | 104.17.68.85 | 104.17.68.85 |
IPv4 | h3 | ✅ 成功 | 46 | cloudflare | | 67 | 172.64.153.172 | 172.64.153.172 |
IPv4 | h3 | ✅ 成功 | 47 | cloudflare | | 14 | julio.ns.cloudflare.com |
172.64.35.209 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare | | 226 | 172.64.49.165 |
172.64.49.165 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare | | 371 | fbi.gov |
104.16.148.244 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare | | 443 |
yx-auto.pages.dev | 172.67.161.98 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare | |
339 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare | | 430
| www.whatismyip.com | 104.26.12.23 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare | |
87 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 338 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare | | 65
| saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare | | 74 |
172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare | | 90 |
www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare | | 250
| japan.com | 172.67.70.92 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare | | 261 |
eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare | | 265 |
time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare | | 308 |
cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare | | 150
| local-aria2-webui.masx200.ddns-ip.net | 104.21.14.41 | IPv4 | h3 | ✅ 成功 |
53 | cloudflare | | 326 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h3 | ✅ 成功 |
53 | cloudflare | | 335 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 | 53
| cloudflare | | 336 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 53 |
cloudflare | | 434 | freeyx.cloudflare88.eu.org | 141.101.121.254 | IPv4 | h3 |
✅ 成功 | 53 | cloudflare | | 59 | cf.877771.xyz | 172.67.152.183 | IPv4 | h3 |
✅ 成功 | 54 | cloudflare | | 366 | bestcf.030101.xyz | 104.19.152.12 | IPv4 |
h3 | ✅ 成功 | 54 | cloudflare | | 169 |
04c6cf21-1294-4fae-8bf8-715bbc897b60.masx200.netlib.re | 172.67.174.219 | IPv4 |
h3 | ✅ 成功 | 55 | cloudflare | | 213 | cris.ns.cloudflare.com |
108.162.195.202 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare | | 259 | 172.67.243.218
| 172.67.243.218 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare | | 310 |
cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare | | 327 |
cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare | | 343 |
toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare | | 407 |
wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare
| | 464 | cu.877774.xyz | 104.26.4.118 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 36 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare | |
70 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h3 | ✅ 成功 | 56 |
cloudflare | | 181 | asia.877774.xyz | 104.17.142.146 | IPv4 | h3 | ✅ 成功 | 56
| cloudflare | | 195 | www.visa.com.hk | 104.18.21.69 | IPv4 | h3 | ✅ 成功 | 56
| cloudflare | | 299 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功
| 56 | cloudflare | | 313 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功
| 56 | cloudflare | | 355 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅
成功 | 56 | cloudflare | | 356 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 |
✅ 成功 | 56 | cloudflare | | 412 | zread.ai | 172.67.202.78 | IPv4 | h3 | ✅
成功 | 56 | cloudflare | | 472 | cu.877774.xyz | 104.26.4.117 | IPv4 | h3 | ✅
成功 | 56 | cloudflare | | 115 | ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅
成功 | 57 | cloudflare | | 180 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 |
✅ 成功 | 57 | cloudflare | | 207 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 |
✅ 成功 | 57 | cloudflare | | 209 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 |
✅ 成功 | 57 | cloudflare | | 241 | comicabc.com | 172.67.174.21 | IPv4 | h3 |
✅ 成功 | 57 | cloudflare | | 295 | 162.159.36.104 | 162.159.36.104 | IPv4 | h3
| ✅ 成功 | 57 | cloudflare | | 447 | huxley.ns.cloudflare.com | 108.162.195.188
| IPv4 | h3 | ✅ 成功 | 57 | cloudflare | | 471 | cu.877774.xyz | 104.26.4.116 |
IPv4 | h3 | ✅ 成功 | 57 | cloudflare | | 66 | saas.sin.fan | 162.159.36.20 |
IPv4 | h3 | ✅ 成功 | 58 | cloudflare | | 85 | www.okcupid.com | 104.17.48.63 |
IPv4 | h3 | ✅ 成功 | 58 | cloudflare | | 93 | dnschecker.org | 104.26.6.89 |
IPv4 | h3 | ✅ 成功 | 58 | cloudflare | | 153 | www.visa.cn | 162.159.153.2 |
IPv4 | h3 | ✅ 成功 | 58 | cloudflare | | 260 | eur.877774.xyz | 104.21.26.150 |
IPv4 | h3 | ✅ 成功 | 58 | cloudflare | | 317 | cmcc.877774.xyz | 104.16.149.9 |
IPv4 | h3 | ✅ 成功 | 58 | cloudflare | | 330 | cmcc.877774.xyz | 104.16.148.9 |
IPv4 | h3 | ✅ 成功 | 58 | cloudflare | | 39 | braden.ns.cloudflare.com |
108.162.195.169 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare | | 291 | steamdb.info |
172.66.175.250 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare | | 294 | 172.64.35.24 |
172.64.35.24 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare | | 309 | cmcc.877774.xyz |
104.16.149.1 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare | | 88 | www.okcupid.com |
104.16.239.254 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare | | 131 |
craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare
| | 166 | 172.64.147.73 | 172.64.147.73 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare
| | 275 | 172.67.79.211 | 172.67.79.211 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare
| | 168 | 104.19.223.58 | 104.19.223.58 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare
| | 227 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare
| | 231 | 172.64.159.6 | 172.64.159.6 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 466 | cu.877774.xyz | 104.26.4.111 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare | |
470 | cu.877774.xyz | 104.26.4.115 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare | | 8
| bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare
| | 107 | 104.18.254.88 | 104.18.254.88 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare
| | 348 | shopify.com | 23.227.38.33 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare | |
353 | www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare | |
382 | www.digitalocean.com | 104.19.173.68 | IPv4 | h3 | ✅ 成功 | 62 |
cloudflare | | 154 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 63 |
cloudflare | | 284 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 63
| cloudflare | | 340 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 63 |
cloudflare | | 458 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h3 | ✅
成功 | 63 | cloudflare | | 50 |
72806a5a-a251-48b4-a523-dfbd1c981ec0.ce225219-fea4-47a0-bb82-70b612b27ab7.netlib.re
| 172.67.212.197 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare | | 62 | na.877774.xyz
| 104.19.74.233 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare | | 232 | singapore.com
| 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare | | 276 | ipinfo.in |
104.21.21.129 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare | | 304 | www.ipget.net |
172.67.207.26 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare | | 314 | cmcc.877774.xyz
| 104.16.149.6 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare | | 439 |
moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare
| | 5 | 172.67.106.26 | 172.67.106.26 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 175 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare | |
256 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare
| | 324 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare
| | 333 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 65 |
cloudflare | | 386 | ae8a9c24-83de.masx200.ddns-ip.net | 104.21.14.41 | IPv4 |
h3 | ✅ 成功 | 65 | cloudflare | | 6 | bowen.ns.cloudflare.com | 108.162.195.83
| IPv4 | h3 | ✅ 成功 | 66 | cloudflare | | 162 | www.udemy.com | 104.16.142.237
| IPv4 | h3 | ✅ 成功 | 66 | cloudflare | | 200 | damien.ns.cloudflare.com |
172.64.35.168 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare | | 318 | cmcc.877774.xyz
| 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 12 条记录
- **快 (50-100ms)**: 88 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 11 次
- **IPv6 失败**: 173 次

### 按协议统计

- **none**: 184 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
