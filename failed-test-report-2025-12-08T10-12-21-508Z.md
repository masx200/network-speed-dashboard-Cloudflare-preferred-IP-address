# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/8 10:12:21
- **数据来源**: connectivity_results.json
- **总测试数**: 495
- **失败测试数**: 183
- **成功测试数**: 312
- **失败率**: 36.97%
- **平均延迟**: 72.82ms
- **最小延迟**: 33ms
- **最大延迟**: 883ms

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 174 次 (95.1%)
- **连接超时: I/O超时**: 9 次 (4.9%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (174 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (9 次测试)

| 序号 | 主机/域名        | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ---------------- | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 447  | 119.194.220.146  | 119.194.220.146 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout |
| 455  | cfip.xxxxxxxx.tk | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 461  | 121.188.182.190  | 121.188.182.190 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout |
| 481  | 115.22.115.218   | 115.22.115.218  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 115.22.115.218:443: i/o timeout  |
| 482  | ct.877774.xyz    | ct.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout   |
| 488  | 111.171.108.67   | 111.171.108.67  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout  |
| 491  | cf.877774.xyz    | cf.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout   |
| 494  | 175.212.207.13   | 175.212.207.13  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout  |
| 495  | 52.76.110.129    | 52.76.110.129   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 52.76.110.129:443: i/o timeout   |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 174 次 (95.1%)
- **连接超时**: 9 次 (4.9%)

#### 错误模式分析

**超时集中度分析**: 共有 9 次超时，主要集中在IP段 119.194（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 183 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 9 次，IPv6失败 174 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：singapore.com (3次),
uriah.ns.cloudflare.com (3次), cris.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 422 |
yx-auto.pages.dev | 104.21.9.230 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare | | 292
| otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare
| | 42 | www.wto.org | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare | |
166 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare | |
403 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare | |
425 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 472 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare
| | 82 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare
| | 247 | iplocation.io | 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare
| | 358 | 172.67.79.211 | 172.67.79.211 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare
| | 421 | yx-auto.pages.dev | 172.67.161.98 | IPv4 | h3 | ✅ 成功 | 37 |
cloudflare | | 140 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 | 38 |
cloudflare | | 143 | cf.090227.xyz | 172.66.44.77 | IPv4 | h3 | ✅ 成功 | 38 |
cloudflare | | 163 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功 | 38 |
cloudflare | | 267 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 38
| cloudflare | | 283 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h3 |
✅ 成功 | 38 | cloudflare | | 480 | www.7749tv.com | 104.19.133.4 | IPv4 | h3 |
✅ 成功 | 38 | cloudflare | | 208 | huxley.ns.cloudflare.com | 172.64.35.188 |
IPv4 | h3 | ✅ 成功 | 39 | cloudflare | | 264 | www.okcupid.com | 104.16.144.63
| IPv4 | h3 | ✅ 成功 | 39 | cloudflare | | 101 | comicabc.com | 172.67.174.21 |
IPv4 | h3 | ✅ 成功 | 40 | cloudflare | | 177 |
ae8a9c24-83de.masx200.ddns-ip.net | 104.21.14.41 | IPv4 | h3 | ✅ 成功 | 40 |
cloudflare | | 373 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功
| 40 | cloudflare | | 401 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h3 | ✅ 成功
| 40 | cloudflare | | 463 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 40
| cloudflare | | 468 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅ 成功 | 40 |
cloudflare | | 243 | 172.64.159.6 | 172.64.159.6 | IPv4 | h3 | ✅ 成功 | 41 |
cloudflare | | 365 | 172.64.35.24 | 172.64.35.24 | IPv4 | h3 | ✅ 成功 | 41 |
cloudflare | | 117 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 42
| cloudflare | | 256 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 |
✅ 成功 | 42 | cloudflare | | 346 | 172.64.33.67 | 172.64.33.67 | IPv4 | h3 | ✅
成功 | 42 | cloudflare | | 448 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 |
✅ 成功 | 42 | cloudflare | | 63 | www.udemy.com | 104.16.142.237 | IPv4 | h3 |
✅ 成功 | 43 | cloudflare | | 99 | www.4444.cloudflare.182682.xyz | 162.159.153.2 | IPv4 | h3 | ✅
成功 | 43 | cloudflare | | 148 | sullivan.ns.cloudflare.com | 172.64.35.161 |
IPv4 | h3 | ✅ 成功 | 43 | cloudflare | | 268 | trevor.ns.cloudflare.com |
4444.cloudflare.182682.xyz | IPv4 | h3 | ✅ 成功 | 43 | cloudflare | | 477 |
04c6cf21-1294-4fae-8bf8-715bbc897b60.masx200.netlib.re | 104.21.47.252 | IPv4 |
h3 | ✅ 成功 | 43 | cloudflare | | 426 | www.glassdoor.com | 104.17.64.70 | IPv4
| h3 | ✅ 成功 | 44 | cloudflare | | 427 | local-aria2-webui.masx200.ddns-ip.net
| 104.21.14.41 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare | | 266 | www.okcupid.com
| 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare | | 431 | ipv4.ip.sb |
104.26.12.31 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare | | 456 | cfip.xxxxxxxx.tk
| 104.16.241.229 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare | | 86 | 4444.cloudflare.182682.xyz
| 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare | | 142 | na.877774.xyz
| 104.18.38.235 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare | | 332 |
72806a5a-a251-48b4-a523-dfbd1c981ec0.ce225219-fea4-47a0-bb82-70b612b27ab7.netlib.re
| 104.21.61.182 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare | | 350 |
pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h3 | ✅ 成功 | 46 |
cloudflare | | 356 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 46 |
cloudflare | | 397 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 46
| cloudflare | | 415 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h3 | ✅ 成功 |
46 | cloudflare | | 13 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅ 成功 |
47 | cloudflare | | 37 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 |
47 | cloudflare | | 319 | time.is | 172.67.68.157 | IPv4 | h3 | ✅ 成功 | 47 |
cloudflare | | 336 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 47 |
cloudflare | | 68 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅ 成功 | 48
| cloudflare | | 94 | cu.877774.xyz | 104.26.4.118 | IPv4 | h3 | ✅ 成功 | 48 |
cloudflare | | 95 | cu.877774.xyz | 104.26.4.119 | IPv4 | h3 | ✅ 成功 | 48 |
cloudflare | | 366 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅
成功 | 48 | cloudflare | | 402 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅
成功 | 48 | cloudflare | | 118 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 |
✅ 成功 | 49 | cloudflare | | 129 | stock.hostmonit.com | 172.67.187.251 | IPv4
| h3 | ✅ 成功 | 49 | cloudflare | | 139 | cfip.1323123.xyz | 104.16.133.220 |
IPv4 | h3 | ✅ 成功 | 49 | cloudflare | | 154 | 172.64.147.73 | 172.64.147.73 |
IPv4 | h3 | ✅ 成功 | 49 | cloudflare | | 232 | 172.67.243.218 | 172.67.243.218
| IPv4 | h3 | ✅ 成功 | 49 | cloudflare | | 263 | www.okcupid.com |
104.18.160.63 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare | | 434 |
bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare
| | 89 | cu.877774.xyz | 104.26.4.113 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 181 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 192 | www.hugedomains.com | 104.26.6.37 | IPv4 | h3 | ✅ 成功 | 50 |
cloudflare | | 278 | ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅ 成功 | 50 |
cloudflare | | 314 | www.digitalocean.com | 104.19.174.68 | IPv4 | h3 | ✅ 成功
| 50 | cloudflare | | 404 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功
| 50 | cloudflare | | 5 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 51 |
cloudflare | | 9 | fbi.gov | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 51 |
cloudflare | | 71 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅ 成功 | 51 |
cloudflare | | 144 | cf.090227.xyz | 172.66.47.179 | IPv4 | h3 | ✅ 成功 | 51 |
cloudflare | | 239 | www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 |
51 | cloudflare | | 464 | zread.ai | 172.67.202.78 | IPv4 | h3 | ✅ 成功 | 51 |
cloudflare | | 8 | 172.64.229.249 | 172.64.229.249 | IPv4 | h3 | ✅ 成功 | 52 |
cloudflare | | 74 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 52 |
cloudflare | | 91 | cu.877774.xyz | 104.26.4.115 | IPv4 | h3 | ✅ 成功 | 52 |
cloudflare | | 96 | cu.877774.xyz | 104.26.4.111 | IPv4 | h3 | ✅ 成功 | 52 |
cloudflare | | 120 | asia.877774.xyz | 104.17.142.146 | IPv4 | h3 | ✅ 成功 | 52
| cloudflare | | 178 | ae8a9c24-83de.masx200.ddns-ip.net | 172.67.157.182 | IPv4
| h3 | ✅ 成功 | 52 | cloudflare | | 182 | craig.ns.cloudflare.com |
108.162.195.192 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare | | 190 |
www.hugedomains.com | 104.26.7.37 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare | |
203 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare | |
215 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h3 | ✅ 成功 | 52 |
cloudflare | | 251 | cf.zhetengsha.eu.org | 172.66.47.179 | IPv4 | h3 | ✅ 成功
| 52 | cloudflare | | 385 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅
成功 | 52 | cloudflare | | 451 | cfip.xxxxxxxx.tk | 190.93.246.67 | IPv4 | h3 |
✅ 成功 | 52 | cloudflare | | 21 | uriah.ns.cloudflare.com | 172.64.35.194 |
IPv4 | h3 | ✅ 成功 | 53 | cloudflare | | 38 | www.visa.com.hk | 104.18.21.69 |
IPv4 | h3 | ✅ 成功 | 53 | cloudflare | | 90 | cu.877774.xyz | 104.26.4.114 |
IPv4 | h3 | ✅ 成功 | 53 | cloudflare | | 233 | cf.877774.xyz | 104.18.41.190 |
IPv4 | h3 | ✅ 成功 | 53 | cloudflare | | 238 | www.whatismyip.com |
104.26.13.23 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare | | 246 | iplocation.io |
104.26.11.222 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare | | 294 |
otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare
| | 318 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare | | 383
| cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare | | 454
| cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare | |
46 | dnschecker.org | 104.26.7.89 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 64 条记录
- **快 (50-100ms)**: 36 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 9 次
- **IPv6 失败**: 174 次

### 按协议统计

- **none**: 183 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
