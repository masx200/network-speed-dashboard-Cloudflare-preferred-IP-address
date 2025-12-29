# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:07:51
- **数据来源**: connectivity_results-20251229-170751.json
- **总测试数**: 494
- **失败测试数**: 2
- **成功测试数**: 492
- **失败率**: 0.40%
- **平均延迟**: 74.57ms
- **最小延迟**: 35ms
- **最大延迟**: 851ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:07:51
- **IP地址**: 2a09:bac1:76a0:c8::3c0:62
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
| 102 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 337 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

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
| 396 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 279 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 395 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 113 | na.877774.xyz | 104.18.187.25 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 187 | fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 345 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 179 | cf.zhetengsha.eu.org | 104.18.43.174 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 260 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 286 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 370 | ifconfig.co | 172.67.168.106 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 492 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 493 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 62 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 95 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 233 | palera.in | 188.114.96.3 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 287 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 325 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 329 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 393 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 476 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 37 | comicabc.com | 2a06:98c1:3120::3 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 131 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 169 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 228 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 247 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 270 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 282 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 284 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 290 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 317 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 342 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 394 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 415 | 162.159.61.183 | 162.159.61.183 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 460 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 467 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 488 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 86 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 87 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 167 | www.whatismyip.com | 172.67.69.129 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 168 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 170 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 249 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 349 | 198.62.62.4 | 198.62.62.4 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 414 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 430 | 104.18.81.19 | 104.18.81.19 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 472 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 20 | icook.hk | 104.21.90.210 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 38 | comicabc.com | 2a06:98c1:3121::3 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 254 | singapore.com | 172.67.75.194 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 300 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 368 | eur.877774.xyz | 104.21.29.164 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 479 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 42 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 52 | www.ipget.net | 172.67.207.26 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 59 | ct.877774.xyz | 172.64.229.173 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 67 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 97 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 129 | toy-people.com | 104.26.3.36 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 132 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 151 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 153 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 162 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 164 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 165 | www.whatismyip.com | 104.26.13.23 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 177 | saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 258 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 259 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 285 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 289 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 302 | 172.67.106.26 | 172.67.106.26 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 347 | 104.17.142.12 | 104.17.142.12 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 363 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 366 | eur.877774.xyz | 104.21.47.209 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 406 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 412 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 455 | 104.16.65.1 | 104.16.65.1 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 458 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 461 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 489 | iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 40 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 84 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 100 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 136 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 157 | asia.877774.xyz | 104.17.139.62 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 211 | www.glassdoor.com | 104.16.25.46 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 223 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 235 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 255 | singapore.com | 104.26.12.140 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 272 | www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 299 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 315 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 344 | www.udemy.com | 104.16.142.237 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 355 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 407 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 462 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 487 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 8 | shopify.com | 23.227.38.33 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 22 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 28 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 48 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 77 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 90 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 92 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 125 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 127 | toy-people.com | 104.26.2.36 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 128 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 143 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 144 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 163 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 166 | www.whatismyip.com | 104.26.12.23 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 183 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 189 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 224 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 244 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 266 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 276 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 293 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 341 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 352 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 362 | www.digitalocean.com | 104.19.173.68 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 409 | japan.com | 104.26.5.60 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 452 | 104.16.105.166 | 104.16.105.166 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 468 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 475 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 481 | cf.0sm.com | 104.21.7.133 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 494 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 9 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 19 | icook.hk | 172.67.161.104 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 34 | www.okcupid.com | 104.16.239.254 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 36 | comicabc.com | 104.21.64.10 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 91 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 98 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 111 | freeyx.cloudflare88.eu.org | 2606:4700:3009:aa59:4b67:100d:4f66:caf5 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 149 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 182 | cf.zhetengsha.eu.org | 2606:4700:440a::ac40:98f1 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 194 | cf.090227.xyz | 2a06:98c1:3108::6812:2a62 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 209 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 222 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 231 | 104.16.61.163 | 104.16.61.163 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 243 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 267 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 268 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 295 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 297 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 312 | dnschecker.org | 172.67.73.216 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 354 | icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 413 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 447 | 104.19.212.207 | 104.19.212.207 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 478 | ipinfo.in | 104.21.21.129 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 480 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 61 | ct.877774.xyz | 172.64.229.44 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 81 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 89 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 118 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 140 | zread.ai | 104.21.76.240 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 218 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 229 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 257 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 265 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 291 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 294 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 309 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 310 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 316 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 324 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 330 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 353 | icook.tw | 104.20.28.74 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 356 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 405 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 58 | ct.877774.xyz | 172.64.229.174 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 63 | ct.877774.xyz | 172.64.229.217 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 70 | www.visa.cn | 162.159.153.2 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 79 | cf.877771.xyz | 172.67.152.183 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 142 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 172 | bestcf.030101.xyz | 104.19.47.227 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 202 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 230 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 271 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 275 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 292 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 301 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 350 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 371 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 426 | 162.159.128.253 | 162.159.128.253 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 459 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 484 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 11 | 104.18.254.88 | 104.18.254.88 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 30 | www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 44 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 56 | ct.877774.xyz | 172.64.229.195 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 96 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 152 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 181 | cf.zhetengsha.eu.org | 2a06:98c1:3105::6812:230f | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 186 | xn--b6gac.eu.org | 2a06:98c1:3120::3 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 204 | www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 250 | ip.gs | 104.21.14.176 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 253 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 269 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 273 | www.visa.com.hk | 104.18.21.69 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 277 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 126 条记录
- **快 (50-100ms)**: 74 条记录
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
