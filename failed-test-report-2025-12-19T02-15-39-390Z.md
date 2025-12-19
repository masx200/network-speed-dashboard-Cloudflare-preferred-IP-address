# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/19 02:15:39
- **数据来源**: connectivity_results-20251219-021538.json
- **总测试数**: 453
- **失败测试数**: 2
- **成功测试数**: 451
- **失败率**: 0.44%
- **平均延迟**: 95.18ms
- **最小延迟**: 67ms
- **最大延迟**: 529ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/19 02:15:39
- **IP地址**: 2a09:bac5:6190:183c::26a:77
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
| 36 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 390 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 2 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 198.41（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 2 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: 所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好



---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 44 | cf.0sm.com | 172.67.187.145 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 6 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 32 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 51 | 104.18.39.196 | 104.18.39.196 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 339 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 427 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 62 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 75 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 91 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 292 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 385 | www.udemy.com | 104.16.143.237 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 31 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 54 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 90 | ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 121 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 136 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 189 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 224 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 316 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 353 | dnschecker.org | 172.67.73.216 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 445 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 4 | www.gov.ua | 104.21.23.72 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 38 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 68 | steamdb.info | 104.20.34.212 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 182 | zread.ai | 172.67.202.78 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 184 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 208 | www.whatismyip.com | 104.26.12.23 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 335 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 380 | tasteatlas.com | 104.17.36.105 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 29 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 53 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 61 | ct.877774.xyz | 172.64.229.217 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 64 | ct.877774.xyz | 172.64.229.161 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 85 | 104.18.254.88 | 104.18.254.88 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 100 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 156 | na.877774.xyz | 104.18.38.235 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 180 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 351 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 361 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 370 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 375 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 423 | ifconfig.co | 2606:4700:3030::ac43:a86a | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 424 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 11 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 14 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 46 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 67 | steamdb.info | 172.66.175.250 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 96 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 97 | icook.hk | 104.21.90.210 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 113 | www.okcupid.com | 104.18.160.63 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 126 | www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 164 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 186 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 216 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 231 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 281 | www.glassdoor.com | 104.17.64.70 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 332 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 358 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 405 | eur.877774.xyz | 104.21.29.164 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 429 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 56 | shopify.com | 23.227.38.33 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 99 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 118 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 133 | 172.67.243.218 | 172.67.243.218 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 183 | zread.ai | 104.21.76.240 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 212 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 244 | bestcf.030101.xyz | 104.19.146.144 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 255 | cf.zhetengsha.eu.org | 2a06:98c1:3108::6812:2a62 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 290 | www.digitalocean.com | 104.19.174.68 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 306 | palera.in | 104.21.58.72 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 329 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 340 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 362 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 379 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 431 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 446 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 9 | comicabc.com | 104.21.64.10 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 17 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 30 | iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 35 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 39 | www.pcmag.com | 104.16.20.118 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 65 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 71 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 109 | www.visa.cn | 162.159.152.2 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 125 | www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 137 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 144 | freeyx.cloudflare88.eu.org | 141.101.121.193 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 175 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 178 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 211 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 221 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 229 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 257 | fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 264 | cf.090227.xyz | 2606:4700:4407::ac40:9052 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 280 | www.glassdoor.com | 104.16.25.46 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 289 | www.digitalocean.com | 104.19.173.68 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 319 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 331 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 369 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 428 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 432 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 438 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 441 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 18 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 45 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 49 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 77 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 81 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 84 | 103.160.204.59 | 103.160.204.59 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 92 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 94 | www.4chan.org | 104.16.228.229 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 101 | 172.67.120.0 | 172.67.120.0 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 119 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 123 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 155 | na.877774.xyz | 104.18.187.25 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 181 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 197 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 207 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 213 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 225 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 228 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 236 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 243 | bestcf.030101.xyz | 104.17.96.48 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 291 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 308 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 313 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 322 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 324 | www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 359 | cf.877774.xyz | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 393 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 399 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 443 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 28 | iplocation.io | 104.26.10.222 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 69 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 80 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 89 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 112 | www.okcupid.com | 104.16.223.254 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 134 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 145 | freeyx.cloudflare88.eu.org | 2606:4700:3010:0:fb:e00f:f23d:42c6 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 152 | cu.877774.xyz | 104.26.4.119 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 188 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 205 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 214 | 172.64.151.55 | 172.64.151.55 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 223 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 234 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 237 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 240 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 241 | saas.sin.fan | 162.159.36.20 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 250 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 330 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 336 | silkbook.com | 104.26.9.160 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 343 | 172.67.106.26 | 172.67.106.26 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 357 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 386 | www.udemy.com | 104.16.142.237 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 387 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 396 | 104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 425 | 104.19.175.123 | 104.19.175.123 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 13 | www.ipget.net | 188.114.97.3 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 63 | ct.877774.xyz | 172.64.229.44 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 82 | 172.67.110.232 | 172.67.110.232 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 95 | www.4chan.org | 104.16.229.229 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 116 | cf.877771.xyz | 172.67.152.183 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 120 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 124 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 149 | cu.877774.xyz | 104.26.4.116 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 153 | cu.877774.xyz | 104.26.4.111 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 187 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 233 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 262 | cf.090227.xyz | 104.18.43.174 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 288 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 317 | ip.gs | 104.21.14.176 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 318 | ip.gs | 172.67.160.28 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 334 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 337 | silkbook.com | 172.67.75.208 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 344 | 104.18.14.76 | 104.18.14.76 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 345 | 162.159.133.85 | 162.159.133.85 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 400 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 58 | ct.877774.xyz | 172.64.229.174 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 66 | 104.18.37.13 | 104.18.37.13 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 70 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 106 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 115 | cf.877771.xyz | 104.21.80.180 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 151 | cu.877774.xyz | 104.26.4.118 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 157 | na.877774.xyz | 104.19.74.233 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 218 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 261 | cf.090227.xyz | 172.64.144.82 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 272 | www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 282 | 104.16.61.163 | 104.16.61.163 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 287 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 297 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 333 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 352 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 388 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 411 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 414 | 172.67.181.209 | 172.67.181.209 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 426 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 453 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 37 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 42 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 83 | cloudflare |
| 52 | 104.16.45.84 | 104.16.45.84 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |

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
