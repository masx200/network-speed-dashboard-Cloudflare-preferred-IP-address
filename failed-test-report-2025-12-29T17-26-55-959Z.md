# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:26:55
- **数据来源**: connectivity_results-20251229-172655.json
- **总测试数**: 494
- **失败测试数**: 2
- **成功测试数**: 492
- **失败率**: 0.40%
- **平均延迟**: 74.30ms
- **最小延迟**: 42ms
- **最大延迟**: 1660ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:26:55
- **IP地址**: 2a09:bac5:9f22:1678::23d:89
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

- **连接超时: I/O超时**: 2 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (2 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 25 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 375 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

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
| 329 | www.visa.com.hk | 104.18.21.69 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 334 | 104.18.42.26 | 104.18.42.26 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 202 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 63 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 186 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 211 | bestcf.030101.xyz | 2606:4700::fffd:819d:acda | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 254 | cf.090227.xyz | 2a06:98c1:310d::6812:2bae | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 313 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 167 | toy-people.com | 104.26.3.36 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 242 | cf.zhetengsha.eu.org | 2a06:98c1:3105::6812:230f | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 245 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 295 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 331 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 96 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 416 | 104.19.175.123 | 104.19.175.123 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 424 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 127 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 169 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 237 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 246 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 255 | cf.090227.xyz | 2606:4700:4407::ac40:9052 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 360 | dnschecker.org | 104.26.7.89 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 377 | www.udemy.com | 104.16.142.237 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 425 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 483 | 104.16.105.166 | 104.16.105.166 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 184 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 305 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 309 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 336 | 172.67.106.26 | 172.67.106.26 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 421 | www.csgo.com | 195.85.59.161 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 426 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 427 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 437 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 444 | japan.com | 172.67.70.92 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 445 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 446 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 473 | 104.18.151.172 | 104.18.151.172 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 102 | 172.67.120.0 | 172.67.120.0 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 114 | www.okcupid.com | 104.16.239.254 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 148 | na.877774.xyz | 104.19.74.233 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 157 | zread.ai | 104.21.76.240 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 200 | www.whatismyip.com | 172.67.69.129 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 221 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 303 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 327 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 350 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 357 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 449 | 104.18.255.167 | 104.18.255.167 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 468 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 474 | 104.17.139.37 | 104.17.139.37 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 43 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 82 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 101 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 208 | bestcf.030101.xyz | 198.41.209.230 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 223 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 296 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 315 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 324 | silkbook.com | 104.26.8.160 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 340 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 369 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 390 | www.digitalocean.com | 104.19.174.68 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 429 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 451 | 104.26.4.90 | 104.26.4.90 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 462 | 104.18.89.52 | 104.18.89.52 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 478 | 104.19.212.207 | 104.19.212.207 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 484 | 104.17.142.212 | 104.17.142.212 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 486 | 104.16.65.1 | 104.16.65.1 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 83 | 103.160.204.59 | 103.160.204.59 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 131 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 183 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 187 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 225 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 249 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 264 | www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 270 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 323 | silkbook.com | 172.67.75.208 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 328 | www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 341 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 344 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 374 | 172.67.79.211 | 172.67.79.211 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 382 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 395 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 480 | 104.31.16.158 | 104.31.16.158 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 88 | cu.877774.xyz | 104.18.42.54 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 123 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 166 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 190 | asia.877774.xyz | 104.16.211.153 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 222 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 234 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 266 | www.glassdoor.com | 104.17.64.70 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 283 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 284 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 308 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 310 | ip.gs | 172.67.160.28 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 316 | singapore.com | 172.67.75.194 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 342 | cf.877774.xyz | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 343 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 365 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 379 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 387 | icook.tw | 104.20.28.74 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 401 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 408 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 436 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 460 | 162.159.140.116 | 162.159.140.116 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 485 | 104.18.189.153 | 104.18.189.153 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 5 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 45 | ipinfo.in | 172.67.198.203 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 52 | steamdb.info | 172.66.175.250 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 70 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 97 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 98 | icook.hk | 172.67.161.104 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 125 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 128 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 129 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 156 | zread.ai | 172.67.202.78 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 247 | fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 280 | time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 314 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 319 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 333 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 359 | dnschecker.org | 104.26.6.89 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 362 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 370 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 378 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 399 | eur.877774.xyz | 104.21.26.150 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 442 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 463 | 104.18.166.129 | 104.18.166.129 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 475 | 104.19.220.22 | 104.19.220.22 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 482 | 104.18.223.253 | 104.18.223.253 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 491 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 493 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 65 | 104.18.37.13 | 104.18.37.13 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 95 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 112 | www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 199 | www.whatismyip.com | 104.26.13.23 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 226 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 238 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 239 | cf.zhetengsha.eu.org | 104.18.43.174 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 248 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 262 | www.ipchicken.com | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 272 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 302 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 317 | singapore.com | 104.26.13.140 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 320 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 356 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 363 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 372 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 396 | 104.19.223.58 | 104.19.223.58 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 423 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 428 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 435 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 441 | 172.64.91.69 | 172.64.91.69 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 450 | 173.245.49.194 | 173.245.49.194 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 494 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 9 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 36 | ct.877774.xyz | 172.64.229.44 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 48 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 80 | www.gov.ua | 172.67.209.127 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 113 | www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 122 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 168 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 170 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 188 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 204 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 231 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 232 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 285 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 293 | palera.in | 104.21.58.72 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 330 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 351 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 364 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 381 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 389 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 394 | 104.18.78.214 | 104.18.78.214 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 420 | www.7749tv.com | 104.17.196.161 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 431 | 198.41.208.15 | 198.41.208.15 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 447 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 452 | 162.159.136.89 | 162.159.136.89 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 18 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 56 | 104.18.39.196 | 104.18.39.196 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 89 | www.4chan.org | 104.16.228.229 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 100 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 124 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 165 | toy-people.com | 104.26.2.36 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 243 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 297 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 318 | singapore.com | 104.26.12.140 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 321 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 335 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 339 | 162.159.133.85 | 162.159.133.85 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 430 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 459 | 104.26.8.117 | 104.26.8.117 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 487 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 490 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 6 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 38 | ct.877774.xyz | 172.64.229.173 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 107 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 130 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 164 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 177 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 25 条记录
- **快 (50-100ms)**: 175 条记录
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
