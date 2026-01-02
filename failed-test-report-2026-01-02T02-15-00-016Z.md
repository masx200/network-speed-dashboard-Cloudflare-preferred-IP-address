# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/1/2 02:15:00
- **数据来源**: connectivity_results-20260102-021459.json
- **总测试数**: 728
- **失败测试数**: 2
- **成功测试数**: 726
- **失败率**: 0.27%
- **平均延迟**: 63.53ms
- **最小延迟**: 43ms
- **最大延迟**: 501ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/1/2 02:15:00
- **IP地址**: 2a09:bac1:7680:28::4:338
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
| 169 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |
| 437 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 2 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 172.64（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 2 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: 所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好



---

## 🚀 延迟最低的 726 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 635 | 172.64.153.141 | 172.64.153.141 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 641 | 104.20.21.147 | 104.20.21.147 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 36 | cf.090227.xyz | 104.18.43.174 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 245 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 543 | cu.877774.xyz | 104.26.4.111 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 647 | 104.20.25.82 | 104.20.25.82 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 96 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 117 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 177 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 294 | 104.18.255.167 | 104.18.255.167 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 443 | www.pcmag.com | 104.16.21.118 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 468 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 561 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 565 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 575 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 576 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 581 | toy-people.com | 104.26.2.36 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 606 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 638 | 104.18.42.61 | 104.18.42.61 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 654 | 104.17.215.66 | 104.17.215.66 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 14 | www.ipget.net | 2a06:98c1:3120::3 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 51 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 108 | singapore.com | 104.26.12.140 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 136 | dnschecker.org | 104.26.7.89 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 214 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 257 | 162.159.61.183 | 162.159.61.183 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 302 | 104.18.189.153 | 104.18.189.153 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 338 | 172.64.229.191 | 172.64.229.191 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 340 | 104.16.255.1 | 104.16.255.1 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 500 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 520 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 539 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 542 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 550 | cu.877774.xyz | 104.26.4.118 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 556 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 640 | 104.26.2.2 | 104.26.2.2 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 703 | 104.18.95.45 | 104.18.95.45 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 42 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 49 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 64 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 92 | ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 107 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 109 | singapore.com | 104.26.13.140 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 167 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 276 | 104.26.8.117 | 104.26.8.117 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 345 | 104.17.105.198 | 104.17.105.198 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 366 | 104.20.26.58 | 104.20.26.58 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 404 | 104.26.4.44 | 104.26.4.44 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 453 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 460 | 104.18.39.196 | 104.18.39.196 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 526 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 580 | toy-people.com | 104.26.3.36 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 582 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 653 | 104.19.34.231 | 104.19.34.231 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 705 | 104.19.54.245 | 104.19.54.245 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 17 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 31 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 39 | cf.090227.xyz | 2a06:98c1:3108::6812:2a62 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 40 | www.ipchicken.com | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 47 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 67 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 88 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 140 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 149 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 163 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 172 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 252 | 172.64.52.127 | 172.64.52.127 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 258 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 286 | 104.19.220.22 | 104.19.220.22 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 328 | 172.67.74.57 | 172.67.74.57 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 403 | 104.26.8.192 | 104.26.8.192 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 406 | 104.17.24.232 | 104.17.24.232 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 444 | www.pcmag.com | 104.16.20.118 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 469 | shopify.com | 23.227.38.33 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 476 | 104.18.37.13 | 104.18.37.13 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 477 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 496 | icook.hk | 172.67.161.104 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 498 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 513 | www.visa.cn | 162.159.153.2 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 531 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 532 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 549 | cu.877774.xyz | 104.26.4.117 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 567 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 584 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 594 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 626 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 630 | bestcf.030101.xyz | 2606:4700::254f:35ca:8fda:9c69 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 645 | 104.26.1.181 | 104.26.1.181 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 651 | 104.17.170.137 | 104.17.170.137 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 655 | 104.19.153.47 | 104.19.153.47 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 669 | 162.159.61.106 | 162.159.61.106 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 671 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 684 | 104.18.44.178 | 104.18.44.178 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 690 | 104.26.3.78 | 104.26.3.78 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 707 | 104.17.209.156 | 104.17.209.156 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 720 | 2a06:98c1:3108:69e8:8084:4e8:69fb:42f3 | 2a06:98c1:3108:69e8:8084:4e8:69fb:42f3 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 10 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 21 | cf.zhetengsha.eu.org | 2606:4700:4407::ac40:9052 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 86 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 112 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 146 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 156 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 158 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 170 | 172.67.79.211 | 172.67.79.211 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 171 | 198.62.62.4 | 198.62.62.4 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 210 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 234 | 104.19.175.123 | 104.19.175.123 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 242 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 243 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 264 | 172.64.91.69 | 172.64.91.69 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 277 | 162.159.140.116 | 162.159.140.116 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 293 | 104.19.154.200 | 104.19.154.200 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 296 | 104.17.69.244 | 104.17.69.244 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 301 | 104.17.142.212 | 104.17.142.212 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 315 | 162.159.21.116 | 162.159.21.116 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 327 | 104.26.15.85 | 104.26.15.85 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 341 | 104.17.168.159 | 104.17.168.159 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 343 | 104.17.170.110 | 104.17.170.110 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 421 | 162.159.39.99 | 162.159.39.99 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 448 | steamdb.info | 104.20.34.212 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 461 | 104.16.45.84 | 104.16.45.84 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 501 | iplocation.io | 104.26.10.222 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 512 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 515 | www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 518 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 559 | na.877774.xyz | 104.19.74.233 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 595 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 622 | www.whatismyip.com | 104.26.12.23 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 633 | 172.64.145.119 | 172.64.145.119 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 646 | 172.67.75.11 | 172.67.75.11 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 659 | 104.17.189.30 | 104.17.189.30 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 665 | 162.159.38.67 | 162.159.38.67 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 675 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 688 | 104.18.35.200 | 104.18.35.200 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 714 | 162.159.38.238 | 162.159.38.238 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 13 | www.ipget.net | 2a06:98c1:3121::3 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 18 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 22 | cf.zhetengsha.eu.org | 2a06:98c1:310d::6812:2bae | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 25 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 54 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 103 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 120 | 104.18.42.26 | 104.18.42.26 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 122 | www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 127 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 134 | dnschecker.org | 104.26.6.89 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 148 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 155 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 157 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 178 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 211 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 216 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 221 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 229 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 244 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 263 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 272 | 104.26.5.134 | 104.26.5.134 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 283 | 104.17.162.3 | 104.17.162.3 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 295 | 104.19.212.207 | 104.19.212.207 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 325 | 172.64.150.30 | 172.64.150.30 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 342 | 104.17.16.248 | 104.17.16.248 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 373 | 104.20.18.47 | 104.20.18.47 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 383 | 104.17.100.254 | 104.17.100.254 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 394 | 162.159.19.37 | 162.159.19.37 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 399 | 172.67.68.252 | 172.67.68.252 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 422 | 162.159.7.12 | 162.159.7.12 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 433 | 172.67.49.134 | 172.67.49.134 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 456 | www.gov.ua | 104.21.23.72 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 475 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 499 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 519 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 527 | 172.67.243.218 | 172.67.243.218 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 546 | cu.877774.xyz | 104.26.4.114 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 557 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 568 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 588 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 607 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 624 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 636 | 172.64.146.67 | 172.64.146.67 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 642 | 104.26.5.101 | 104.26.5.101 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 681 | 173.245.59.26 | 173.245.59.26 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 689 | 104.18.47.169 | 104.18.47.169 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 704 | 104.19.48.127 | 104.19.48.127 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 718 | 162.159.44.20 | 162.159.44.20 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 8 | comicabc.com | 104.21.64.10 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 11 | www.ipget.net | 172.67.207.26 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 12 | www.ipget.net | 104.21.15.212 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 15 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 37 | cf.090227.xyz | 172.64.144.82 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 52 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 58 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 65 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 73 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 100 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 101 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 128 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 137 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 142 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 176 | icook.tw | 104.20.28.74 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 182 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 188 | eur.877774.xyz | 104.21.29.164 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 191 | 104.19.223.58 | 104.19.223.58 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 209 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 220 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 224 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 231 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 238 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 240 | 198.41.194.162 | 198.41.194.162 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 260 | japan.com | 104.26.5.60 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 262 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 278 | 104.18.81.19 | 104.18.81.19 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 300 | 104.16.105.166 | 104.16.105.166 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 320 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 334 | 172.67.72.254 | 172.67.72.254 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 371 | 104.20.20.42 | 104.20.20.42 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 372 | 104.26.4.190 | 104.26.4.190 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 384 | 104.17.101.208 | 104.17.101.208 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 401 | 104.20.17.233 | 104.20.17.233 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 414 | 104.17.21.106 | 104.17.21.106 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 431 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 442 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 458 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 484 | ct.877774.xyz | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 488 | ct.877774.xyz | 172.64.229.44 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 489 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 522 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 540 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 554 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 591 | zread.ai | 172.67.202.78 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 592 | zread.ai | 104.21.76.240 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 598 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 601 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 621 | www.whatismyip.com | 172.67.69.129 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 625 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 649 | 104.26.10.239 | 104.26.10.239 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 658 | 104.17.25.241 | 104.17.25.241 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 676 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 694 | 104.20.19.4 | 104.20.19.4 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 706 | 104.17.154.94 | 104.17.154.94 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 712 | 172.64.229.2 | 172.64.229.2 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 719 | 172.64.53.222 | 172.64.53.222 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 6 | saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 38 | cf.090227.xyz | 2a06:98c1:3101::ac40:919e | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 53 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 63 | time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 80 | palera.in | 104.21.58.72 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 90 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 95 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 105 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 173 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 189 | eur.877774.xyz | 104.21.47.209 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 213 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 226 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 241 | 198.41.208.15 | 198.41.208.15 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 253 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 304 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 307 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 316 | 162.159.6.115 | 162.159.6.115 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 324 | 104.18.45.95 | 104.18.45.95 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 331 | 104.20.24.107 | 104.20.24.107 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 333 | 104.20.30.198 | 104.20.30.198 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 375 | 104.20.19.201 | 104.20.19.201 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 402 | 172.67.70.253 | 172.67.70.253 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 409 | 104.16.147.114 | 104.16.147.114 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 436 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 462 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 466 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 479 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 482 | 104.18.254.88 | 104.18.254.88 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 504 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 506 | 172.67.120.0 | 172.67.120.0 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 516 | www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 524 | cf.877771.xyz | 172.67.152.183 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 535 | www.okcupid.com | 104.18.160.63 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 536 | www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 537 | www.okcupid.com | 104.16.239.254 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 538 | www.okcupid.com | 104.16.223.254 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 569 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 578 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 585 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 639 | 104.18.40.216 | 104.18.40.216 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 652 | 104.17.115.224 | 104.17.115.224 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 656 | 104.17.209.79 | 104.17.209.79 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 657 | 104.19.35.242 | 104.19.35.242 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 678 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 685 | 172.64.146.218 | 172.64.146.218 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 698 | 172.67.76.194 | 172.67.76.194 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 700 | 104.16.155.38 | 104.16.155.38 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 5 | saas.sin.fan | 162.159.36.20 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 26 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 60 | www.glassdoor.com | 104.17.64.70 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 83 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 93 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 106 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 174 | 172.64.35.24 | 172.64.35.24 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 181 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 187 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 198 | ifconfig.co | 172.67.168.106 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 256 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 288 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 326 | 172.67.68.211 | 172.67.68.211 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 337 | 104.17.101.37 | 104.17.101.37 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 339 | 104.17.156.81 | 104.17.156.81 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 381 | 104.19.50.35 | 104.19.50.35 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 387 | 104.18.35.166 | 104.18.35.166 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 397 | 104.26.2.166 | 104.26.2.166 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 413 | 104.16.251.143 | 104.16.251.143 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 478 | 172.67.110.232 | 172.67.110.232 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 487 | ct.877774.xyz | 172.64.229.195 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 492 | www.4chan.org | 104.16.229.229 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 493 | freeyx.cloudflare88.eu.org | 141.101.121.81 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 495 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 503 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 525 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 544 | cu.877774.xyz | 104.26.4.112 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 610 | asia.877774.xyz | 104.17.139.62 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 616 | 172.64.53.181 | 172.64.53.181 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 643 | 172.67.73.120 | 172.67.73.120 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 683 | 104.18.36.213 | 104.18.36.213 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 691 | 104.26.14.114 | 104.26.14.114 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 692 | 104.20.18.138 | 104.20.18.138 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 7 | comicabc.com | 172.67.174.21 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 20 | cf.zhetengsha.eu.org | 104.18.42.98 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 57 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 87 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 119 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 144 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 192 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 207 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 227 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 228 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 251 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 255 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 284 | 104.18.151.172 | 104.18.151.172 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 290 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 303 | 104.16.65.1 | 104.16.65.1 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 309 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 311 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 330 | 172.67.65.159 | 172.67.65.159 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 344 | 104.18.39.15 | 104.18.39.15 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 356 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 361 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 363 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 369 | 172.67.67.5 | 172.67.67.5 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 379 | 104.18.40.39 | 104.18.40.39 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 385 | 104.16.157.50 | 104.16.157.50 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 398 | 104.20.16.244 | 104.20.16.244 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 405 | 172.67.76.195 | 172.67.76.195 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 432 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 447 | steamdb.info | 172.66.175.250 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 452 | ipinfo.in | 172.67.198.203 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 464 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 497 | icook.hk | 104.21.90.210 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 564 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 571 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 583 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 620 | 172.64.145.242 | 172.64.145.242 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 632 | 162.159.3.89 | 162.159.3.89 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 668 | 162.159.45.165 | 162.159.45.165 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 670 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 672 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 679 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 693 | 104.20.26.29 | 104.20.26.29 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 708 | 104.18.92.221 | 104.18.92.221 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 728 | 2a06:98c1:310d:0:b658:30d1:bf2:8feb | 2a06:98c1:310d:0:b658:30d1:bf2:8feb | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 30 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 55 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 56 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 79 | 104.16.61.163 | 104.16.61.163 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 110 | singapore.com | 172.67.75.194 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 139 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 175 | icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 179 | www.digitalocean.com | 104.19.174.68 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 194 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 202 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 222 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 269 | 162.159.136.89 | 162.159.136.89 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 271 | 172.64.229.7 | 172.64.229.7 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 275 | 104.26.3.162 | 104.26.3.162 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 279 | 104.18.89.52 | 104.18.89.52 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 280 | 104.18.166.129 | 104.18.166.129 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 281 | 198.41.208.224 | 198.41.208.224 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 313 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 335 | 104.26.3.176 | 104.26.3.176 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 358 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 400 | 104.26.6.159 | 104.26.6.159 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 411 | 104.17.154.254 | 104.17.154.254 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 434 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 441 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 446 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 454 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 483 | ct.877774.xyz | 172.64.229.174 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 507 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 534 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 551 | cu.877774.xyz | 104.26.4.119 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 563 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 579 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 593 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 629 | bestcf.030101.xyz | 2606:4700:0:7a67:3de4:1f89:27a3:6eeb | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 695 | 104.20.29.50 | 104.20.29.50 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 701 | 172.64.145.205 | 172.64.145.205 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 27 | 172.64.151.55 | 172.64.151.55 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 28 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 43 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 78 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 85 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 91 | ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 99 | ip.gs | 104.21.14.176 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 116 | silkbook.com | 172.67.75.208 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 118 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 154 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 159 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 199 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 212 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 236 | 172.64.82.114 | 172.64.82.114 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 254 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 265 | 172.64.48.226 | 172.64.48.226 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 299 | 104.18.223.253 | 104.18.223.253 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 306 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 310 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 355 | 162.159.39.62 | 162.159.39.62 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 360 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 377 | 104.16.148.187 | 104.16.148.187 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 395 | 162.159.62.6 | 162.159.62.6 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 408 | 104.16.153.12 | 104.16.153.12 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 491 | www.4chan.org | 104.16.228.229 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 547 | cu.877774.xyz | 104.26.4.115 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 577 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 603 | 162.159.39.177 | 162.159.39.177 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 604 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 612 | asia.877774.xyz | 104.16.211.153 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 667 | 108.162.198.148 | 108.162.198.148 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 680 | 172.64.147.232 | 172.64.147.232 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 696 | 104.26.1.232 | 104.26.1.232 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 709 | 104.17.171.35 | 104.17.171.35 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 722 | 2a06:98c1:310c:8b:38b6:2a7:3894:7893 | 2a06:98c1:310c:8b:38b6:2a7:3894:7893 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 24 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 50 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 61 | time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 82 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 104 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 165 | tasteatlas.com | 104.17.36.105 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 168 | 104.17.142.12 | 104.17.142.12 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 223 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 230 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 232 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 273 | 162.159.137.204 | 162.159.137.204 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 329 | 172.67.64.214 | 172.67.64.214 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 347 | 108.162.198.48 | 108.162.198.48 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 382 | 104.16.155.76 | 104.16.155.76 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 450 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 455 | www.gov.ua | 172.67.209.127 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 517 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 523 | cf.877771.xyz | 104.21.80.180 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 529 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 545 | cu.877774.xyz | 104.26.4.113 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 548 | cu.877774.xyz | 104.26.4.116 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 552 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 560 | na.877774.xyz | 104.18.38.235 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 566 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 650 | 104.17.211.218 | 104.17.211.218 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 702 | 104.17.174.10 | 104.17.174.10 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 726 | 2a06:98c1:310b:aa90:9623:824a:3237:5427 | 2a06:98c1:310b:aa90:9623:824a:3237:5427 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 16 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 23 | fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 66 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 123 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 124 | 172.67.106.26 | 172.67.106.26 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 126 | 162.159.133.85 | 162.159.133.85 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 166 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 185 | 104.18.78.214 | 104.18.78.214 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 186 | 162.159.140.85 | 162.159.140.85 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 204 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 219 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 225 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 259 | japan.com | 172.67.70.92 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 267 | 173.245.49.194 | 173.245.49.194 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 285 | 104.17.139.37 | 104.17.139.37 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 319 | 172.64.42.158 | 172.64.42.158 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 351 | 162.159.0.115 | 162.159.0.115 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 365 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 427 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 435 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 449 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 474 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 514 | www.visa.cn | 162.159.152.2 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 558 | na.877774.xyz | 104.18.187.25 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 572 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 644 | 104.26.3.120 | 104.26.3.120 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 663 | 162.159.39.136 | 162.159.39.136 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 699 | 172.67.69.87 | 172.67.69.87 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 723 | 2a06:98c1:310a:0:ae62:421e:28df:1252 | 2a06:98c1:310a:0:ae62:421e:28df:1252 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 102 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 135 | dnschecker.org | 172.67.73.216 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 160 | www.udemy.com | 104.16.143.237 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 162 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 180 | www.digitalocean.com | 104.19.173.68 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 239 | www.csgo.com | 195.85.59.161 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 261 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 274 | 162.159.128.253 | 162.159.128.253 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 297 | 104.31.16.158 | 104.31.16.158 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 298 | 104.17.167.134 | 104.17.167.134 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 336 | 104.17.169.180 | 104.17.169.180 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 357 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 376 | 104.17.60.233 | 104.17.60.233 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 465 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 490 | ct.877774.xyz | 172.64.229.173 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 587 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 611 | asia.877774.xyz | 104.17.142.146 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 634 | 162.159.1.111 | 162.159.1.111 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 697 | 104.20.20.225 | 104.20.20.225 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 45 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 81 | palera.in | 172.67.157.122 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 121 | www.visa.com.hk | 104.18.21.69 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 125 | 104.18.14.76 | 104.18.14.76 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 152 | cf.877774.xyz | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 208 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 312 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 332 | 104.20.22.185 | 104.20.22.185 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 362 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 396 | 104.26.12.113 | 104.26.12.113 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 410 | 104.17.30.164 | 104.17.30.164 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 467 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 570 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 615 | 172.64.34.153 | 172.64.34.153 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 9 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 19 | cf.zhetengsha.eu.org | 172.64.145.158 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 111 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 113 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 233 | 172.67.181.209 | 172.67.181.209 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 407 | 104.17.50.237 | 104.17.50.237 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 451 | ipinfo.in | 104.21.21.129 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 457 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 664 | 172.64.53.40 | 172.64.53.40 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 687 | 104.18.33.200 | 104.18.33.200 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 59 | www.glassdoor.com | 104.16.25.46 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 97 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 114 | silkbook.com | 104.26.9.160 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 115 | silkbook.com | 104.26.8.160 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 237 | www.7749tv.com | 104.19.133.4 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 367 | 172.67.77.196 | 172.67.77.196 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 380 | 104.17.119.199 | 104.17.119.199 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 390 | 104.18.37.110 | 104.18.37.110 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 412 | 104.17.53.25 | 104.17.53.25 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 510 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 533 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 69 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 98 | ip.gs | 172.67.160.28 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 184 | 104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 196 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 268 | 104.26.4.90 | 104.26.4.90 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 308 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 368 | 104.26.8.148 | 104.26.8.148 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 445 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 505 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 617 | 108.162.198.170 | 108.162.198.170 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 628 | bestcf.030101.xyz | 104.19.145.15 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 613 | 162.159.45.145 | 162.159.45.145 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 138 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 183 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | h2 | ✅ 成功 | 65 | cloudflare |
| 415 | 104.16.144.235 | 104.16.144.235 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 619 | 172.64.151.235 | 172.64.151.235 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 132 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 153 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 215 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 305 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 627 | bestcf.030101.xyz | 104.17.31.49 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 62 | time.is | 104.26.13.54 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 72 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 190 | eur.877774.xyz | 104.21.26.150 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 321 | 104.18.47.253 | 104.18.47.253 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 429 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 440 | cf.0sm.com | 172.67.187.145 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 485 | ct.877774.xyz | 172.64.229.217 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 521 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare |
| 4 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 218 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 282 | 104.19.148.121 | 104.19.148.121 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 161 | www.udemy.com | 104.16.142.237 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 721 | 2a06:98c1:3100:6e15:b337:b7f3:946f:f64c | 2a06:98c1:3100:6e15:b337:b7f3:946f:f64c | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 41 | www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 164 | tasteatlas.com | 104.17.37.105 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 438 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 486 | ct.877774.xyz | 172.64.229.161 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 494 | freeyx.cloudflare88.eu.org | 2606:4700:3010:0:f5e8:7af2:1225:455c | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 502 | iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 541 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 193 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 439 | cf.0sm.com | 104.21.7.133 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 131 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 623 | www.whatismyip.com | 104.26.13.23 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 392 | 104.18.39.228 | 104.18.39.228 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 393 | 104.18.44.148 | 104.18.44.148 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 420 | 172.64.52.15 | 172.64.52.15 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 70 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | h2 | ✅ 成功 | 85 | cloudflare |
| 34 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | h2 | ✅ 成功 | 86 | cloudflare |
| 48 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | h2 | ✅ 成功 | 86 | cloudflare |
| 291 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | h2 | ✅ 成功 | 86 | cloudflare |
| 589 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | h2 | ✅ 成功 | 86 | cloudflare |
| 713 | 172.64.38.159 | 172.64.38.159 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 715 | 162.159.10.211 | 162.159.10.211 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 76 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | h2 | ✅ 成功 | 87 | cloudflare |
| 94 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 87 | cloudflare |
| 249 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | h2 | ✅ 成功 | 87 | cloudflare |
| 292 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | h2 | ✅ 成功 | 87 | cloudflare |
| 370 | 104.20.21.161 | 104.20.21.161 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 605 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 206 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | h2 | ✅ 成功 | 88 | cloudflare |
| 354 | 172.64.52.110 | 172.64.52.110 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 374 | 104.26.12.227 | 104.26.12.227 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 141 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 205 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | h2 | ✅ 成功 | 89 | cloudflare |
| 352 | 108.162.194.125 | 108.162.194.125 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 353 | 162.159.45.93 | 162.159.45.93 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 418 | 108.162.198.69 | 108.162.198.69 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 596 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 674 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | h2 | ✅ 成功 | 89 | cloudflare |
| 33 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | h2 | ✅ 成功 | 90 | cloudflare |
| 84 | 162.159.36.104 | 162.159.36.104 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 195 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | h2 | ✅ 成功 | 90 | cloudflare |
| 423 | 162.159.44.128 | 162.159.44.128 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 424 | 162.159.38.35 | 162.159.38.35 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 425 | 172.64.53.144 | 172.64.53.144 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 573 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 574 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 608 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | h2 | ✅ 成功 | 90 | cloudflare |
| 631 | 162.159.18.240 | 162.159.18.240 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 129 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 133 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 145 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 150 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 197 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 217 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 346 | 162.159.44.176 | 162.159.44.176 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 386 | 162.159.34.55 | 162.159.34.55 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 416 | 172.64.147.235 | 172.64.147.235 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 470 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 508 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 75 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 248 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 266 | 162.159.24.131 | 162.159.24.131 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 287 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 318 | 162.159.13.51 | 162.159.13.51 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 323 | 173.245.58.237 | 173.245.58.237 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 472 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 661 | 162.159.42.146 | 162.159.42.146 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 29 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 32 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 44 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 289 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 364 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 419 | 162.159.19.219 | 162.159.19.219 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 553 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 600 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 660 | 172.64.52.67 | 172.64.52.67 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 686 | 162.159.21.39 | 162.159.21.39 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 725 | 2a06:98c1:51:9abb:2942:bacb:7903:ccf8 | 2a06:98c1:51:9abb:2942:bacb:7903:ccf8 | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 68 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 74 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 201 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 270 | 162.159.58.65 | 162.159.58.65 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 426 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | h2 | ✅ 成功 | 94 | cloudflare |
| 637 | 162.159.22.29 | 162.159.22.29 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 1 | 172.64.154.18 | 172.64.154.18 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 89 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | h2 | ✅ 成功 | 95 | cloudflare |
| 314 | 162.159.17.243 | 162.159.17.243 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 349 | 172.64.53.0 | 172.64.53.0 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 350 | 172.64.50.51 | 172.64.50.51 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 473 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | h2 | ✅ 成功 | 95 | cloudflare |
| 602 | 162.159.38.171 | 162.159.38.171 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 618 | 162.159.41.141 | 162.159.41.141 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 666 | 162.159.44.133 | 162.159.44.133 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 143 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | h2 | ✅ 成功 | 96 | cloudflare |
| 471 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 528 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 530 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 555 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | h2 | ✅ 成功 | 96 | cloudflare |
| 599 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | h2 | ✅ 成功 | 96 | cloudflare |
| 614 | 172.64.52.90 | 172.64.52.90 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 673 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | h2 | ✅ 成功 | 96 | cloudflare |
| 711 | 172.64.52.8 | 172.64.52.8 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 727 | 2606:4700:50:8972:9b9d:22a4:4f8:5a18 | 2606:4700:50:8972:9b9d:22a4:4f8:5a18 | IPv6 | h2 | ✅ 成功 | 96 | cloudflare |
| 348 | 162.159.38.192 | 162.159.38.192 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 682 | 162.159.3.40 | 162.159.3.40 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 710 | 108.162.198.173 | 108.162.198.173 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 130 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h2 | ✅ 成功 | 98 | cloudflare |
| 235 | 108.162.198.54 | 108.162.198.54 | IPv4 | h2 | ✅ 成功 | 98 | cloudflare |
| 509 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h2 | ✅ 成功 | 98 | cloudflare |
| 511 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | h2 | ✅ 成功 | 98 | cloudflare |
| 609 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | h2 | ✅ 成功 | 98 | cloudflare |
| 724 | 2400:cb00:f00e:0:f0:52d4:e4e1:bcd8 | 2400:cb00:f00e:0:f0:52d4:e4e1:bcd8 | IPv6 | h2 | ✅ 成功 | 98 | cloudflare |
| 322 | 162.159.46.238 | 162.159.46.238 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 590 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | h2 | ✅ 成功 | 99 | cloudflare |
| 147 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 151 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | h2 | ✅ 成功 | 100 | cloudflare |
| 246 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 417 | 162.159.45.176 | 162.159.45.176 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 597 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 662 | 162.159.21.222 | 162.159.21.222 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 388 | 162.159.42.140 | 162.159.42.140 | IPv4 | h2 | ✅ 成功 | 101 | cloudflare |
| 430 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | h2 | ✅ 成功 | 101 | cloudflare |
| 677 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | h2 | ✅ 成功 | 101 | cloudflare |
| 2 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 203 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 3 | 2a06:98c1:51:0:98:2c59:3632:e8dd | 2a06:98c1:51:0:98:2c59:3632:e8dd | IPv6 | h2 | ✅ 成功 | 103 | cloudflare |
| 250 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | h2 | ✅ 成功 | 103 | cloudflare |
| 317 | 108.162.192.66 | 108.162.192.66 | IPv4 | h2 | ✅ 成功 | 103 | cloudflare |
| 247 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h2 | ✅ 成功 | 104 | cloudflare |
| 716 | 162.159.39.246 | 162.159.39.246 | IPv4 | h2 | ✅ 成功 | 104 | cloudflare |
| 717 | 162.159.45.134 | 162.159.45.134 | IPv4 | h2 | ✅ 成功 | 104 | cloudflare |
| 389 | 108.162.195.1 | 108.162.195.1 | IPv4 | h2 | ✅ 成功 | 105 | cloudflare |
| 562 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h2 | ✅ 成功 | 105 | cloudflare |
| 586 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 71 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | h2 | ✅ 成功 | 108 | cloudflare |
| 391 | 162.159.36.26 | 162.159.36.26 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 463 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 46 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | h2 | ✅ 成功 | 112 | cloudflare |
| 359 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | h2 | ✅ 成功 | 112 | cloudflare |
| 378 | 104.19.44.238 | 104.19.44.238 | IPv4 | h2 | ✅ 成功 | 120 | cloudflare |
| 480 | 103.160.204.59 | 103.160.204.59 | IPv4 | h2 | ✅ 成功 | 124 | cloudflare |
| 648 | 172.67.72.36 | 172.67.72.36 | IPv4 | h2 | ✅ 成功 | 128 | cloudflare |
| 77 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | h2 | ✅ 成功 | 142 | cloudflare |
| 428 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | h2 | ✅ 成功 | 179 | cloudflare |
| 35 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 301 | cloudflare |
| 459 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 446 | cloudflare |
| 200 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 450 | cloudflare |
| 481 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 501 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 135 条记录
- **快 (50-100ms)**: 557 条记录
- **正常 (100-200ms)**: 30 条记录
- **慢 (200-500ms)**: 3 条记录
- **很慢 (>500ms)**: 1 条记录


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
