# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/19 02:16:36
- **数据来源**: connectivity_results-20251219-021635.json
- **总测试数**: 458
- **失败测试数**: 2
- **成功测试数**: 456
- **失败率**: 0.44%
- **平均延迟**: 96.00ms
- **最小延迟**: 65ms
- **最大延迟**: 659ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/19 02:16:36
- **IP地址**: 2a09:bac1:76c0:8e0::c:381
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 33.4475, -112.0866
- **时区**: America/Phoenix
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 2 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (2 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 97 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |
| 284 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 2 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 172.64（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 2 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: 所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好



---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 148 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 106 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 132 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 299 | www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 429 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 101 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 239 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 41 | silkbook.com | 104.26.9.160 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 72 | dnschecker.org | 104.26.7.89 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 151 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 238 | shopify.com | 23.227.38.33 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 279 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 311 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 336 | cu.877774.xyz | 104.26.4.115 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 413 | cf.zhetengsha.eu.org | 2606:4700:4407::ac40:9052 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 152 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 374 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 425 | cf.090227.xyz | 2a06:98c1:3101::ac40:919e | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 124 | 104.19.223.58 | 104.19.223.58 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 128 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 163 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 268 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 315 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 346 | freeyx.cloudflare88.eu.org | 2606:4700:3010:0:fb:e00f:f2d6:8f63 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 421 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 45 | 104.18.42.26 | 104.18.42.26 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 52 | 104.18.14.76 | 104.18.14.76 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 54 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 111 | icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 113 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 137 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 166 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 168 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 282 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 329 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 333 | na.877774.xyz | 104.19.74.233 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 409 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 414 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 426 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 43 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 95 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 105 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 112 | icook.tw | 104.20.28.74 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 144 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 155 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 171 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 219 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 242 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 298 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 318 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 334 | na.877774.xyz | 104.18.38.235 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 341 | cu.877774.xyz | 104.26.4.111 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 392 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 449 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 73 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 107 | 172.67.79.211 | 172.67.79.211 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 125 | eur.877774.xyz | 104.21.26.150 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 142 | ifconfig.co | 172.67.168.106 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 157 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 194 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 210 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 245 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 250 | steamdb.info | 172.66.175.250 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 272 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 275 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 332 | na.877774.xyz | 104.18.187.25 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 350 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 367 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 415 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 418 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 103 | www.udemy.com | 104.16.143.237 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 172 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 187 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 196 | www.pcmag.com | 104.16.21.118 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 207 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 228 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 233 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 256 | ct.877774.xyz | 172.64.229.161 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 267 | iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 303 | www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 364 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 419 | fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 432 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 451 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 36 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 66 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 85 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 87 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 121 | 104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 131 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 143 | ifconfig.co | 2606:4700:3030::ac43:a86a | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 146 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 167 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 176 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 203 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 234 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 274 | www.4chan.org | 104.16.228.229 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 320 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 382 | zread.ai | 104.21.76.240 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 385 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 404 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 453 | time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 37 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 51 | 172.67.106.26 | 172.67.106.26 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 84 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 133 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 185 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 198 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 200 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 237 | 104.16.45.84 | 104.16.45.84 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 246 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 248 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 265 | 104.18.254.88 | 104.18.254.88 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 297 | www.visa.cn | 162.159.153.2 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 352 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 360 | asia.877774.xyz | 104.16.211.153 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 361 | asia.877774.xyz | 104.17.139.62 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 372 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 375 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 381 | zread.ai | 172.67.202.78 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 403 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 417 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 434 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 438 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 454 | time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 456 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 35 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 39 | silkbook.com | 104.26.8.160 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 50 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 55 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 61 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 74 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 160 | www.ipget.net | 104.21.15.212 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 161 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 222 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 236 | 104.18.39.196 | 104.18.39.196 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 257 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 289 | 172.67.120.0 | 172.67.120.0 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 309 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 343 | cu.877774.xyz | 104.26.4.113 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 383 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 395 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 398 | bestcf.030101.xyz | 2606:4700:0:3701:6875:ca63:41c9:a311 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 411 | cf.zhetengsha.eu.org | 104.18.43.174 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 420 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 5 | www.glassdoor.com | 104.17.64.70 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 13 | ip.gs | 172.67.160.28 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 46 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 47 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 60 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 70 | dnschecker.org | 172.67.73.216 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 102 | 104.17.142.12 | 104.17.142.12 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 104 | www.udemy.com | 104.16.142.237 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 119 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 141 | ifconfig.co | 104.21.54.91 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 162 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 216 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 247 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 269 | iplocation.io | 104.26.10.222 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 300 | www.okcupid.com | 104.16.223.254 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 302 | www.okcupid.com | 104.18.160.63 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 319 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 345 | freeyx.cloudflare88.eu.org | 141.101.121.134 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 366 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 378 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 397 | bestcf.030101.xyz | 104.19.146.144 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 439 | www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 440 | www.ipchicken.com | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 457 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 14 | ip.gs | 104.21.14.176 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 22 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 42 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 94 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 96 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 99 | tasteatlas.com | 104.17.36.105 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 108 | 198.62.62.4 | 198.62.62.4 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 122 | 104.18.78.214 | 104.18.78.214 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 123 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 174 | 172.67.181.209 | 172.67.181.209 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 179 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 195 | www.pcmag.com | 104.16.20.118 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 206 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 211 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 225 | cf.0sm.com | 104.21.7.133 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 240 | www.gov.ua | 172.67.209.127 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 288 | cfip.xxxxxxxx.tk | 104.17.127.110 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 305 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 308 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 313 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 330 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 340 | cu.877774.xyz | 104.26.4.119 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 348 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 373 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 443 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 15 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 20 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 76 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 79 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 109 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 114 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 200 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 2 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 2 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*
