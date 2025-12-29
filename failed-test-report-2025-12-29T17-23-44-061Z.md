# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:23:44
- **数据来源**: connectivity_results-20251229-172343.json
- **总测试数**: 494
- **失败测试数**: 2
- **成功测试数**: 492
- **失败率**: 0.40%
- **平均延迟**: 68.28ms
- **最小延迟**: 35ms
- **最大延迟**: 745ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:23:44
- **IP地址**: 2a09:bac5:7973:1cd2::2df:10c
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 41.8874, -87.6318
- **时区**: America/Chicago
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
| 20 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |
| 332 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |

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
| 293 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 380 | www.visa.com.hk | 104.18.21.69 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 287 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 315 | ipinfo.in | 104.21.21.129 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 350 | ct.877774.xyz | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 364 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 110 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 425 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 219 | cf.zhetengsha.eu.org | 104.18.35.15 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 348 | ct.877774.xyz | 172.64.229.173 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 360 | iplocation.io | 104.26.10.222 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 391 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 114 | japan.com | 104.26.5.60 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 156 | 104.17.167.134 | 104.17.167.134 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 173 | asia.877774.xyz | 104.17.142.146 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 227 | xn--b6gac.eu.org | 188.114.96.3 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 320 | steamdb.info | 104.20.34.212 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 29 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 88 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 135 | 104.18.81.19 | 104.18.81.19 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 178 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 261 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 285 | ip.gs | 104.21.14.176 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 294 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 297 | silkbook.com | 104.26.8.160 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 309 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 321 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 329 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 430 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 446 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 474 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 71 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 82 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 84 | 172.67.181.209 | 172.67.181.209 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 155 | 104.31.16.158 | 104.31.16.158 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 172 | asia.877774.xyz | 104.16.211.153 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 193 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 326 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 340 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 397 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 454 | cf.877771.xyz | 172.67.152.183 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 96 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 97 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 116 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 175 | zread.ai | 172.67.202.78 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 209 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 317 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 339 | www.gov.ua | 172.67.209.127 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 345 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 351 | ct.877774.xyz | 172.64.229.195 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 372 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 388 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 390 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 431 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 455 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 473 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 480 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 491 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 62 | eur.877774.xyz | 104.21.47.209 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 117 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 122 | 172.64.91.69 | 172.64.91.69 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 184 | bestcf.030101.xyz | 104.19.147.41 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 191 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 264 | palera.in | 172.67.157.122 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 312 | cf.0sm.com | 104.21.7.133 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 313 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 322 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 369 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 396 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 411 | www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 424 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 460 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 476 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 6 | 162.159.133.85 | 162.159.133.85 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 13 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 49 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 55 | www.digitalocean.com | 104.19.174.68 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 95 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 120 | 162.159.61.183 | 162.159.61.183 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 211 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 233 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 248 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 382 | www.okcupid.com | 104.16.223.254 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 389 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 395 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 409 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 412 | www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 414 | www.visa.cn | 162.159.153.2 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 489 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 18 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 74 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 90 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 115 | japan.com | 172.67.70.92 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 125 | 104.17.69.244 | 104.17.69.244 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 171 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 189 | time.is | 104.26.13.54 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 204 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 235 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 237 | cf.090227.xyz | 172.64.144.82 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 247 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 263 | palera.in | 104.21.58.72 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 283 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 291 | singapore.com | 104.26.13.140 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 338 | www.gov.ua | 104.21.23.72 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 362 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 383 | www.okcupid.com | 104.16.239.254 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 402 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 433 | freeyx.cloudflare88.eu.org | 2606:4700:3010:bf:5dba:fabf:8068:e072 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 434 | na.877774.xyz | 104.19.74.233 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 456 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 469 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 46 | 198.62.62.4 | 198.62.62.4 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 48 | icook.tw | 104.20.28.74 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 51 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 56 | www.digitalocean.com | 104.19.173.68 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 79 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 132 | 162.159.128.253 | 162.159.128.253 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 134 | 104.26.8.117 | 104.26.8.117 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 149 | 104.17.139.37 | 104.17.139.37 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 177 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 179 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 203 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 206 | www.whatismyip.com | 104.26.13.23 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 221 | cf.zhetengsha.eu.org | 2606:4700:440a::ac40:98f1 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 224 | saas.sin.fan | 162.159.36.20 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 276 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 300 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 310 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 341 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 353 | 104.18.254.88 | 104.18.254.88 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 381 | www.okcupid.com | 104.18.160.63 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 386 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 392 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 393 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 466 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 12 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 50 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 57 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 63 | eur.877774.xyz | 104.21.26.150 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 154 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 181 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 222 | 172.64.151.55 | 172.64.151.55 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 226 | xn--b6gac.eu.org | 188.114.97.3 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 238 | cf.090227.xyz | 2a06:98c1:3101::ac40:919e | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 334 | shopify.com | 23.227.38.33 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 357 | cu.877774.xyz | 104.18.42.54 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 385 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 420 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 429 | cf.877774.xyz | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 432 | freeyx.cloudflare88.eu.org | 141.101.120.176 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 478 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 487 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 66 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 139 | 198.41.208.224 | 198.41.208.224 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 158 | 104.16.105.166 | 104.16.105.166 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 188 | time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 192 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 197 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 215 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 228 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 229 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 254 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 270 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 280 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 286 | ip.gs | 172.67.160.28 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 306 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 428 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 27 | tasteatlas.com | 104.17.37.105 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 44 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 67 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 68 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 126 | 104.26.4.90 | 104.26.4.90 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 127 | 162.159.136.89 | 162.159.136.89 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 207 | www.whatismyip.com | 104.26.12.23 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 260 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 262 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 290 | singapore.com | 104.26.12.140 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 296 | silkbook.com | 172.67.75.208 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 328 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 347 | ct.877774.xyz | 172.64.229.161 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 352 | 103.160.204.59 | 103.160.204.59 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 355 | 172.67.49.134 | 172.67.49.134 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 422 | dnschecker.org | 104.26.7.89 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 7 | 104.18.14.76 | 104.18.14.76 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 22 | www.udemy.com | 104.16.142.237 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 32 | toy-people.com | 104.26.2.36 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 100 | www.csgo.com | 195.85.59.161 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 137 | 104.18.89.52 | 104.18.89.52 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 166 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 167 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 176 | zread.ai | 104.21.76.240 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 234 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 265 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 295 | silkbook.com | 104.26.9.160 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 323 | 104.18.39.196 | 104.18.39.196 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 349 | ct.877774.xyz | 172.64.229.174 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 384 | www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 410 | 172.67.120.0 | 172.67.120.0 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 421 | dnschecker.org | 104.26.6.89 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 436 | na.877774.xyz | 104.18.187.25 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 135 条记录
- **快 (50-100ms)**: 65 条记录
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
