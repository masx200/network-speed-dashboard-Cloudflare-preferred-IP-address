# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:24:19
- **数据来源**: connectivity_results-20251229-172418.json
- **总测试数**: 492
- **失败测试数**: 5
- **成功测试数**: 487
- **失败率**: 1.02%
- **平均延迟**: 81.58ms
- **最小延迟**: 44ms
- **最大延迟**: 622ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:24:19
- **IP地址**: 2a09:bac5:9f22:2828::400:1
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 36.4766, -78.1847
- **时区**: America/New_York
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 5 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (5 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 110 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | none | N/A | 0 | N/A | dial tcp 162.159.44.187:443: i/o timeout |
| 292 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |
| 395 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | none | N/A | 0 | N/A | dial tcp 108.162.195.203:443: i/o timeout |
| 435 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 456 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.35.110:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 5 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 5 次超时，主要集中在IP段 172.64（2 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 5 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: 所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好



---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 198 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 132 | fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 411 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 100 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 119 | saas.sin.fan | 162.159.36.20 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 409 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 80 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 108 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 264 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 410 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 103 | asia.877774.xyz | 104.17.139.62 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 308 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 335 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 154 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 219 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 233 | silkbook.com | 172.67.75.208 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 351 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 354 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 400 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 413 | cf.0sm.com | 104.21.7.133 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 438 | 104.16.45.84 | 104.16.45.84 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 462 | cu.877774.xyz | 172.64.145.202 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 31 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 48 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 99 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 106 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 107 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 128 | xn--b6gac.eu.org | 188.114.96.3 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 205 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 328 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 367 | 172.64.229.7 | 172.64.229.7 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 369 | 162.159.137.204 | 162.159.137.204 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 432 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 486 | www.okcupid.com | 104.16.223.254 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 67 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 76 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 81 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 130 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 169 | palera.in | 188.114.96.3 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 170 | palera.in | 188.114.97.3 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 190 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 204 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 226 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 235 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 236 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 283 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 349 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 368 | 104.26.5.134 | 104.26.5.134 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 378 | 104.19.148.121 | 104.19.148.121 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 390 | 104.16.105.166 | 104.16.105.166 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 404 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 442 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 450 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 490 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 73 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 120 | saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 270 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 294 | 198.62.62.4 | 198.62.62.4 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 347 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 419 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 420 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 434 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 105 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 187 | ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 191 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 197 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 201 | singapore.com | 104.26.12.140 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 215 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 234 | silkbook.com | 104.26.8.160 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 253 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 345 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 348 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 364 | 104.26.4.90 | 104.26.4.90 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 384 | 104.18.255.167 | 104.18.255.167 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 405 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 424 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 431 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 466 | ct.877774.xyz | 172.64.229.161 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 469 | ct.877774.xyz | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 474 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 78 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 133 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 160 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 196 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 223 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 231 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 246 | 172.67.106.26 | 172.67.106.26 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 257 | dnschecker.org | 104.26.6.89 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 286 | www.udemy.com | 104.16.143.237 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 340 | 198.41.208.15 | 198.41.208.15 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 355 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 356 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 365 | 162.159.136.89 | 162.159.136.89 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 392 | 104.18.189.153 | 104.18.189.153 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 439 | shopify.com | 23.227.38.33 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 7 | www.ipget.net | 172.67.207.26 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 42 | freeyx.cloudflare88.eu.org | 2606:4700:3010:0:fb:e00f:f23d:42c6 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 97 | www.whatismyip.com | 172.67.69.129 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 121 | 172.64.151.55 | 172.64.151.55 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 129 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 147 | www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 166 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 200 | singapore.com | 172.67.75.194 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 212 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 256 | dnschecker.org | 104.26.7.89 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 261 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 279 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 333 | www.7749tv.com | 104.19.133.4 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 350 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 371 | 104.26.3.162 | 104.26.3.162 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 414 | cf.0sm.com | 172.67.187.145 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 427 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 436 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 461 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 471 | ct.877774.xyz | 172.64.229.217 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 481 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 11 | comicabc.com | 172.67.174.21 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 13 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 21 | www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 49 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 68 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 94 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 95 | www.whatismyip.com | 104.26.13.23 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 123 | cf.zhetengsha.eu.org | 104.18.35.15 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 125 | cf.zhetengsha.eu.org | 2606:4700:440a::ac40:98f1 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 153 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 171 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 189 | ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 203 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 214 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 227 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 229 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 232 | silkbook.com | 104.26.9.160 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 243 | www.visa.com.hk | 104.18.21.69 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 260 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 310 | eur.877774.xyz | 104.21.26.150 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 357 | 162.159.140.85 | 162.159.140.85 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 370 | 162.159.128.253 | 162.159.128.253 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 373 | 162.159.140.116 | 162.159.140.116 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 381 | 104.17.139.37 | 104.17.139.37 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 417 | www.pcmag.com | 104.16.21.118 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 423 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 453 | 172.67.49.134 | 172.67.49.134 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 38 | na.877774.xyz | 104.19.74.233 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 156 | www.glassdoor.com | 104.17.64.70 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 168 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 282 | tasteatlas.com | 104.17.37.105 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 289 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 311 | eur.877774.xyz | 104.21.29.164 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 336 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 403 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 421 | ipinfo.in | 104.21.21.129 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 429 | steamdb.info | 104.20.34.212 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 487 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 39 | na.877774.xyz | 104.18.38.235 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 79 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 96 | www.whatismyip.com | 104.26.12.23 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 98 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 112 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 124 | cf.zhetengsha.eu.org | 2a06:98c1:3105::6812:230f | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 131 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 134 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 146 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 172 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 216 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 240 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 262 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 295 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 352 | japan.com | 172.67.70.92 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 402 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 426 | www.gov.ua | 172.67.209.127 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 479 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 492 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 104 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 126 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 127 | xn--b6gac.eu.org | 188.114.97.3 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 137 | cf.090227.xyz | 172.64.144.82 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 165 | time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 193 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 217 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 237 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 290 | 104.17.142.12 | 104.17.142.12 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 299 | icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 303 | 104.18.78.214 | 104.18.78.214 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 307 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 323 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 331 | www.csgo.com | 195.85.59.161 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 334 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 380 | 104.18.151.172 | 104.18.151.172 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 391 | 104.17.142.212 | 104.17.142.212 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 449 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 451 | 103.160.204.59 | 103.160.204.59 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 452 | 104.18.254.88 | 104.18.254.88 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 463 | cu.877774.xyz | 104.18.42.54 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 488 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 491 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 77 | toy-people.com | 104.26.3.36 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 179 | 104.16.61.163 | 104.16.61.163 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 188 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 202 | singapore.com | 104.26.13.140 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 10 条记录
- **快 (50-100ms)**: 190 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 5 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 5 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*
