# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/19 02:15:46
- **数据来源**: connectivity_results-20251219-021546.json
- **总测试数**: 453
- **失败测试数**: 2
- **成功测试数**: 451
- **失败率**: 0.44%
- **平均延迟**: 54.53ms
- **最小延迟**: 33ms
- **最大延迟**: 521ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/19 02:15:46
- **IP地址**: 2a09:bac5:6211:1250::1d3:d4
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 37.1835, -121.7714
- **时区**: America/Los_Angeles
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
| 34 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 391 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

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
| 332 | silkbook.com | 172.67.75.208 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 123 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 397 | icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 112 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 118 | www.okcupid.com | 104.16.239.254 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 407 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 108 | www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 181 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 205 | www.whatismyip.com | 104.26.12.23 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 252 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 259 | cf.090227.xyz | 172.64.144.82 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 414 | 172.67.181.209 | 172.67.181.209 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 9 | www.ipget.net | 188.114.96.3 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 21 | ipinfo.in | 172.67.198.203 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 25 | steamdb.info | 172.66.175.250 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 28 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 48 | 104.18.37.13 | 104.18.37.13 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 49 | www.gov.ua | 104.21.23.72 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 73 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 116 | www.okcupid.com | 104.16.223.254 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 125 | 172.67.243.218 | 172.67.243.218 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 147 | cu.877774.xyz | 104.26.4.116 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 151 | cu.877774.xyz | 104.26.4.111 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 191 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 207 | www.whatismyip.com | 104.26.13.23 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 241 | bestcf.030101.xyz | 104.17.96.48 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 250 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 253 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 262 | cf.090227.xyz | 2606:4700:4407::ac40:9052 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 297 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 328 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 329 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 379 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 406 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 409 | eur.877774.xyz | 104.21.47.209 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 443 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 446 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 26 | steamdb.info | 104.20.34.212 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 52 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 58 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 106 | www.visa.cn | 162.159.153.2 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 109 | www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 115 | www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 120 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 128 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 129 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 171 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 174 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 178 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 179 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 192 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 221 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 231 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 234 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 246 | cf.zhetengsha.eu.org | 172.64.144.82 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 254 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 261 | cf.090227.xyz | 2a06:98c1:310d::6812:2bae | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 289 | time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 292 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 313 | ip.gs | 104.21.14.176 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 316 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 320 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 330 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 338 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 339 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 341 | www.visa.com.hk | 104.18.21.69 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 343 | 104.18.42.26 | 104.18.42.26 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 362 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 393 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 415 | 104.19.175.123 | 104.19.175.123 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 427 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 430 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 435 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 444 | japan.com | 104.26.5.60 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 445 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 447 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 22 | ipinfo.in | 104.21.21.129 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 24 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 64 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 67 | palera.in | 172.67.157.122 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 68 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 74 | iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 75 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 76 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 78 | 104.18.254.88 | 104.18.254.88 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 107 | www.visa.cn | 162.159.152.2 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 113 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 121 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 148 | cu.877774.xyz | 104.26.4.117 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 170 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 173 | toy-people.com | 104.26.3.36 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 175 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 177 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 180 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 198 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 208 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 209 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 219 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 223 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 251 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 257 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 273 | www.glassdoor.com | 104.17.64.70 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 308 | ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 324 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 327 | singapore.com | 104.26.13.140 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 335 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 337 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 344 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 355 | dnschecker.org | 104.26.6.89 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 363 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 364 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 366 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 382 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 395 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 402 | 104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 429 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 431 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 442 | japan.com | 172.67.70.92 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 3 | shopify.com | 23.227.38.33 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 17 | www.pcmag.com | 104.16.20.118 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 51 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 62 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 89 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 94 | icook.hk | 172.67.161.104 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 103 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 139 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 168 | zread.ai | 172.67.202.78 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 186 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 206 | www.whatismyip.com | 172.67.69.129 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 222 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 225 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 315 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 325 | singapore.com | 172.67.75.194 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 340 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 342 | www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 359 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 361 | cf.877774.xyz | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 365 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 378 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 380 | www.udemy.com | 104.16.143.237 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 381 | www.udemy.com | 104.16.142.237 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 401 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 422 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 432 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 10 | www.ipget.net | 188.114.97.3 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 19 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 45 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 55 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 95 | icook.hk | 104.21.90.210 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 96 | icook.hk | 2a06:98c1:3121::3 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 134 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 154 | na.877774.xyz | 104.18.38.235 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 167 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 210 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 213 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 215 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 237 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 238 | 172.64.151.55 | 172.64.151.55 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 239 | saas.sin.fan | 162.159.36.20 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 240 | saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 255 | fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 269 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 280 | 104.16.61.163 | 104.16.61.163 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 307 | ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 314 | ip.gs | 172.67.160.28 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 331 | silkbook.com | 104.26.9.160 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 347 | 162.159.133.85 | 162.159.133.85 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 348 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 356 | dnschecker.org | 104.26.7.89 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 357 | dnschecker.org | 172.67.73.216 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 358 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 376 | tasteatlas.com | 104.17.36.105 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 387 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 390 | 104.17.142.12 | 104.17.142.12 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 423 | www.csgo.com | 195.85.59.161 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 426 | ifconfig.co | 2606:4700:3030::ac43:a86a | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 428 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 12 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 15 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 20 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 23 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 37 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 41 | ct.877774.xyz | 172.64.229.174 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 47 | ct.877774.xyz | 172.64.229.161 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 65 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 105 | 172.67.120.0 | 172.67.120.0 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 117 | www.okcupid.com | 104.18.160.63 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 124 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 159 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 197 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 211 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 217 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 227 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 236 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 242 | bestcf.030101.xyz | 104.19.146.144 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 248 | cf.zhetengsha.eu.org | 2a06:98c1:3101::ac40:919e | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 270 | www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 271 | www.ipchicken.com | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 279 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 288 | time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 200 条记录
- **快 (50-100ms)**: 0 条记录
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
