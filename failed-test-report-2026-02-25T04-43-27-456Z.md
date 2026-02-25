# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/2/25 04:43:27
- **数据来源**: connectivity_results-20260225-044326.json
- **总测试数**: 836
- **失败测试数**: 3
- **成功测试数**: 833
- **失败率**: 0.36%
- **平均延迟**: 57.39ms
- **最小延迟**: 30ms
- **最大延迟**: 1480ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/2/25 04:43:27
- **IP地址**: 2a09:bac5:7973:25a5::3c0:53
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

- **连接超时: I/O超时**: 3 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 119 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 201 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.35.202:443: i/o timeout |
| 388 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 3 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 172.64（2 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 3 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: 所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好



---

## 🚀 延迟最低的 833 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 4 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 30 | cloudflare |
| 140 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 399 | 104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 104 | www.okcupid.com | 104.16.239.254 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 110 | www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 115 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 116 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 121 | cfip.xxxxxxxx.tk | 190.93.246.67 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 260 | cf.090227.xyz | 104.18.42.98 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 299 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 312 | ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 316 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 323 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 391 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 394 | icook.tw | 104.20.28.74 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 419 | www.csgo.com | 195.85.59.161 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 532 | 104.17.156.81 | 104.17.156.81 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 756 | 104.26.8.171 | 104.26.8.171 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 757 | 104.20.20.192 | 104.20.20.192 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 46 | cf.0sm.com | 104.21.7.133 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 53 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 63 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 65 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 102 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 176 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 194 | www.whatismyip.com | 172.67.69.129 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 242 | bestcf.030101.xyz | 104.17.206.188 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 255 | fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 259 | cf.090227.xyz | 172.64.145.158 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 262 | cf.090227.xyz | 2a06:98c1:3105::6812:230f | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 297 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 311 | ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 321 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 328 | singapore.com | 172.67.75.194 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 329 | singapore.com | 104.26.13.140 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 348 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 364 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 379 | tasteatlas.com | 104.17.36.105 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 397 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 400 | 104.18.78.214 | 104.18.78.214 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 401 | www.digitalocean.com | 104.19.174.68 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 402 | www.digitalocean.com | 104.19.173.68 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 435 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 439 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 451 | japan.com | 104.26.5.60 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 468 | 162.159.137.204 | 162.159.137.204 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 504 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 535 | 104.17.168.159 | 104.17.168.159 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 542 | 104.26.4.44 | 104.26.4.44 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 722 | 104.18.47.46 | 104.18.47.46 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 763 | 104.17.193.113 | 104.17.193.113 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 12 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 16 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 31 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 40 | ct.877774.xyz | 172.64.229.161 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 42 | ipinfo.in | 172.67.198.203 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 60 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 80 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 123 | cfip.xxxxxxxx.tk | 198.41.214.141 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 139 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 142 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 149 | na.877774.xyz | 104.18.187.25 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 166 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 182 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 195 | www.whatismyip.com | 104.26.12.23 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 196 | www.whatismyip.com | 104.26.13.23 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 217 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 219 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 225 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 241 | bestcf.030101.xyz | 104.19.156.188 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 247 | cf.zhetengsha.eu.org | 104.18.42.98 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 298 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 300 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 313 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 343 | 172.67.106.26 | 172.67.106.26 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 353 | dnschecker.org | 104.26.7.89 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 355 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 356 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 368 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 380 | tasteatlas.com | 104.17.37.105 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 393 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 395 | icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 408 | eur.877774.xyz | 104.21.29.164 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 409 | eur.877774.xyz | 104.21.47.209 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 420 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 437 | 162.159.140.85 | 162.159.140.85 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 450 | 162.159.61.183 | 162.159.61.183 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 457 | 172.64.91.69 | 172.64.91.69 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 463 | 104.26.4.90 | 104.26.4.90 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 470 | 104.26.3.162 | 104.26.3.162 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 490 | 104.19.212.207 | 104.19.212.207 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 491 | 104.17.69.244 | 104.17.69.244 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 493 | 104.17.167.134 | 104.17.167.134 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 497 | 104.18.189.153 | 104.18.189.153 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 515 | 104.18.47.253 | 104.18.47.253 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 527 | 172.67.72.254 | 172.67.72.254 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 568 | 104.20.19.201 | 104.20.19.201 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 580 | 104.18.35.166 | 104.18.35.166 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 590 | 104.26.2.166 | 104.26.2.166 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 598 | 104.17.24.232 | 104.17.24.232 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 600 | 104.16.153.12 | 104.16.153.12 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 645 | 172.64.146.67 | 172.64.146.67 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 654 | 172.67.75.11 | 172.67.75.11 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 659 | 104.17.170.137 | 104.17.170.137 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 675 | 104.26.14.117 | 104.26.14.117 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 679 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 683 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 689 | 172.64.154.113 | 172.64.154.113 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 693 | 172.64.145.108 | 172.64.145.108 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 707 | 104.26.7.7 | 104.26.7.7 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 727 | 172.64.154.226 | 172.64.154.226 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 745 | 172.64.229.134 | 172.64.229.134 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 748 | 104.26.1.88 | 104.26.1.88 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 749 | 104.20.28.239 | 104.20.28.239 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 760 | 172.64.229.156 | 172.64.229.156 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 761 | 104.25.241.198 | 104.25.241.198 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 771 | 104.25.250.174 | 104.25.250.174 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 772 | 104.20.31.132 | 104.20.31.132 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 782 | 104.20.24.244 | 104.20.24.244 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 786 | 104.20.19.180 | 104.20.19.180 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 836 | 2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b | 2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 14 | www.pcmag.com | 104.16.20.118 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 15 | www.pcmag.com | 104.16.21.118 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 17 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 22 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 28 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 36 | ct.877774.xyz | 172.64.229.195 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 47 | cf.0sm.com | 172.67.187.145 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 58 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 59 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 61 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 62 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 75 | 172.67.49.134 | 172.67.49.134 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 90 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 106 | cf.877771.xyz | 104.21.80.180 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 113 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 114 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 128 | cfip.xxxxxxxx.tk | 188.114.97.144 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 141 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 150 | na.877774.xyz | 104.19.74.233 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 161 | freeyx.cloudflare88.eu.org | 2606:4700:3009:aa59:4b67:cdf7:f50d:f763 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 164 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 165 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 186 | asia.877774.xyz | 104.17.139.62 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 199 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 207 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 222 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 223 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 226 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 229 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 233 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 234 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 235 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 239 | saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 244 | bestcf.030101.xyz | 2606:4700:0:dd:df95:6eb1:ffa4:6779 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 249 | cf.zhetengsha.eu.org | 2a06:98c1:3108::6812:2a62 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 284 | www.glassdoor.com | 104.16.25.46 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 317 | ip.gs | 172.67.160.28 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 320 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 324 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 332 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 335 | silkbook.com | 104.26.8.160 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 336 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 341 | 104.18.42.26 | 104.18.42.26 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 346 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 349 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 351 | dnschecker.org | 172.67.73.216 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 377 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 386 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 403 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 411 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 428 | 104.19.175.123 | 104.19.175.123 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 434 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 441 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 452 | japan.com | 172.67.70.92 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 466 | 172.64.229.7 | 172.64.229.7 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 474 | 104.18.89.52 | 104.18.89.52 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 494 | 104.18.223.253 | 104.18.223.253 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 498 | 104.16.65.1 | 104.16.65.1 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 508 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 517 | 104.18.45.95 | 104.18.45.95 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 518 | 172.64.150.30 | 172.64.150.30 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 520 | 104.26.15.85 | 104.26.15.85 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 521 | 172.67.74.57 | 172.67.74.57 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 523 | 172.67.65.159 | 172.67.65.159 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 526 | 104.20.30.198 | 104.20.30.198 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 528 | 104.26.3.176 | 104.26.3.176 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 529 | 104.17.169.180 | 104.17.169.180 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 530 | 104.17.101.37 | 104.17.101.37 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 533 | 104.16.255.1 | 104.16.255.1 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 549 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 566 | 104.20.18.47 | 104.20.18.47 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 567 | 104.26.12.227 | 104.26.12.227 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 570 | 104.16.148.187 | 104.16.148.187 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 574 | 104.19.50.35 | 104.19.50.35 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 589 | 104.26.12.113 | 104.26.12.113 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 592 | 172.67.68.252 | 172.67.68.252 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 604 | 104.17.53.25 | 104.17.53.25 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 608 | 172.64.147.235 | 172.64.147.235 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 623 | 172.64.42.158 | 172.64.42.158 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 637 | 172.64.145.242 | 172.64.145.242 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 650 | 104.17.211.218 | 104.17.211.218 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 663 | 104.19.153.47 | 104.19.153.47 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 667 | 104.17.189.30 | 104.17.189.30 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 686 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 691 | 172.64.152.215 | 172.64.152.215 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 694 | 104.18.37.177 | 104.18.37.177 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 716 | 104.17.214.136 | 104.17.214.136 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 717 | 104.17.187.186 | 104.17.187.186 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 726 | 104.18.40.202 | 104.18.40.202 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 736 | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 750 | 104.26.6.247 | 104.26.6.247 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 753 | 104.20.25.181 | 104.20.25.181 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 754 | 172.67.65.150 | 172.67.65.150 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 759 | 104.17.97.228 | 104.17.97.228 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 764 | 104.18.172.20 | 104.18.172.20 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 775 | 172.67.67.0 | 172.67.67.0 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 792 | 104.17.56.208 | 104.17.56.208 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 799 | 104.25.249.225 | 104.25.249.225 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 829 | 2a06:98c1:3104:da84:1c63:f149:4d21:b339 | 2a06:98c1:3104:da84:1c63:f149:4d21:b339 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 830 | 2803:f800:50:9516:e4a1:4ba9:1c5e:7533 | 2803:f800:50:9516:e4a1:4ba9:1c5e:7533 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 834 | 2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea | 2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 5 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 6 | comicabc.com | 104.21.64.10 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 10 | www.ipget.net | 104.21.15.212 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 41 | ct.877774.xyz | 172.64.229.173 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 43 | ipinfo.in | 104.21.21.129 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 52 | shopify.com | 23.227.38.33 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 57 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 67 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 73 | 103.160.204.59 | 103.160.204.59 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 79 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 81 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 100 | www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 107 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 108 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 109 | www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 120 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 124 | cfip.xxxxxxxx.tk | 104.18.228.35 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 125 | cfip.xxxxxxxx.tk | 104.21.91.19 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 129 | cfip.xxxxxxxx.tk | 104.17.127.110 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 146 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 157 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 162 | toy-people.com | 104.26.3.36 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 174 | zread.ai | 172.67.202.78 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 178 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 209 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 216 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 218 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 220 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 230 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 236 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 246 | cf.zhetengsha.eu.org | 172.64.145.158 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 248 | cf.zhetengsha.eu.org | 2a06:98c1:3101::ac40:919e | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 250 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 253 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 257 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 280 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 295 | time.is | 104.26.13.54 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 303 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 342 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 350 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 352 | dnschecker.org | 104.26.6.89 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 369 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 384 | www.udemy.com | 104.16.142.237 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 385 | www.udemy.com | 104.16.143.237 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 407 | eur.877774.xyz | 104.21.26.150 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 410 | ifconfig.co | 172.67.168.106 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 425 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 473 | 104.18.81.19 | 104.18.81.19 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 475 | 104.18.166.129 | 104.18.166.129 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 487 | 104.19.220.22 | 104.19.220.22 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 488 | 104.19.154.200 | 104.19.154.200 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 492 | 104.31.16.158 | 104.31.16.158 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 496 | 104.17.142.212 | 104.17.142.212 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 502 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 503 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 525 | 104.20.24.107 | 104.20.24.107 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 534 | 104.17.16.248 | 104.17.16.248 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 563 | 104.20.21.161 | 104.20.21.161 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 564 | 104.20.20.42 | 104.20.20.42 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 603 | 104.17.154.254 | 104.17.154.254 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 646 | 104.18.42.61 | 104.18.42.61 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 648 | 104.26.2.2 | 104.26.2.2 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 651 | 172.67.73.120 | 172.67.73.120 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 655 | 104.20.25.82 | 104.20.25.82 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 665 | 104.17.25.241 | 104.17.25.241 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 695 | 104.18.41.101 | 104.18.41.101 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 706 | 104.26.4.4 | 104.26.4.4 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 708 | 104.17.153.151 | 104.17.153.151 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 752 | 2a06:98c1:3104::f3:8fed:cac0 | 2a06:98c1:3104::f3:8fed:cac0 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 785 | 172.67.72.212 | 172.67.72.212 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 790 | 104.18.148.235 | 104.18.148.235 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 793 | 104.25.247.78 | 104.25.247.78 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 797 | 104.25.241.85 | 104.25.241.85 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 798 | 104.25.240.123 | 104.25.240.123 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 801 | 104.25.250.205 | 104.25.250.205 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 803 | 2a06:98c1:310d:85:ac4c:8137:506:5188 | 2a06:98c1:310d:85:ac4c:8137:506:5188 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 811 | 172.64.144.132 | 172.64.144.132 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 2 | steamdb.info | 172.66.175.250 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 9 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 37 | ct.877774.xyz | 172.64.229.217 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 54 | 104.18.37.13 | 104.18.37.13 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 71 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 74 | 104.18.254.88 | 104.18.254.88 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 82 | icook.hk | 172.67.161.104 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 84 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 85 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 99 | www.visa.cn | 162.159.152.2 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 101 | www.okcupid.com | 104.16.223.254 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 117 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 122 | cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 127 | cfip.xxxxxxxx.tk | 190.93.247.169 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 132 | 172.67.243.218 | 172.67.243.218 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 147 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 151 | na.877774.xyz | 104.18.38.235 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 156 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 160 | freeyx.cloudflare88.eu.org | 2606:4700:3010:0:51:d1e4:65f0:5ed1 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 163 | toy-people.com | 104.26.2.36 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 167 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 175 | zread.ai | 104.21.76.240 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 185 | asia.877774.xyz | 104.16.211.153 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 193 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 211 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 221 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 231 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 232 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 238 | 172.64.151.55 | 172.64.151.55 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 243 | bestcf.030101.xyz | 2606:4700:0:d9:2acf:b5e0:5a46:4358 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 261 | cf.090227.xyz | 2606:4700:440a::ac40:98f1 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 285 | www.glassdoor.com | 104.17.64.70 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 286 | 104.16.61.163 | 104.16.61.163 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 301 | palera.in | 104.21.58.72 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 304 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 325 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 330 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 331 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 339 | www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 340 | www.visa.com.hk | 104.18.21.69 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 345 | 162.159.133.85 | 162.159.133.85 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 363 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 376 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 381 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 396 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 404 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 406 | 104.19.223.58 | 104.19.223.58 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 430 | 172.64.82.114 | 172.64.82.114 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 433 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 462 | 173.245.49.194 | 173.245.49.194 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 469 | 162.159.128.253 | 162.159.128.253 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 476 | 198.41.208.224 | 198.41.208.224 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 485 | 104.18.151.172 | 104.18.151.172 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 486 | 104.17.139.37 | 104.17.139.37 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 489 | 104.18.255.167 | 104.18.255.167 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 495 | 104.16.105.166 | 104.16.105.166 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 506 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 519 | 172.67.68.211 | 172.67.68.211 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 554 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 569 | 104.17.60.233 | 104.17.60.233 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 585 | 104.18.39.228 | 104.18.39.228 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 586 | 104.18.44.148 | 104.18.44.148 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 588 | 162.159.62.6 | 162.159.62.6 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 601 | 104.16.147.114 | 104.16.147.114 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 606 | 104.17.21.106 | 104.17.21.106 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 620 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 621 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 622 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 660 | 104.17.115.224 | 104.17.115.224 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 662 | 104.17.209.79 | 104.17.209.79 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 677 | 162.159.61.106 | 162.159.61.106 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 681 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 685 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 711 | 104.17.110.226 | 104.17.110.226 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 725 | 104.18.42.106 | 104.18.42.106 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 728 | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 733 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 765 | 104.17.111.150 | 104.17.111.150 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 776 | 104.26.11.33 | 104.26.11.33 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 791 | 104.25.245.215 | 104.25.245.215 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 800 | 104.25.254.14 | 104.25.254.14 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 832 | 2a06:98c1:3107:54:2c60:eafc:f14d:ca4b | 2a06:98c1:3107:54:2c60:eafc:f14d:ca4b | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 30 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 44 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 50 | 104.18.39.196 | 104.18.39.196 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 64 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 83 | icook.hk | 104.21.90.210 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 103 | www.okcupid.com | 104.18.160.63 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 112 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 118 | cfip.xxxxxxxx.tk | 104.25.105.1 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 130 | cfip.xxxxxxxx.tk | 104.20.255.53 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 210 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 227 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 251 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 252 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 274 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 281 | www.ipchicken.com | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 282 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 283 | www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 302 | palera.in | 172.67.157.122 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 333 | silkbook.com | 104.26.9.160 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 354 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 383 | 104.17.142.12 | 104.17.142.12 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 387 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 398 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 426 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 442 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 464 | 162.159.136.89 | 162.159.136.89 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 505 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 537 | 104.18.39.15 | 104.18.39.15 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 552 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 565 | 104.26.4.190 | 104.26.4.190 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 576 | 104.17.100.254 | 104.17.100.254 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 599 | 104.17.50.237 | 104.17.50.237 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 607 | 104.16.144.235 | 104.16.144.235 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 625 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 652 | 104.26.3.120 | 104.26.3.120 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 657 | 104.26.10.239 | 104.26.10.239 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 661 | 104.19.34.231 | 104.19.34.231 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 664 | 104.19.35.242 | 104.19.35.242 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 678 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 684 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 687 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 715 | 104.16.251.254 | 104.16.251.254 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 720 | 104.18.32.174 | 104.18.32.174 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 732 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 755 | 104.26.0.210 | 104.26.0.210 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 758 | 104.26.13.110 | 104.26.13.110 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 783 | 104.20.22.91 | 104.20.22.91 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 784 | 104.26.4.135 | 104.26.4.135 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 787 | 104.26.5.194 | 104.26.5.194 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 794 | 104.25.244.36 | 104.25.244.36 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 802 | 2a06:98c1:3108:0:edda:98f0:da65:4271 | 2a06:98c1:3108:0:edda:98f0:da65:4271 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 815 | 2a06:98c1:51::ee:b8fb:877f | 2a06:98c1:51::ee:b8fb:877f | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 35 | ct.877774.xyz | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 49 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 51 | 104.16.45.84 | 104.16.45.84 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 68 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 77 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 78 | iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 158 | freeyx.cloudflare88.eu.org | 141.101.120.232 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 159 | freeyx.cloudflare88.eu.org | 141.101.121.66 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 177 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 197 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 198 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 212 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 214 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 224 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 228 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 240 | saas.sin.fan | 162.159.36.20 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 266 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 292 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 294 | time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 315 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 319 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 344 | 104.18.14.76 | 104.18.14.76 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 371 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 389 | 172.67.79.211 | 172.67.79.211 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 390 | 198.62.62.4 | 198.62.62.4 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 421 | 172.67.181.209 | 172.67.181.209 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 427 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 436 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 460 | 104.26.8.192 | 104.26.8.192 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 467 | 104.26.5.134 | 104.26.5.134 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 472 | 162.159.140.116 | 162.159.140.116 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 484 | 104.17.162.3 | 104.17.162.3 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 501 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 507 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 522 | 172.67.64.214 | 172.67.64.214 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 555 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 571 | 104.19.44.238 | 104.19.44.238 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 578 | 104.16.157.50 | 104.16.157.50 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 594 | 104.20.17.233 | 104.20.17.233 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 602 | 104.17.30.164 | 104.17.30.164 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 626 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 649 | 104.20.21.147 | 104.20.21.147 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 704 | 172.67.67.152 | 172.67.67.152 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 709 | 104.19.144.159 | 104.19.144.159 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 734 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 746 | 104.18.42.16 | 104.18.42.16 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 773 | 172.67.70.56 | 172.67.70.56 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 804 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 831 | 2a06:98c1:3100:f702:ebbf:618b:76c:9ba7 | 2a06:98c1:3100:f702:ebbf:618b:76c:9ba7 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 835 | 2a06:98c1:3106:6a:7ba4:346b:e06c:71c7 | 2a06:98c1:3106:6a:7ba4:346b:e06c:71c7 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 7 | comicabc.com | 172.67.174.21 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 8 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 34 | ct.877774.xyz | 172.64.229.174 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 38 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 39 | ct.877774.xyz | 172.64.229.44 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 45 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 48 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 55 | www.gov.ua | 104.21.23.72 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 56 | www.gov.ua | 172.67.209.127 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 69 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 86 | cu.877774.xyz | 104.18.42.54 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 88 | www.4chan.org | 104.16.229.229 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 95 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 97 | 172.67.120.0 | 172.67.120.0 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 136 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 213 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 237 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 245 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 254 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 256 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 314 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 337 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 338 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 347 | cf.877774.xyz | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 378 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 440 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 499 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 531 | 172.64.229.191 | 172.64.229.191 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 550 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 572 | 104.18.40.39 | 104.18.40.39 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 575 | 104.16.155.76 | 104.16.155.76 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 583 | 104.18.37.110 | 104.18.37.110 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 593 | 104.26.6.159 | 104.26.6.159 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 605 | 104.16.251.143 | 104.16.251.143 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 624 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 644 | 172.64.153.141 | 172.64.153.141 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 658 | 104.17.215.66 | 104.17.215.66 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 729 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 730 | 2a06:98c1:3105:0:2359:4222:d558:10fb | 2a06:98c1:3105:0:2359:4222:d558:10fb | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 731 | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 735 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 770 | 104.18.166.232 | 104.18.166.232 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 808 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 822 | 172.64.229.15 | 172.64.229.15 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 1 | 172.64.154.18 | 172.64.154.18 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 13 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 66 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 70 | 172.67.110.232 | 172.67.110.232 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 131 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 206 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 208 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 310 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 318 | ip.gs | 104.21.14.176 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 334 | silkbook.com | 172.67.75.208 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 382 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 453 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 454 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 483 | 104.19.148.121 | 104.19.148.121 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 573 | 104.17.119.199 | 104.17.119.199 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 591 | 104.20.16.244 | 104.20.16.244 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 596 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 597 | 172.67.76.195 | 172.67.76.195 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 636 | 172.64.151.235 | 172.64.151.235 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 640 | 172.64.145.119 | 172.64.145.119 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 653 | 104.26.1.181 | 104.26.1.181 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 714 | 104.18.44.25 | 104.18.44.25 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 768 | 172.64.153.140 | 172.64.153.140 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 769 | 104.25.245.233 | 104.25.245.233 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 818 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | IPv6 | h2 | ✅ 成功 | 42 | cloudflare |
| 98 | www.visa.cn | 162.159.153.2 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 296 | time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 308 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 455 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 500 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 524 | 104.20.22.185 | 104.20.22.185 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 562 | 172.67.67.5 | 172.67.67.5 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 577 | 104.17.101.208 | 104.17.101.208 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 584 | 162.159.36.26 | 162.159.36.26 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 647 | 104.18.40.216 | 104.18.40.216 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 712 | 104.16.155.230 | 104.16.155.230 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 762 | 104.25.244.239 | 104.25.244.239 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 819 | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7 | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 76 | iplocation.io | 104.26.10.222 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 126 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 187 | asia.877774.xyz | 104.17.142.146 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 215 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 327 | singapore.com | 104.26.12.140 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 405 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 438 | www.7749tv.com | 104.19.133.4 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 619 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | h2 | ✅ 成功 | 44 | cloudflare |
| 656 | 172.67.72.36 | 172.67.72.36 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 11 | www.ipget.net | 172.67.207.26 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 23 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 29 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 32 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 87 | cu.877774.xyz | 172.64.145.202 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 111 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 326 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 456 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 556 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 705 | 104.20.17.51 | 104.20.17.51 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 718 | 104.18.36.1 | 104.18.36.1 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 813 | 172.64.52.181 | 172.64.52.181 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 814 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 105 | cf.877771.xyz | 172.67.152.183 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 279 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 627 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 641 | 104.26.5.101 | 104.26.5.101 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 700 | 104.26.6.171 | 104.26.6.171 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 89 | www.4chan.org | 104.16.228.229 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 91 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 432 | 198.41.208.15 | 198.41.208.15 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 702 | 172.67.79.150 | 172.67.79.150 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 710 | 104.17.53.129 | 104.17.53.129 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 134 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 322 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 582 | 108.162.195.1 | 108.162.195.1 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 595 | 172.67.70.253 | 172.67.70.253 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 703 | 104.20.25.161 | 104.20.25.161 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 713 | 104.17.25.87 | 104.17.25.87 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 807 | 2803:f800:51:0:fc87:e2d6:88c3:378b | 2803:f800:51:0:fc87:e2d6:88c3:378b | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 538 | 104.17.105.198 | 104.17.105.198 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 690 | 172.64.41.216 | 172.64.41.216 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 423 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 738 | 108.162.198.85 | 108.162.198.85 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 812 | 162.159.45.150 | 162.159.45.150 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 579 | 162.159.34.55 | 162.159.34.55 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 744 | 162.159.39.146 | 162.159.39.146 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 188 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 536 | 104.17.170.110 | 104.17.170.110 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 723 | 162.159.1.145 | 162.159.1.145 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 27 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 275 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 739 | 162.159.38.226 | 162.159.38.226 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 740 | 162.159.45.65 | 162.159.45.65 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 742 | 162.159.44.101 | 162.159.44.101 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 767 | 162.159.20.46 | 162.159.20.46 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 143 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 3 | steamdb.info | 104.20.34.212 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 24 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 265 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 634 | 162.159.41.141 | 162.159.41.141 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 668 | 172.64.52.67 | 172.64.52.67 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 692 | 162.159.36.223 | 162.159.36.223 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 809 | 172.64.153.183 | 172.64.153.183 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 19 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 92 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 269 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 510 | 162.159.21.116 | 162.159.21.116 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 631 | 172.64.52.90 | 172.64.52.90 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 144 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 276 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 444 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 615 | 162.159.44.128 | 162.159.44.128 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 781 | 162.159.39.180 | 162.159.39.180 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 431 | 198.41.194.162 | 198.41.194.162 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 443 | 172.64.52.127 | 172.64.52.127 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 514 | 162.159.46.238 | 162.159.46.238 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 544 | 108.162.194.125 | 108.162.194.125 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 643 | 162.159.22.29 | 162.159.22.29 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 701 | 172.67.78.67 | 172.67.78.67 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 821 | 162.159.38.45 | 162.159.38.45 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 189 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 372 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 446 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 458 | 172.64.48.226 | 172.64.48.226 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 461 | 172.64.50.51 | 172.64.50.51 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 471 | 104.26.8.117 | 104.26.8.117 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 628 | 162.159.38.171 | 162.159.38.171 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 639 | 162.159.3.89 | 162.159.3.89 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 826 | 162.159.45.67 | 162.159.45.67 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 168 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 277 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 367 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 545 | 162.159.0.115 | 162.159.0.115 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 796 | 108.162.198.70 | 108.162.198.70 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 823 | 162.159.38.83 | 162.159.38.83 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 824 | 162.159.39.20 | 162.159.39.20 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 309 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 616 | 162.159.38.35 | 162.159.38.35 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 719 | 172.64.49.146 | 172.64.49.146 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 774 | 162.159.11.128 | 162.159.11.128 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 806 | 162.159.39.74 | 162.159.39.74 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 366 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 424 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 465 | 162.159.58.65 | 162.159.58.65 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 724 | 162.159.36.52 | 162.159.36.52 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 825 | 172.64.52.224 | 172.64.52.224 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 827 | 108.162.198.198 | 108.162.198.198 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 459 | 162.159.24.131 | 162.159.24.131 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 509 | 162.159.17.243 | 162.159.17.243 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 511 | 162.159.6.115 | 162.159.6.115 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 540 | 108.162.198.48 | 108.162.198.48 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 633 | 172.64.34.153 | 172.64.34.153 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 697 | 162.159.16.136 | 162.159.16.136 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 609 | 108.162.198.69 | 108.162.198.69 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 688 | 162.159.6.44 | 162.159.6.44 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 699 | 104.26.1.194 | 104.26.1.194 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 766 | 162.159.0.41 | 162.159.0.41 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 413 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 543 | 172.64.53.0 | 172.64.53.0 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 805 | 162.159.33.191 | 162.159.33.191 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 422 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 642 | 162.159.1.111 | 162.159.1.111 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 672 | 162.159.44.133 | 162.159.44.133 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 414 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 638 | 162.159.18.240 | 162.159.18.240 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 669 | 162.159.42.146 | 162.159.42.146 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 271 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 273 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 291 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 478 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 171 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 203 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 358 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 448 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 173 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 179 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 417 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 480 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 632 | 172.64.53.181 | 172.64.53.181 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 680 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 682 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 96 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 361 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 618 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 155 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 183 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 192 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 268 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 362 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 370 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 418 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 481 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 551 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 553 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 557 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 137 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 148 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 172 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 272 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 278 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 293 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 360 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 416 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | h2 | ✅ 成功 | 79 | cloudflare |
| 666 | 162.159.21.222 | 162.159.21.222 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 152 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 184 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 267 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 375 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 482 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 828 | 2a06:98c1:51:a594:2926:2b16:6e8d:843e | 2a06:98c1:51:a594:2926:2b16:6e8d:843e | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 833 | 2a06:98c1:51:aa:3e22:dd48:6279:eeb9 | 2a06:98c1:51:aa:3e22:dd48:6279:eeb9 | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 191 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 205 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 447 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 737 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 94 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 138 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 204 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | h2 | ✅ 成功 | 84 | cloudflare |
| 581 | 162.159.42.140 | 162.159.42.140 | IPv4 | h2 | ✅ 成功 | 84 | cloudflare |
| 629 | 162.159.39.177 | 162.159.39.177 | IPv4 | h2 | ✅ 成功 | 84 | cloudflare |
| 21 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | h2 | ✅ 成功 | 86 | cloudflare |
| 290 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 169 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 365 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 153 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 18 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 587 | 162.159.19.37 | 162.159.19.37 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 359 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 26 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h2 | ✅ 成功 | 94 | cloudflare |
| 200 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 539 | 162.159.44.176 | 162.159.44.176 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 741 | 172.64.53.165 | 172.64.53.165 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 747 | 162.159.36.205 | 162.159.36.205 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 617 | 172.64.53.144 | 172.64.53.144 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 670 | 172.64.53.40 | 172.64.53.40 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 743 | 172.64.52.189 | 172.64.52.189 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 93 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 135 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 154 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 674 | 108.162.198.148 | 108.162.198.148 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 779 | 162.159.12.120 | 162.159.12.120 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 25 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h2 | ✅ 成功 | 98 | cloudflare |
| 264 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h2 | ✅ 成功 | 98 | cloudflare |
| 305 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h2 | ✅ 成功 | 98 | cloudflare |
| 512 | 108.162.192.66 | 108.162.192.66 | IPv4 | h2 | ✅ 成功 | 98 | cloudflare |
| 789 | 198.41.222.191 | 198.41.222.191 | IPv4 | h2 | ✅ 成功 | 98 | cloudflare |
| 270 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 541 | 162.159.38.192 | 162.159.38.192 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 429 | 108.162.198.54 | 108.162.198.54 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 445 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 513 | 162.159.13.51 | 162.159.13.51 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 547 | 172.64.52.110 | 172.64.52.110 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 612 | 172.64.52.15 | 172.64.52.15 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 613 | 162.159.39.99 | 162.159.39.99 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 696 | 172.64.32.77 | 172.64.32.77 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 788 | 172.64.53.202 | 172.64.53.202 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 449 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | h2 | ✅ 成功 | 101 | cloudflare |
| 479 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h2 | ✅ 成功 | 101 | cloudflare |
| 611 | 162.159.19.219 | 162.159.19.219 | IPv4 | h2 | ✅ 成功 | 101 | cloudflare |
| 676 | 162.159.45.165 | 162.159.45.165 | IPv4 | h2 | ✅ 成功 | 101 | cloudflare |
| 777 | 162.159.3.128 | 162.159.3.128 | IPv4 | h2 | ✅ 成功 | 101 | cloudflare |
| 778 | 162.159.38.134 | 162.159.38.134 | IPv4 | h2 | ✅ 成功 | 101 | cloudflare |
| 780 | 162.159.58.17 | 162.159.58.17 | IPv4 | h2 | ✅ 成功 | 101 | cloudflare |
| 820 | 172.64.53.41 | 172.64.53.41 | IPv4 | h2 | ✅ 成功 | 101 | cloudflare |
| 180 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 288 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 673 | 162.159.38.67 | 162.159.38.67 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 263 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h2 | ✅ 成功 | 103 | cloudflare |
| 373 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h2 | ✅ 成功 | 103 | cloudflare |
| 392 | 172.64.35.24 | 172.64.35.24 | IPv4 | h2 | ✅ 成功 | 103 | cloudflare |
| 546 | 162.159.45.93 | 162.159.45.93 | IPv4 | h2 | ✅ 成功 | 103 | cloudflare |
| 751 | 162.159.44.202 | 162.159.44.202 | IPv4 | h2 | ✅ 成功 | 103 | cloudflare |
| 190 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h2 | ✅ 成功 | 104 | cloudflare |
| 614 | 162.159.7.12 | 162.159.7.12 | IPv4 | h2 | ✅ 成功 | 104 | cloudflare |
| 721 | 172.64.40.196 | 172.64.40.196 | IPv4 | h2 | ✅ 成功 | 104 | cloudflare |
| 133 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h2 | ✅ 成功 | 105 | cloudflare |
| 145 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h2 | ✅ 成功 | 105 | cloudflare |
| 306 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h2 | ✅ 成功 | 105 | cloudflare |
| 810 | 2a06:98c1:51:6e:e874:db4f:a1d5:2163 | 2a06:98c1:51:6e:e874:db4f:a1d5:2163 | IPv6 | h2 | ✅ 成功 | 105 | cloudflare |
| 289 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h2 | ✅ 成功 | 106 | cloudflare |
| 671 | 162.159.39.136 | 162.159.39.136 | IPv4 | h2 | ✅ 成功 | 106 | cloudflare |
| 357 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 698 | 104.26.11.160 | 104.26.11.160 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 817 | 162.159.43.50 | 162.159.43.50 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 548 | 162.159.39.62 | 162.159.39.62 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 477 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 610 | 162.159.45.176 | 162.159.45.176 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 20 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 630 | 162.159.45.145 | 162.159.45.145 | IPv4 | h2 | ✅ 成功 | 118 | cloudflare |
| 795 | 162.159.1.39 | 162.159.1.39 | IPv4 | h2 | ✅ 成功 | 119 | cloudflare |
| 415 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h2 | ✅ 成功 | 120 | cloudflare |
| 635 | 108.162.198.170 | 108.162.198.170 | IPv4 | h2 | ✅ 成功 | 125 | cloudflare |
| 181 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h2 | ✅ 成功 | 132 | cloudflare |
| 170 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h2 | ✅ 成功 | 134 | cloudflare |
| 307 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h2 | ✅ 成功 | 134 | cloudflare |
| 287 | 162.159.36.104 | 162.159.36.104 | IPv4 | h2 | ✅ 成功 | 136 | cloudflare |
| 202 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h2 | ✅ 成功 | 143 | cloudflare |
| 516 | 173.245.58.237 | 173.245.58.237 | IPv4 | h2 | ✅ 成功 | 159 | cloudflare |
| 561 | 104.26.8.148 | 104.26.8.148 | IPv4 | h2 | ✅ 成功 | 170 | cloudflare |
| 374 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h2 | ✅ 成功 | 197 | cloudflare |
| 816 | 172.64.151.253 | 172.64.151.253 | IPv4 | h2 | ✅ 成功 | 312 | cloudflare |
| 258 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 361 | cloudflare |
| 72 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 531 | cloudflare |
| 33 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 863 | cloudflare |
| 560 | 172.67.77.196 | 172.67.77.196 | IPv4 | h2 | ✅ 成功 | 921 | cloudflare |
| 559 | 104.20.26.58 | 104.20.26.58 | IPv4 | h2 | ✅ 成功 | 1138 | cloudflare |
| 558 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | h2 | ✅ 成功 | 1409 | cloudflare |
| 412 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 1480 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 606 条记录
- **快 (50-100ms)**: 167 条记录
- **正常 (100-200ms)**: 52 条记录
- **慢 (200-500ms)**: 2 条记录
- **很慢 (>500ms)**: 6 条记录


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
