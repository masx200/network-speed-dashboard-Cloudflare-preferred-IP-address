# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/8 09:50:45
- **数据来源**: connectivity_results.json
- **总测试数**: 500
- **失败测试数**: 188
- **成功测试数**: 312
- **失败率**: 37.60%
- **平均延迟**: 106.82ms
- **最小延迟**: 67ms
- **最大延迟**: 546ms

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 175 次 (93.1%)
- **连接超时: I/O超时**: 10 次 (5.3%)
- **连接被拒绝: 通用连接拒绝**: 2 次 (1.1%)
- **DNS解析错误: 其他DNS错误**: 1 次 (0.5%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (175 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
| ---- | --------- | ------ | ------ | ---- | ------ | -------- | ------ | -------- |

#### 连接超时: I/O超时 (10 次测试)

| 序号 | 主机/域名        | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ---------------- | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 379  | ct.877774.xyz    | ct.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout   |
| 444  | 111.171.108.67   | 111.171.108.67  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout  |
| 480  | 175.212.207.13   | 175.212.207.13  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout  |
| 481  | 121.188.182.190  | 121.188.182.190 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout |
| 482  | 119.194.220.146  | 119.194.220.146 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout |
| 483  | cf.877774.xyz    | cf.877774.xyz   | IPv4   | none | N/A    | 0        | N/A    | dial tcp cf.877774.xyz:443: i/o timeout   |
| 484  | 115.22.115.218   | 115.22.115.218  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 115.22.115.218:443: i/o timeout  |
| 485  | 52.76.110.129    | 52.76.110.129   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 52.76.110.129:443: i/o timeout   |
| 486  | cfip.xxxxxxxx.tk | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 498  | cfip.xxxxxxxx.tk | 104.20.255.53   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.20.255.53:443: i/o timeout   |

#### 连接被拒绝: 通用连接拒绝 (2 次测试)

| 序号 | 主机/域名       | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                  |
| ---- | --------------- | --------------- | ------ | ---- | ------ | -------- | ------ | --------------------------------------------------------- |
| 201  | 222.105.131.225 | 222.105.131.225 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 222.105.131.225:443: connect: connection refused |
| 346  | 138.2.18.82     | 138.2.18.82     | IPv4   | none | N/A    | 0        | N/A    | dial tcp 138.2.18.82:443: connect: connection refused     |

#### DNS解析错误: 其他DNS错误 (1 次测试)

| 序号 | 主机/域名     | 目标IP        | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                                                              |
| ---- | ------------- | ------------- | ------ | ---- | ------ | -------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------- |
| 266  | 158.101.77.33 | 158.101.77.33 | IPv4   | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": read tcp 10.1.1.23:47500->158.101.77.33:443: read: connection reset by peer |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 175 次 (93.1%)
- **连接超时**: 10 次 (5.3%)
- **连接被拒绝**: 2 次 (1.1%)
- **DNS解析错误**: 1 次 (0.5%)

#### 错误模式分析

**超时集中度分析**: 共有 10 次超时，主要集中在IP段 172.64（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 187 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 13 次，IPv6失败 175 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：456.cloudflare.182682.xyz (3次),
abdullah.ns.cloudflare.com (3次), bowen.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 113 |
104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare | | 204 |
damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare
| | 280 | eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare
| | 352 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 81 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h3 | ✅ 成功 | 70 |
cloudflare | | 154 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h3 | ✅ 成功
| 71 | cloudflare | | 173 | asia.877774.xyz | 104.16.211.153 | IPv4 | h3 | ✅
成功 | 71 | cloudflare | | 438 | cu.877774.xyz | 104.26.4.119 | IPv4 | h3 | ✅
成功 | 71 | cloudflare | | 252 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅
成功 | 72 | cloudflare | | 395 | www.digitalocean.com | 104.19.174.68 | IPv4 |
h3 | ✅ 成功 | 72 | cloudflare | | 495 | cfip.xxxxxxxx.tk | 190.93.247.169 |
IPv4 | h3 | ✅ 成功 | 72 | cloudflare | | 86 | www.okcupid.com | 104.17.48.63 |
IPv4 | h3 | ✅ 成功 | 73 | cloudflare | | 320 | cmcc.877774.xyz | 104.16.149.10
| IPv4 | h3 | ✅ 成功 | 73 | cloudflare | | 30 | bowen.ns.cloudflare.com |
172.64.35.83 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare | | 176 |
04c6cf21-1294-4fae-8bf8-715bbc897b60.masx200.netlib.re | 172.67.174.219 | IPv4 |
h3 | ✅ 成功 | 74 | cloudflare | | 306 | stock.hostmonit.com | 104.21.7.193 |
IPv4 | h3 | ✅ 成功 | 74 | cloudflare | | 342 | www.hugedomains.com |
104.26.6.37 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare | | 383 |
ae8a9c24-83de.masx200.ddns-ip.net | 104.21.14.41 | IPv4 | h3 | ✅ 成功 | 74 |
cloudflare | | 15 | 456.cloudflare.182682.xyz | 104.26.8.160 | IPv4 | h3 | ✅
成功 | 75 | cloudflare | | 288 | ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅
成功 | 75 | cloudflare | | 323 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 |
✅ 成功 | 75 | cloudflare | | 85 | www.okcupid.com | 104.16.223.254 | IPv4 | h3
| ✅ 成功 | 76 | cloudflare | | 315 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3
| ✅ 成功 | 76 | cloudflare | | 500 | cfip.xxxxxxxx.tk | 104.17.127.110 | IPv4 |
h3 | ✅ 成功 | 76 | cloudflare | | 73 | na.877774.xyz | 104.18.38.235 | IPv4 |
h3 | ✅ 成功 | 77 | cloudflare | | 366 | www.4chan.org | 104.16.228.229 | IPv4 |
h3 | ✅ 成功 | 77 | cloudflare | | 59 | www.ipchicken.com | 104.26.7.112 | IPv4
| h3 | ✅ 成功 | 78 | cloudflare | | 232 | ip.sb | 104.26.12.31 | IPv4 | h3 | ✅
成功 | 78 | cloudflare | | 292 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 |
78 | cloudflare | | 334 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3 | ✅ 成功
| 78 | cloudflare | | 347 | www.7749tv.com | 104.19.133.4 | IPv4 | h3 | ✅ 成功
| 78 | cloudflare | | 357 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 78
| cloudflare | | 60 | www.ipchicken.com | 172.67.68.101 | IPv4 | h3 | ✅ 成功 |
79 | cloudflare | | 94 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h3 |
✅ 成功 | 79 | cloudflare | | 134 | palera.in | 172.67.157.122 | IPv4 | h3 | ✅
成功 | 79 | cloudflare | | 159 | local-aria2-webui.masx200.ddns-ip.net |
104.21.14.41 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare | | 233 | ip.sb |
104.26.13.31 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare | | 311 | 172.64.35.24 |
172.64.35.24 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare | | 374 | bestcf.030101.xyz
| 104.17.54.201 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare | | 421 |
huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare
| | 454 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h3 | ✅ 成功 | 79 |
cloudflare | | 231 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 80 |
cloudflare | | 249 | 172.64.159.6 | 172.64.159.6 | IPv4 | h3 | ✅ 成功 | 80 |
cloudflare | | 399 | tasteatlas.com | 104.17.36.105 | IPv4 | h3 | ✅ 成功 | 80 |
cloudflare | | 477 | yx-auto.pages.dev | 172.67.161.98 | IPv4 | h3 | ✅ 成功 |
80 | cloudflare | | 17 | 456.cloudflare.182682.xyz | 104.26.9.160 | IPv4 | h3 |
✅ 成功 | 81 | cloudflare | | 63 | uriah.ns.cloudflare.com | 172.64.35.194 |
IPv4 | h3 | ✅ 成功 | 81 | cloudflare | | 208 | cfip.1323123.xyz |
104.16.133.220 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare | | 251 | singapore.com |
172.67.75.194 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare | | 262 | ip.gs |
104.21.14.176 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare | | 267 | japan.com |
104.26.5.60 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare | | 300 | gamer.com.tw |
104.18.3.197 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare | | 359 | cf.0sm.com |
104.21.7.133 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare | | 364 | shopify.com |
23.227.38.33 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare | | 427 | www.pcmag.com |
104.16.21.118 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare | | 472 |
moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare
| | 494 | cfip.xxxxxxxx.tk | 104.25.105.1 | IPv4 | h3 | ✅ 成功 | 81 |
cloudflare | | 9 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 82 |
cloudflare | | 34 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅
成功 | 82 | cloudflare | | 163 | www.udemy.com | 104.16.143.237 | IPv4 | h3 | ✅
成功 | 82 | cloudflare | | 298 | 104.16.45.84 | 104.16.45.84 | IPv4 | h3 | ✅
成功 | 82 | cloudflare | | 327 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h3 | ✅
成功 | 82 | cloudflare | | 7 | 104.18.78.214 | 104.18.78.214 | IPv4 | h3 | ✅
成功 | 83 | cloudflare | | 16 | 456.cloudflare.182682.xyz | 172.67.75.208 | IPv4
| h3 | ✅ 成功 | 83 | cloudflare | | 71 | 172.67.181.209 | 172.67.181.209 | IPv4
| h3 | ✅ 成功 | 83 | cloudflare | | 90 | www.glassdoor.com | 104.17.64.70 |
IPv4 | h3 | ✅ 成功 | 83 | cloudflare | | 91 | www.glassdoor.com | 104.16.25.46
| IPv4 | h3 | ✅ 成功 | 83 | cloudflare | | 151 | 104.16.223.179 |
104.16.223.179 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare | | 191 | www.visa.com.hk
| 104.18.21.69 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare | | 202 |
damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h3 | ✅ 成功 | 83 |
cloudflare | | 305 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 83
| cloudflare | | 322 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅ 成功 |
83 | cloudflare | | 333 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功
| 83 | cloudflare | | 413 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 |
h3 | ✅ 成功 | 83 | cloudflare | | 439 | cu.877774.xyz | 104.26.4.111 | IPv4 |
h3 | ✅ 成功 | 83 | cloudflare | | 74 | na.877774.xyz | 104.18.187.25 | IPv4 |
h3 | ✅ 成功 | 84 | cloudflare | | 99 | dnschecker.org | 104.26.6.89 | IPv4 | h3
| ✅ 成功 | 84 | cloudflare | | 105 | benedict.ns.cloudflare.com |
108.162.195.205 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare | | 128 |
dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 84 |
cloudflare | | 239 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 84 |
cloudflare | | 250 | singapore.com | 104.26.13.140 | IPv4 | h3 | ✅ 成功 | 84 |
cloudflare | | 381 | 172.64.146.16 | 172.64.146.16 | IPv4 | h3 | ✅ 成功 | 84 |
cloudflare | | 431 | zread.ai | 172.67.202.78 | IPv4 | h3 | ✅ 成功 | 84 |
cloudflare | | 446 | iplocation.io | 104.26.10.222 | IPv4 | h3 | ✅ 成功 | 84 |
cloudflare | | 492 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h3 | ✅ 成功 |
84 | cloudflare | | 69 | saas.sin.fan | 162.159.36.20 | IPv4 | h3 | ✅ 成功 | 85
| cloudflare | | 72 | na.877774.xyz | 104.19.74.233 | IPv4 | h3 | ✅ 成功 | 85 |
cloudflare | | 76 | cf.877771.xyz | 172.67.152.183 | IPv4 | h3 | ✅ 成功 | 85 |
cloudflare | | 111 | 172.64.148.15 | 172.64.148.15 | IPv4 | h3 | ✅ 成功 | 85 |
cloudflare | | 112 | 104.18.254.88 | 104.18.254.88 | IPv4 | h3 | ✅ 成功 | 85 |
cloudflare | | 138 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h3 | ✅
成功 | 85 | cloudflare | | 243 | 103.160.204.59 | 103.160.204.59 | IPv4 | h3 |
✅ 成功 | 85 | cloudflare | | 299 | gamer.com.tw | 104.18.2.197 | IPv4 | h3 | ✅
成功 | 85 | cloudflare | | 321 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 |
✅ 成功 | 85 | cloudflare | | 341 | www.hugedomains.com | 104.26.7.37 | IPv4 |
h3 | ✅ 成功 | 85 | cloudflare | | 353 | www.csgo.com | 195.85.59.95 | IPv4 | h3
| ✅ 成功 | 85 | cloudflare | | 368 | www.visa.com.sg | 104.18.13.229 | IPv4 |
h3 | ✅ 成功 | 85 | cloudflare | | 445 | 172.67.110.232 | 172.67.110.232 | IPv4
| h3 | ✅ 成功 | 85 | cloudflare | | 61 | uriah.ns.cloudflare.com |
108.162.195.194 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare | | 124 | ifconfig.co |
104.21.54.91 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 100 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 13 次
- **IPv6 失败**: 175 次

### 按协议统计

- **none**: 187 次失败
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
