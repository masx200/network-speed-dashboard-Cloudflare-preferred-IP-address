# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:24:01
- **数据来源**: connectivity_results-20251229-172400.json
- **总测试数**: 495
- **失败测试数**: 3
- **成功测试数**: 492
- **失败率**: 0.61%
- **平均延迟**: 72.11ms
- **最小延迟**: 32ms
- **最大延迟**: 2625ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:24:01
- **IP地址**: 2a09:bac1:76c1:6298::23c:a2
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 38.6877, -77.8369
- **时区**: America/New_York
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 3 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 41 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 325 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | none | N/A | 0 | N/A | dial tcp 108.162.195.173:443: i/o timeout |
| 335 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 3 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 3 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: 所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好



---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 157 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 375 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 178 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 124 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 259 | ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 69 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 72 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 128 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 281 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 304 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 313 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 352 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 460 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 26 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 47 | cu.877774.xyz | 172.64.145.202 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 84 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 120 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 136 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 171 | cf.zhetengsha.eu.org | 2606:4700:440a::ac40:98f1 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 227 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 270 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 274 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 289 | 104.18.42.26 | 104.18.42.26 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 394 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 416 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 49 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 71 | 172.67.243.218 | 172.67.243.218 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 86 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 150 | asia.877774.xyz | 104.16.211.153 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 234 | 104.16.61.163 | 104.16.61.163 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 239 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 267 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 288 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 305 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 396 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 450 | 104.26.8.117 | 104.26.8.117 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 451 | 162.159.140.116 | 162.159.140.116 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 15 | icook.hk | 2a06:98c1:3121::3 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 27 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 59 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 80 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 82 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 126 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 162 | saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 165 | bestcf.030101.xyz | 2606:4700:0:e7ac:854f:c15c:d3b1:fc6a | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 176 | fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 179 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 185 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 211 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 218 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 252 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 278 | silkbook.com | 172.67.75.208 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 282 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 295 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 334 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 377 | ipinfo.in | 172.67.198.203 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 378 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 390 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 418 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 424 | ct.877774.xyz | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 453 | 198.41.208.224 | 198.41.208.224 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 476 | 104.17.167.134 | 104.17.167.134 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 7 | www.ipget.net | 172.67.207.26 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 22 | iplocation.io | 104.26.10.222 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 70 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 81 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 181 | cf.090227.xyz | 104.18.43.174 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 219 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 238 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 248 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 280 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 321 | tasteatlas.com | 104.17.37.105 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 336 | 104.17.142.12 | 104.17.142.12 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 351 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 368 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 399 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 407 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 462 | 104.16.45.84 | 104.16.45.84 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 475 | 104.31.16.158 | 104.31.16.158 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 489 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 9 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 18 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 32 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 53 | www.visa.cn | 162.159.152.2 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 168 | cf.zhetengsha.eu.org | 172.64.152.241 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 192 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 208 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 262 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 285 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 298 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 306 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 330 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 337 | 172.67.79.211 | 172.67.79.211 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 345 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 372 | www.pcmag.com | 104.16.20.118 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 379 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 389 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 405 | steamdb.info | 172.66.175.250 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 414 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 425 | ct.877774.xyz | 172.64.229.195 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 448 | 104.17.69.244 | 104.17.69.244 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 13 | icook.hk | 104.21.90.210 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 14 | icook.hk | 2a06:98c1:3120::3 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 114 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 129 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 182 | cf.090227.xyz | 172.64.144.82 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 225 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 263 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 303 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 328 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 454 | 104.19.148.121 | 104.19.148.121 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 466 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 470 | shopify.com | 23.227.38.33 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 478 | 104.16.105.166 | 104.16.105.166 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 481 | 104.16.65.1 | 104.16.65.1 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 5 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 56 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 83 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 101 | na.877774.xyz | 104.19.74.233 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 116 | toy-people.com | 104.26.2.36 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 143 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 151 | asia.877774.xyz | 104.17.142.146 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 170 | cf.zhetengsha.eu.org | 2a06:98c1:3105::6812:230f | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 193 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 203 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 205 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 209 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 226 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 237 | time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 268 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 276 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 287 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 301 | dnschecker.org | 172.67.73.216 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 384 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 456 | 104.18.89.52 | 104.18.89.52 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 477 | 104.18.223.253 | 104.18.223.253 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 483 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 25 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 55 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 79 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 121 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 140 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 141 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 251 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 257 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 264 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 279 | silkbook.com | 104.26.8.160 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 290 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 361 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 421 | ct.877774.xyz | 172.64.229.161 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 432 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 443 | 104.18.166.129 | 104.18.166.129 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 10 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 16 | comicabc.com | 172.67.174.21 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 19 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 24 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 60 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 62 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 76 | www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 77 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 92 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 163 | bestcf.030101.xyz | 198.41.209.230 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 201 | www.glassdoor.com | 104.17.64.70 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 231 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 232 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 235 | time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 240 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 260 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 275 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 331 | www.udemy.com | 104.16.142.237 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 353 | eur.877774.xyz | 104.21.26.150 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 357 | 104.19.223.58 | 104.19.223.58 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 367 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 370 | ifconfig.co | 172.67.168.106 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 376 | ipinfo.in | 104.21.21.129 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 391 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 392 | japan.com | 172.67.70.92 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 411 | www.gov.ua | 172.67.209.127 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 486 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 493 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 91 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 184 | cf.090227.xyz | 2a06:98c1:3108::6812:2a62 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 187 | www.ipchicken.com | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 197 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 202 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 207 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 210 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 221 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 300 | dnschecker.org | 104.26.6.89 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 317 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 324 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 343 | icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 349 | www.digitalocean.com | 104.19.174.68 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 385 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 410 | 172.64.91.69 | 172.64.91.69 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 446 | 162.159.137.204 | 162.159.137.204 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 488 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 43 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 54 | www.visa.cn | 162.159.153.2 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 57 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 137 条记录
- **快 (50-100ms)**: 63 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 3 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 3 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*
