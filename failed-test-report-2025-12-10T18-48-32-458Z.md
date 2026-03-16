# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/10 18:48:32
- **数据来源**: connectivity_results-20251210-184832.json
- **总测试数**: 481
- **失败测试数**: 178
- **成功测试数**: 303
- **失败率**: 37.01%
- **平均延迟**: 142.28ms
- **最小延迟**: 75ms
- **最大延迟**: 702ms

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
| 334  | 111.171.108.67   | 111.171.108.67  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout  |
| 470  | 175.212.207.13   | 175.212.207.13  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout  |
| 472  | 172.67.49.134    | 172.67.49.134   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout   |
| 473  | 172.64.201.25    | 172.64.201.25   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout   |
| 474  | 119.194.220.146  | 119.194.220.146 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout |
| 475  | 121.188.182.190  | 121.188.182.190 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout |
| 481  | cfip.xxxxxxxx.tk | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 171 次 (96.1%)
- **连接超时**: 7 次 (3.9%)

#### 错误模式分析

**超时集中度分析**: 共有 7 次超时，主要集中在IP段 111.171（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 178 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 7 次，IPv6失败 171 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：ashton.ns.cloudflare.com (3次), ip.sb
(3次), braden.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 87 |
www.gov.ua | 172.67.209.127 | IPv4 | h3 | ✅ 成功 | 75 | cloudflare | | 331 |
ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅ 成功 | 76 | cloudflare | | 366 |
104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 77 | cloudflare | | 428 |
comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 78 | cloudflare | | 186 |
wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare
| | 241 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h3 | ✅ 成功 | 80
| cloudflare | | 284 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 81 |
cloudflare | | 248 | 104.18.254.88 | 104.18.254.88 | IPv4 | h3 | ✅ 成功 | 83 |
cloudflare | | 457 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 83
| cloudflare | | 206 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅
成功 | 84 | cloudflare | | 262 | www.pcmag.com | 104.16.21.118 | IPv4 | h3 | ✅
成功 | 84 | cloudflare | | 266 | dnschecker.org | 104.26.7.89 | IPv4 | h3 | ✅
成功 | 84 | cloudflare | | 422 | www.whatismyip.com | 104.26.13.23 | IPv4 | h3 |
✅ 成功 | 84 | cloudflare | | 461 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h3 |
✅ 成功 | 85 | cloudflare | | 235 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3
| ✅ 成功 | 89 | cloudflare | | 249 | bestcf.030101.xyz | 104.17.179.12 | IPv4 |
h3 | ✅ 成功 | 89 | cloudflare | | 365 | 172.64.154.18 | 172.64.154.18 | IPv4 |
h3 | ✅ 成功 | 90 | cloudflare | | 377 | lewis.ns.cloudflare.com |
108.162.195.159 | IPv4 | h3 | ✅ 成功 | 90 | cloudflare | | 395 |
huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅ 成功 | 92 | cloudflare
| | 143 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功 | 95 |
cloudflare | | 446 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 95
| cloudflare | | 419 | www.digitalocean.com | 104.19.174.68 | IPv4 | h3 | ✅
成功 | 96 | cloudflare | | 307 | cloudflare-ip.mofashi.ltd | 104.21.72.233 |
IPv4 | h3 | ✅ 成功 | 97 | cloudflare | | 261 | www.pcmag.com | 104.16.20.118 |
IPv4 | h3 | ✅ 成功 | 100 | cloudflare | | 361 | cf.0sm.com | 104.21.7.133 |
IPv4 | h3 | ✅ 成功 | 100 | cloudflare | | 362 | cf.0sm.com | 172.67.187.145 |
IPv4 | h3 | ✅ 成功 | 101 | cloudflare | | 144 | uriah.ns.cloudflare.com |
108.162.195.194 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare | | 373 | www.4chan.org
| 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare | | 136 |
decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h3 | ✅ 成功 | 103 |
cloudflare | | 413 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h3 | ✅
成功 | 104 | cloudflare | | 414 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4
| h3 | ✅ 成功 | 104 | cloudflare | | 255 | 456.cloudflare.182682.xyz |
104.26.8.160 | IPv4 | h3 | ✅ 成功 | 105 | cloudflare | | 122 |
craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h3 | ✅ 成功 | 106 |
cloudflare | | 150 | eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 106
| cloudflare | | 335 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h3 | ✅ 成功
| 106 | cloudflare | | 343 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅
成功 | 106 | cloudflare | | 124 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4
| h3 | ✅ 成功 | 107 | cloudflare | | 171 | kyree.ns.cloudflare.com |
172.64.35.207 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare | | 190 |
pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h3 | ✅ 成功 | 107 |
cloudflare | | 286 | ip.gs | 104.21.14.176 | IPv4 | h3 | ✅ 成功 | 107 |
cloudflare | | 316 | freeyx.cloudflare88.eu.org | 141.101.120.120 | IPv4 | h3 |
✅ 成功 | 107 | cloudflare | | 340 | ct.877774.xyz | 172.64.229.217 | IPv4 | h3
| ✅ 成功 | 107 | cloudflare | | 83 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅
成功 | 108 | cloudflare | | 142 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3 |
✅ 成功 | 108 | cloudflare | | 197 | icook.hk | 172.67.161.104 | IPv4 | h3 | ✅
成功 | 108 | cloudflare | | 204 | cris.ns.cloudflare.com | 108.162.195.202 |
IPv4 | h3 | ✅ 成功 | 108 | cloudflare | | 243 | damien.ns.cloudflare.com |
172.64.35.168 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare | | 434 | japan.com |
104.26.4.60 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare | | 111 | 172.64.33.67 |
172.64.33.67 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare | | 131 | toy-people.com |
104.26.3.36 | IPv4 | h3 | ✅ 成功 | 109 | cloudflare | | 176 |
rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h3 | ✅ 成功 | 109 |
cloudflare | | 352 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3 | ✅ 成功 | 109 |
cloudflare | | 358 | ae8a9c24-83de.masx200.ddns-ip.net | 104.21.14.41 | IPv4 |
h3 | ✅ 成功 | 109 | cloudflare | | 392 | 104.18.37.40 | 104.18.37.40 | IPv4 |
h3 | ✅ 成功 | 109 | cloudflare | | 466 | cmcc.877774.xyz | 104.16.148.6 | IPv4
| h3 | ✅ 成功 | 109 | cloudflare | | 76 | 172.64.146.16 | 172.64.146.16 | IPv4
| h3 | ✅ 成功 | 110 | cloudflare | | 196 | icook.hk | 104.21.90.210 | IPv4 | h3
| ✅ 成功 | 110 | cloudflare | | 259 | www.visa.cn | 162.159.153.2 | IPv4 | h3 |
✅ 成功 | 110 | cloudflare | | 464 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h3
| ✅ 成功 | 110 | cloudflare | | 45 | dylan.ns.cloudflare.com | 172.64.35.187 |
IPv4 | h3 | ✅ 成功 | 111 | cloudflare | | 178 | rustam.ns.cloudflare.com |
172.64.35.148 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare | | 214 | www.okcupid.com
| 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare | | 408 |
yx-auto.pages.dev | 104.21.6.60 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare | | 145
| uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h3 | ✅ 成功 | 112 |
cloudflare | | 154 | 172.64.147.73 | 172.64.147.73 | IPv4 | h3 | ✅ 成功 | 112 |
cloudflare | | 330 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅ 成功 | 112 |
cloudflare | | 347 | 104.19.175.123 | 104.19.175.123 | IPv4 | h3 | ✅ 成功 | 112
| cloudflare | | 8 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 113 |
cloudflare | | 155 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 | 113
| cloudflare | | 156 | www.visa.com.hk | 104.18.21.69 | IPv4 | h3 | ✅ 成功 |
113 | cloudflare | | 277 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅
成功 | 113 | cloudflare | | 279 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3 | ✅
成功 | 113 | cloudflare | | 281 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅
成功 | 113 | cloudflare | | 293 | 172.64.38.15 | 172.64.38.15 | IPv4 | h3 | ✅
成功 | 113 | cloudflare | | 346 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 |
✅ 成功 | 113 | cloudflare | | 357 | ae8a9c24-83de.masx200.ddns-ip.net |
172.67.157.182 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare | | 386 | ifconfig.co |
104.21.54.91 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare | | 49 | 104.17.142.12 |
104.17.142.12 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare | | 104 | singapore.com |
104.26.12.140 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare | | 32 |
local-aria2-webui.masx200.ddns-ip.net | 104.21.14.41 | IPv4 | h3 | ✅ 成功 | 115
| cloudflare | | 183 | 172.67.106.26 | 172.67.106.26 | IPv4 | h3 | ✅ 成功 | 115
| cloudflare | | 349 | cf.zhetengsha.eu.org | 172.64.145.158 | IPv4 | h3 | ✅
成功 | 115 | cloudflare | | 73 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功
| 116 | cloudflare | | 90 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4
| h3 | ✅ 成功 | 116 | cloudflare | | 240 | 104.18.78.214 | 104.18.78.214 | IPv4
| h3 | ✅ 成功 | 116 | cloudflare | | 280 | cf.877774.xyz | 104.18.41.190 | IPv4
| h3 | ✅ 成功 | 116 | cloudflare | | 304 | asia.877774.xyz | 104.17.142.146 |
IPv4 | h3 | ✅ 成功 | 116 | cloudflare | | 480 | cfip.xxxxxxxx.tk |
190.93.244.201 | IPv4 | h3 | ✅ 成功 | 116 | cloudflare | | 117 | cu.877774.xyz
| 104.26.4.112 | IPv4 | h3 | ✅ 成功 | 117 | cloudflare | | 123 |
craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h3 | ✅ 成功 | 117 |
cloudflare | | 336 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h3 | ✅ 成功
| 117 | cloudflare | | 342 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功
| 117 | cloudflare | | 459 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅
成功 | 117 | cloudflare | | 59 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 |
118 | cloudflare | | 215 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3
| ✅ 成功 | 118 | cloudflare | | 439 | yx-auto.pages.dev | 172.66.47.112 | IPv4
| h3 | ✅ 成功 | 118 | cloudflare | | 105 | singapore.com | 104.26.13.140 | IPv4
| h3 | ✅ 成功 | 119 | cloudflare | | 128 | 172.64.159.6 | 172.64.159.6 | IPv4 |
h3 | ✅ 成功 | 119 | cloudflare | | 424 | www.whatismyip.com | 104.26.12.23 |
IPv4 | h3 | ✅ 成功 | 119 | cloudflare | | 455 | cmcc.877774.xyz | 104.16.149.8
| IPv4 | h3 | ✅ 成功 | 119 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 23 条记录
- **正常 (100-200ms)**: 77 条记录
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
