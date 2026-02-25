# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/2/25 03:50:29
- **数据来源**: connectivity_results-20260225-035028.json
- **总测试数**: 792
- **失败测试数**: 2
- **成功测试数**: 790
- **失败率**: 0.25%
- **平均延迟**: 48.93ms
- **最小延迟**: 24ms
- **最大延迟**: 1954ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/2/25 03:50:29
- **IP地址**: 2a09:bac5:6215:28::4:376
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
| 80 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |
| 131 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 2 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 172.64（1 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 2 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: 所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好



---

## 🚀 延迟最低的 790 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 162 | 162.159.140.85 | 162.159.140.85 | IPv4 | h2 | ✅ 成功 | 24 | cloudflare |
| 151 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 29 | cloudflare |
| 155 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 29 | cloudflare |
| 157 | japan.com | 172.67.70.92 | IPv4 | h2 | ✅ 成功 | 29 | cloudflare |
| 208 | 104.17.142.212 | 104.17.142.212 | IPv4 | h2 | ✅ 成功 | 29 | cloudflare |
| 242 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h2 | ✅ 成功 | 29 | cloudflare |
| 384 | 172.64.146.67 | 172.64.146.67 | IPv4 | h2 | ✅ 成功 | 29 | cloudflare |
| 396 | 104.26.10.239 | 104.26.10.239 | IPv4 | h2 | ✅ 成功 | 29 | cloudflare |
| 763 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 29 | cloudflare |
| 48 | www.visa.com.hk | 104.18.21.69 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 89 | icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 130 | cfip.xxxxxxxx.tk | 198.41.214.141 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 165 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2 | ✅ 成功 | 30 | cloudflare |
| 230 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 257 | 104.26.15.85 | 104.26.15.85 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 326 | 104.26.12.113 | 104.26.12.113 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 331 | 104.20.17.233 | 104.20.17.233 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 334 | 104.26.4.44 | 104.26.4.44 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 391 | 104.26.1.181 | 104.26.1.181 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 395 | 172.67.72.36 | 172.67.72.36 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 460 | 104.26.11.160 | 104.26.11.160 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 470 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 471 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | h2 | ✅ 成功 | 30 | cloudflare |
| 473 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 30 | cloudflare |
| 475 | 104.26.7.7 | 104.26.7.7 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 540 | 104.17.111.150 | 104.17.111.150 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 572 | 103.160.204.59 | 103.160.204.59 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 587 | na.877774.xyz | 104.18.38.235 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 612 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 630 | cu.877774.xyz | 104.26.4.114 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 637 | freeyx.cloudflare88.eu.org | 141.101.120.156 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 640 | freeyx.cloudflare88.eu.org | 2606:4700:3009::83da:e4f:b8a5 | IPv6 | h2 | ✅ 成功 | 30 | cloudflare |
| 666 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 30 | cloudflare |
| 737 | time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 775 | singapore.com | 104.26.13.140 | IPv4 | h2 | ✅ 成功 | 30 | cloudflare |
| 790 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 30 | cloudflare |
| 5 | comicabc.com | 104.21.64.10 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 36 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 52 | cf.877774.xyz | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 54 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 55 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 77 | tasteatlas.com | 104.17.37.105 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 82 | www.udemy.com | 104.16.143.237 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 90 | icook.tw | 104.20.28.74 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 96 | 104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 99 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 100 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 113 | ifconfig.co | 172.67.168.106 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 114 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 122 | cfip.xxxxxxxx.tk | 190.93.246.67 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 129 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 135 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 154 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 166 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 173 | 104.26.4.90 | 104.26.4.90 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 181 | 162.159.128.253 | 162.159.128.253 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 182 | 104.26.3.162 | 104.26.3.162 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 203 | 104.17.69.244 | 104.17.69.244 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 207 | 104.16.105.166 | 104.16.105.166 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 216 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 219 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 224 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 226 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 228 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 229 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 234 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 236 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 240 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 241 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 256 | 172.67.68.211 | 172.67.68.211 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 258 | 172.67.74.57 | 172.67.74.57 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 274 | 104.17.170.110 | 104.17.170.110 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 291 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 302 | 104.26.4.190 | 104.26.4.190 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 303 | 104.20.18.47 | 104.20.18.47 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 323 | 104.18.44.148 | 104.18.44.148 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 325 | 162.159.62.6 | 162.159.62.6 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 328 | 172.67.68.252 | 172.67.68.252 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 329 | 104.20.16.244 | 104.20.16.244 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 330 | 104.26.6.159 | 104.26.6.159 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 336 | 104.17.24.232 | 104.17.24.232 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 346 | 104.16.144.235 | 104.16.144.235 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 379 | 172.64.145.119 | 172.64.145.119 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 389 | 104.26.5.101 | 104.26.5.101 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 435 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 437 | steamdb.info | 104.20.34.212 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 438 | steamdb.info | 172.66.175.250 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 440 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 449 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 461 | 104.26.1.194 | 104.26.1.194 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 464 | 104.20.25.161 | 104.20.25.161 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 467 | 104.20.17.51 | 104.20.17.51 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 479 | 104.17.110.226 | 104.17.110.226 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 480 | 104.16.155.230 | 104.16.155.230 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 483 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 505 | 104.26.0.210 | 104.26.0.210 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 532 | 104.26.8.171 | 104.26.8.171 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 533 | 104.20.20.192 | 104.20.20.192 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 539 | 104.18.172.20 | 104.18.172.20 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 562 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 565 | 172.67.110.232 | 172.67.110.232 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 568 | icook.hk | 172.67.161.104 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 569 | icook.hk | 104.21.90.210 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 574 | zread.ai | 172.67.202.78 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 576 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 583 | www.visa.cn | 162.159.152.2 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 584 | www.visa.cn | 162.159.153.2 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 589 | www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 594 | 172.67.243.218 | 172.67.243.218 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 609 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 610 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 613 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 614 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 624 | cu.877774.xyz | 104.26.4.117 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 628 | cu.877774.xyz | 104.26.4.112 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 638 | freeyx.cloudflare88.eu.org | 2606:4700:3009:aa59:4b67:100d:4fdb:d8a1 | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 641 | asia.877774.xyz | 104.17.142.146 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 643 | asia.877774.xyz | 104.17.139.62 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 645 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 649 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 661 | bestcf.030101.xyz | 2606:4700:0:d9:2acf:b5e0:5a46:4358 | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 663 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 665 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 667 | toy-people.com | 104.26.3.36 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 671 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 684 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 692 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 722 | www.glassdoor.com | 104.16.25.46 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 728 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 749 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 750 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 751 | palera.in | 172.67.157.122 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 764 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 777 | singapore.com | 104.26.12.140 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 780 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 31 | cloudflare |
| 787 | www.whatismyip.com | 104.26.12.23 | IPv4 | h2 | ✅ 成功 | 31 | cloudflare |
| 6 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 8 | www.ipget.net | 172.67.207.26 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 24 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 33 | dnschecker.org | 104.26.6.89 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 35 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 37 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 40 | 172.67.106.26 | 172.67.106.26 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 51 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 72 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 76 | tasteatlas.com | 104.17.36.105 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 98 | www.digitalocean.com | 104.19.173.68 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 104 | eur.877774.xyz | 104.21.47.209 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 105 | eur.877774.xyz | 104.21.26.150 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 126 | cfip.xxxxxxxx.tk | 104.21.91.19 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 128 | cfip.xxxxxxxx.tk | 104.25.105.1 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 132 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 140 | www.7749tv.com | 104.19.255.188 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 144 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 153 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 156 | japan.com | 104.26.5.60 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 160 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 163 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 180 | 104.17.30.164 | 104.17.30.164 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 186 | 104.18.89.52 | 104.18.89.52 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 195 | 104.19.148.121 | 104.19.148.121 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 197 | 104.18.151.172 | 104.18.151.172 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 204 | 104.31.16.158 | 104.31.16.158 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 212 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 214 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 217 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 218 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 223 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 232 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 235 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 239 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 243 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 252 | 104.18.47.253 | 104.18.47.253 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 255 | 172.64.150.30 | 172.64.150.30 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 260 | 172.67.65.159 | 172.67.65.159 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 262 | 104.20.24.107 | 104.20.24.107 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 265 | 104.26.3.176 | 104.26.3.176 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 269 | 172.64.229.191 | 172.64.229.191 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 272 | 104.17.16.248 | 104.17.16.248 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 293 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 299 | 172.67.67.5 | 172.67.67.5 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 301 | 104.20.20.42 | 104.20.20.42 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 306 | 104.17.60.233 | 104.17.60.233 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 310 | 104.17.119.199 | 104.17.119.199 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 312 | 104.16.155.76 | 104.16.155.76 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 322 | 104.18.39.228 | 104.18.39.228 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 345 | 104.17.21.106 | 104.17.21.106 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 360 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 364 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 390 | 172.67.73.120 | 172.67.73.120 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 392 | 104.26.3.120 | 104.26.3.120 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 393 | 172.67.75.11 | 172.67.75.11 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 397 | 104.17.211.218 | 104.17.211.218 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 398 | 104.17.170.137 | 104.17.170.137 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 401 | 104.17.215.66 | 104.17.215.66 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 403 | 104.17.209.79 | 104.17.209.79 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 404 | 104.19.35.242 | 104.19.35.242 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 423 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 429 | www.pcmag.com | 104.16.20.118 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 430 | www.pcmag.com | 104.16.21.118 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 434 | ipinfo.in | 172.67.198.203 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 436 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 442 | www.gov.ua | 104.21.23.72 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 453 | 172.64.152.215 | 172.64.152.215 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 455 | 172.64.145.108 | 172.64.145.108 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 468 | iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 472 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 474 | 104.26.4.4 | 104.26.4.4 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 476 | 104.17.153.151 | 104.17.153.151 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 482 | ct.877774.xyz | 172.64.229.217 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 484 | ct.877774.xyz | 172.64.229.44 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 499 | 104.18.47.46 | 104.18.47.46 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 501 | 104.18.42.106 | 104.18.42.106 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 503 | 104.16.45.84 | 104.16.45.84 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 504 | 104.18.39.196 | 104.18.39.196 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 507 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 508 | 2a06:98c1:3105:0:2359:4222:d558:10fb | 2a06:98c1:3105:0:2359:4222:d558:10fb | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 557 | 2a06:98c1:310d:85:ac4c:8137:506:5188 | 2a06:98c1:310d:85:ac4c:8137:506:5188 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 566 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 571 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 573 | zread.ai | 104.21.76.240 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 575 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 578 | 172.67.49.134 | 172.67.49.134 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 580 | www.4chan.org | 104.16.228.229 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 592 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 593 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 600 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 606 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 607 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 615 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 623 | cu.877774.xyz | 104.26.4.116 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 635 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 639 | freeyx.cloudflare88.eu.org | 2606:4700:3009:7dab:39d5:a339:79b6:3c1c | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 644 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 659 | bestcf.030101.xyz | 104.19.156.188 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 664 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 688 | cf.zhetengsha.eu.org | 104.18.42.98 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 693 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 697 | cf.090227.xyz | 2a06:98c1:3101::ac40:919e | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 712 | www.ipchicken.com | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 726 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 735 | time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 736 | time.is | 104.26.13.54 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 752 | palera.in | 104.21.58.72 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 753 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 770 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 774 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 32 | cloudflare |
| 782 | silkbook.com | 104.26.9.160 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 789 | www.whatismyip.com | 172.67.69.129 | IPv4 | h2 | ✅ 成功 | 32 | cloudflare |
| 3 | shopify.com | 23.227.38.33 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 4 | comicabc.com | 172.67.174.21 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 38 | 104.18.42.26 | 104.18.42.26 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 50 | 162.159.133.85 | 162.159.133.85 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 64 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 65 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 66 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 71 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 74 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 85 | 104.17.142.12 | 104.17.142.12 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 102 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 103 | eur.877774.xyz | 104.21.29.164 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 123 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 125 | cfip.xxxxxxxx.tk | 104.18.228.35 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 134 | cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 147 | 172.64.82.114 | 172.64.82.114 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 159 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 161 | 198.41.208.15 | 198.41.208.15 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 175 | 162.159.136.89 | 162.159.136.89 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 187 | 104.18.166.129 | 104.18.166.129 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 198 | 104.17.139.37 | 104.17.139.37 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 200 | 104.19.154.200 | 104.19.154.200 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 202 | 104.19.212.207 | 104.19.212.207 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 206 | 104.18.223.253 | 104.18.223.253 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 215 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 225 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 227 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 237 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 244 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 259 | 172.67.64.214 | 172.67.64.214 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 261 | 104.20.22.185 | 104.20.22.185 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 263 | 104.20.30.198 | 104.20.30.198 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 264 | 172.67.72.254 | 172.67.72.254 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 271 | 104.16.255.1 | 104.16.255.1 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 275 | 104.18.39.15 | 104.18.39.15 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 287 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 288 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 292 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 295 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 297 | 172.67.77.196 | 172.67.77.196 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 298 | 104.26.8.148 | 104.26.8.148 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 300 | 104.20.21.161 | 104.20.21.161 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 305 | 104.20.19.201 | 104.20.19.201 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 313 | 104.17.100.254 | 104.17.100.254 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 314 | 104.17.101.208 | 104.17.101.208 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 343 | 104.17.53.25 | 104.17.53.25 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 344 | 104.16.251.143 | 104.16.251.143 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 361 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 365 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 376 | 172.64.145.242 | 172.64.145.242 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 424 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 425 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 441 | www.gov.ua | 172.67.209.127 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 447 | cf.0sm.com | 172.67.187.145 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 456 | 104.18.37.177 | 104.18.37.177 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 478 | 104.17.53.129 | 104.17.53.129 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 502 | 104.18.40.202 | 104.18.40.202 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 513 | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 525 | 104.26.1.88 | 104.26.1.88 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 527 | 104.26.6.247 | 104.26.6.247 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 531 | 104.25.250.174 | 104.25.250.174 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 534 | 104.26.13.110 | 104.26.13.110 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 535 | 104.17.97.228 | 104.17.97.228 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 536 | 104.25.241.198 | 104.25.241.198 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 538 | 104.17.193.113 | 104.17.193.113 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 543 | 104.18.166.232 | 104.18.166.232 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 585 | na.877774.xyz | 104.19.74.233 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 598 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 603 | www.okcupid.com | 104.18.160.63 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 604 | www.okcupid.com | 104.16.239.254 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 605 | www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 620 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 625 | cu.877774.xyz | 104.26.4.118 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 627 | cu.877774.xyz | 104.26.4.111 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 679 | 172.64.151.55 | 172.64.151.55 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 680 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 682 | saas.sin.fan | 162.159.36.20 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 683 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 686 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 690 | cf.zhetengsha.eu.org | 2606:4700:440a::ac40:98f1 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 691 | fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 698 | cf.090227.xyz | 2a06:98c1:3108::6812:2a62 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 733 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 741 | 104.16.61.163 | 104.16.61.163 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 761 | ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 762 | ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 768 | ip.gs | 104.21.14.176 | IPv4 | h2 | ✅ 成功 | 33 | cloudflare |
| 778 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 785 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 786 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅ 成功 | 33 | cloudflare |
| 7 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 9 | www.ipget.net | 104.21.15.212 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 23 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 25 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 30 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 31 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 57 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 69 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 84 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 87 | 198.62.62.4 | 198.62.62.4 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 92 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 93 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 97 | www.digitalocean.com | 104.19.174.68 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 101 | 104.18.78.214 | 104.18.78.214 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 133 | cfip.xxxxxxxx.tk | 104.20.255.53 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 136 | 172.67.181.209 | 172.67.181.209 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 148 | www.csgo.com | 195.85.59.161 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 149 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 168 | 162.159.61.183 | 162.159.61.183 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 172 | 173.245.49.194 | 173.245.49.194 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 220 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 222 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 266 | 104.17.169.180 | 104.17.169.180 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 267 | 104.17.101.37 | 104.17.101.37 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 286 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 296 | 104.20.26.58 | 104.20.26.58 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 304 | 104.26.12.227 | 104.26.12.227 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 308 | 104.19.44.238 | 104.19.44.238 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 332 | 172.67.70.253 | 172.67.70.253 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 335 | 172.67.76.195 | 172.67.76.195 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 337 | 104.17.50.237 | 104.17.50.237 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 341 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 375 | 172.64.151.235 | 172.64.151.235 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 383 | 172.64.153.141 | 172.64.153.141 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 387 | 104.26.2.2 | 104.26.2.2 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 399 | 104.17.115.224 | 104.17.115.224 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 402 | 104.19.153.47 | 104.19.153.47 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 406 | 104.17.189.30 | 104.17.189.30 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 416 | 162.159.61.106 | 162.159.61.106 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 427 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 428 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 431 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 439 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 444 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 445 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 448 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 452 | 172.64.41.216 | 172.64.41.216 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 477 | 104.19.144.159 | 104.19.144.159 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 485 | ct.877774.xyz | 172.64.229.161 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 487 | ct.877774.xyz | 172.64.229.174 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 488 | ct.877774.xyz | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 506 | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 510 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 511 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 512 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 523 | 104.18.42.16 | 104.18.42.16 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 528 | 104.26.14.117 | 104.26.14.117 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 558 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 577 | 104.18.254.88 | 104.18.254.88 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 588 | www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 602 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 611 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 621 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 626 | cu.877774.xyz | 104.26.4.119 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 658 | bestcf.030101.xyz | 104.17.206.188 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 662 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 668 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 669 | toy-people.com | 104.26.2.36 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 681 | saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 685 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 714 | www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 738 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 754 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 771 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 772 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 34 | cloudflare |
| 779 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 791 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 34 | cloudflare |
| 11 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 15 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 16 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 49 | www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 68 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 78 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 83 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 88 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 106 | 104.19.223.58 | 104.19.223.58 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 121 | cfip.xxxxxxxx.tk | 104.17.127.110 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 124 | cfip.xxxxxxxx.tk | 190.93.247.169 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 158 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 174 | 104.18.189.153 | 104.18.189.153 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 177 | 172.64.229.7 | 172.64.229.7 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 178 | 104.26.5.134 | 104.26.5.134 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 201 | 104.18.255.167 | 104.18.255.167 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 254 | 104.18.45.95 | 104.18.45.95 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 307 | 104.16.148.187 | 104.16.148.187 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 309 | 104.18.40.39 | 104.18.40.39 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 315 | 104.16.157.50 | 104.16.157.50 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 320 | 104.18.37.110 | 104.18.37.110 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 327 | 104.26.2.166 | 104.26.2.166 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 333 | 104.26.8.192 | 104.26.8.192 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 347 | 172.64.147.235 | 172.64.147.235 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 366 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 385 | 104.18.42.61 | 104.18.42.61 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 394 | 104.20.25.82 | 104.20.25.82 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 420 | 172.64.154.226 | 172.64.154.226 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 421 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 433 | ipinfo.in | 104.21.21.129 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 486 | ct.877774.xyz | 172.64.229.173 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 526 | 104.20.28.239 | 104.20.28.239 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 530 | 172.67.65.150 | 172.67.65.150 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 545 | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7 | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 570 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 582 | 172.67.120.0 | 172.67.120.0 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 591 | cf.877771.xyz | 104.21.80.180 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 601 | www.okcupid.com | 104.16.223.254 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 608 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 629 | cu.877774.xyz | 104.26.4.113 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 650 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 670 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 672 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 687 | cf.zhetengsha.eu.org | 172.64.145.158 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 694 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 709 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 713 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 721 | www.glassdoor.com | 104.17.64.70 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 746 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 760 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 765 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 767 | ip.gs | 172.67.160.28 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 781 | silkbook.com | 104.26.8.160 | IPv4 | h2 | ✅ 成功 | 35 | cloudflare |
| 784 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 35 | cloudflare |
| 81 | www.udemy.com | 104.16.142.237 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 86 | 172.67.79.211 | 172.67.79.211 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 127 | cfip.xxxxxxxx.tk | 188.114.97.144 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 138 | 104.19.175.123 | 104.19.175.123 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 196 | 104.17.162.3 | 104.17.162.3 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 270 | 104.17.156.81 | 104.17.156.81 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 273 | 104.17.168.159 | 104.17.168.159 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 342 | 104.17.154.254 | 104.17.154.254 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 362 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 405 | 104.17.25.241 | 104.17.25.241 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 417 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 457 | 104.18.41.101 | 104.18.41.101 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 462 | 104.26.6.171 | 104.26.6.171 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 465 | 172.67.79.150 | 172.67.79.150 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 537 | 104.25.244.239 | 104.25.244.239 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 544 | 104.18.37.13 | 104.18.37.13 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 586 | na.877774.xyz | 104.18.187.25 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 622 | cu.877774.xyz | 104.26.4.115 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 660 | bestcf.030101.xyz | 2606:4700:0:dd:df95:6eb1:ffa4:6779 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 719 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 769 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 783 | silkbook.com | 172.67.75.208 | IPv4 | h2 | ✅ 成功 | 36 | cloudflare |
| 792 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 | 36 | cloudflare |
| 34 | dnschecker.org | 104.26.7.89 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 47 | 104.18.14.76 | 104.18.14.76 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 67 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 75 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 145 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 152 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 164 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 169 | 172.64.91.69 | 172.64.91.69 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 179 | 162.159.137.204 | 162.159.137.204 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 183 | 104.26.8.117 | 104.26.8.117 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 205 | 104.17.167.134 | 104.17.167.134 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 213 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 245 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 311 | 104.19.50.35 | 104.19.50.35 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 338 | 104.16.153.12 | 104.16.153.12 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 339 | 104.16.147.114 | 104.16.147.114 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 386 | 104.18.40.216 | 104.18.40.216 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 400 | 104.19.34.231 | 104.19.34.231 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 432 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 443 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 463 | 172.67.78.67 | 172.67.78.67 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 529 | 104.20.25.181 | 104.20.25.181 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 555 | 2a06:98c1:3108:0:edda:98f0:da65:4271 | 2a06:98c1:3108:0:edda:98f0:da65:4271 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 560 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 581 | www.4chan.org | 104.16.229.229 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 590 | cf.877771.xyz | 172.67.152.183 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 619 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | h2 | ✅ 成功 | 37 | cloudflare |
| 642 | asia.877774.xyz | 104.16.211.153 | IPv4 | h2 | ✅ 成功 | 37 | cloudflare |
| 10 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 39 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 56 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 91 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 95 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 184 | 162.159.140.116 | 162.159.140.116 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 185 | 104.18.81.19 | 104.18.81.19 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 209 | 104.16.65.1 | 104.16.65.1 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 221 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 494 | 104.18.36.1 | 104.18.36.1 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 521 | 172.64.229.134 | 172.64.229.134 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 773 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 38 | cloudflare |
| 70 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 194 | 198.41.208.224 | 198.41.208.224 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 199 | 104.19.220.22 | 104.19.220.22 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 276 | 104.17.105.198 | 104.17.105.198 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 363 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 469 | iplocation.io | 104.26.10.222 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 481 | ct.877774.xyz | 172.64.229.195 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 489 | 104.17.25.87 | 104.17.25.87 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 541 | 172.64.153.140 | 172.64.153.140 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 548 | 162.159.38.134 | 162.159.38.134 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 695 | cf.090227.xyz | 104.18.43.174 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 739 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 39 | cloudflare |
| 757 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h2 | ✅ 成功 | 39 | cloudflare |
| 73 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 231 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 317 | 104.18.35.166 | 104.18.35.166 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 340 | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 350 | 162.159.19.219 | 162.159.19.219 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 380 | 172.64.42.158 | 172.64.42.158 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 388 | 104.20.21.147 | 104.20.21.147 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 409 | 162.159.21.222 | 162.159.21.222 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 496 | 104.18.32.174 | 104.18.32.174 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 579 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | h2 | ✅ 成功 | 40 | cloudflare |
| 79 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 210 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | h2 | ✅ 成功 | 41 | cloudflare |
| 238 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 373 | 162.159.41.141 | 162.159.41.141 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 451 | 172.64.154.113 | 172.64.154.113 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 490 | 104.18.44.25 | 104.18.44.25 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 674 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 717 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 743 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 744 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 776 | singapore.com | 172.67.75.194 | IPv4 | h2 | ✅ 成功 | 42 | cloudflare |
| 146 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 167 | 172.64.52.127 | 172.64.52.127 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 290 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 446 | cf.0sm.com | 104.21.7.133 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 740 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 348 | 162.159.45.176 | 162.159.45.176 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 454 | 162.159.36.223 | 162.159.36.223 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 725 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h2 | ✅ 成功 | 44 | cloudflare |
| 280 | 172.64.53.0 | 172.64.53.0 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 282 | 108.162.194.125 | 108.162.194.125 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 285 | 162.159.39.62 | 162.159.39.62 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 515 | 108.162.198.85 | 108.162.198.85 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 520 | 162.159.44.101 | 162.159.44.101 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 524 | 162.159.36.205 | 162.159.36.205 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 542 | 104.25.245.233 | 104.25.245.233 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 616 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 618 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 646 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 706 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 139 | 108.162.198.54 | 108.162.198.54 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 247 | 162.159.21.116 | 162.159.21.116 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 367 | 162.159.39.177 | 162.159.39.177 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 415 | 162.159.45.165 | 162.159.45.165 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 491 | 104.16.251.254 | 104.16.251.254 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 492 | 104.17.214.136 | 104.17.214.136 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 617 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 696 | cf.090227.xyz | 172.64.144.82 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 707 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 788 | www.whatismyip.com | 104.26.13.23 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 53 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 59 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 407 | 162.159.42.146 | 162.159.42.146 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 564 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 700 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 32 | dnschecker.org | 172.67.73.216 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 141 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 211 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 233 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 248 | 162.159.6.115 | 162.159.6.115 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 316 | 162.159.34.55 | 162.159.34.55 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 547 | 162.159.3.128 | 162.159.3.128 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 597 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 632 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 652 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 730 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 731 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 458 | 172.64.32.77 | 172.64.32.77 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 497 | 172.64.40.196 | 172.64.40.196 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 554 | 162.159.1.39 | 162.159.1.39 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 596 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 253 | 173.245.58.237 | 173.245.58.237 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 518 | 172.64.53.165 | 172.64.53.165 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 708 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 552 | 172.64.53.202 | 172.64.53.202 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 318 | 162.159.42.140 | 162.159.42.140 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 371 | 172.64.53.181 | 172.64.53.181 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 631 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 766 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 408 | 172.64.52.67 | 172.64.52.67 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 150 | 198.41.194.162 | 198.41.194.162 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 466 | 172.67.67.152 | 172.67.67.152 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 324 | 162.159.19.37 | 162.159.19.37 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 377 | 162.159.3.89 | 162.159.3.89 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 551 | 162.159.39.180 | 162.159.39.180 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 493 | 104.17.187.186 | 104.17.187.186 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 648 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 673 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 729 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 724 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 359 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 354 | 162.159.44.128 | 162.159.44.128 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 358 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 553 | 198.41.222.191 | 198.41.222.191 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 284 | 172.64.52.110 | 172.64.52.110 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 702 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 13 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 107 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 171 | 162.159.24.131 | 162.159.24.131 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 369 | 162.159.45.145 | 162.159.45.145 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 498 | 162.159.1.145 | 162.159.1.145 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 189 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 517 | 162.159.45.65 | 162.159.45.65 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare |
| 705 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 118 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 655 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 656 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 62 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 111 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 112 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 191 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 289 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 294 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 514 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 561 | 2a06:98c1:51:6e:e874:db4f:a1d5:2163 | 2a06:98c1:51:6e:e874:db4f:a1d5:2163 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 634 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 676 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 711 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare |
| 29 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 46 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 418 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 559 | 2803:f800:51:0:fc87:e2d6:88c3:378b | 2803:f800:51:0:fc87:e2d6:88c3:378b | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 689 | cf.zhetengsha.eu.org | 2a06:98c1:3105::6812:230f | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 710 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 718 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | h2 | ✅ 成功 | 74 | cloudflare |
| 21 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 61 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 63 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 110 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 120 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 459 | 162.159.16.136 | 162.159.16.136 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 509 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 563 | 2a06:98c1:51::ee:b8fb:877f | 2a06:98c1:51::ee:b8fb:877f | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 677 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 704 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 720 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 747 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | h2 | ✅ 成功 | 75 | cloudflare |
| 119 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 193 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 422 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 599 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 653 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 657 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 734 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | h2 | ✅ 成功 | 76 | cloudflare |
| 17 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 108 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 192 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 678 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 748 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 758 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 759 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | h2 | ✅ 成功 | 77 | cloudflare |
| 22 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 60 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 190 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 355 | 162.159.38.35 | 162.159.38.35 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 426 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 703 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | h2 | ✅ 成功 | 78 | cloudflare |
| 188 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 651 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | h2 | ✅ 成功 | 80 | cloudflare |
| 745 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 12 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 277 | 108.162.198.48 | 108.162.198.48 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 732 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | h2 | ✅ 成功 | 81 | cloudflare |
| 14 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 20 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 44 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 249 | 108.162.192.66 | 108.162.192.66 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 636 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | h2 | ✅ 成功 | 82 | cloudflare |
| 170 | 172.64.48.226 | 172.64.48.226 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 356 | 172.64.53.144 | 172.64.53.144 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 370 | 172.64.52.90 | 172.64.52.90 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 414 | 108.162.198.148 | 108.162.198.148 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 27 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h2 | ✅ 成功 | 84 | cloudflare |
| 45 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | h2 | ✅ 成功 | 84 | cloudflare |
| 756 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h2 | ✅ 成功 | 84 | cloudflare |
| 283 | 162.159.45.93 | 162.159.45.93 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 319 | 108.162.195.1 | 108.162.195.1 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 368 | 162.159.38.171 | 162.159.38.171 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 633 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 742 | 162.159.36.104 | 162.159.36.104 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 41 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 58 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 116 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 246 | 162.159.17.243 | 162.159.17.243 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 675 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 727 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | h2 | ✅ 成功 | 86 | cloudflare |
| 43 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 109 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 281 | 172.64.50.51 | 172.64.50.51 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 372 | 172.64.34.153 | 172.64.34.153 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 374 | 108.162.198.170 | 108.162.198.170 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 410 | 162.159.39.136 | 162.159.39.136 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 411 | 172.64.53.40 | 172.64.53.40 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 701 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 142 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 251 | 162.159.46.238 | 162.159.46.238 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 522 | 162.159.39.146 | 162.159.39.146 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 595 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 654 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 546 | 162.159.11.128 | 162.159.11.128 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 715 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 117 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 378 | 162.159.18.240 | 162.159.18.240 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 381 | 162.159.22.29 | 162.159.22.29 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 647 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 716 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 412 | 162.159.44.133 | 162.159.44.133 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 279 | 162.159.38.192 | 162.159.38.192 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 500 | 162.159.36.52 | 162.159.36.52 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 519 | 172.64.52.189 | 172.64.52.189 | IPv4 | h2 | ✅ 成功 | 92 | cloudflare |
| 357 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | h2 | ✅ 成功 | 94 | cloudflare |
| 278 | 162.159.44.176 | 162.159.44.176 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 516 | 162.159.38.226 | 162.159.38.226 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 550 | 162.159.58.17 | 162.159.58.17 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 349 | 108.162.198.69 | 108.162.198.69 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 495 | 172.64.49.146 | 172.64.49.146 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 143 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 351 | 162.159.39.99 | 162.159.39.99 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 723 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h2 | ✅ 成功 | 101 | cloudflare |
| 549 | 162.159.12.120 | 162.159.12.120 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 28 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 755 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 19 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h2 | ✅ 成功 | 111 | cloudflare |
| 26 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 450 | 162.159.6.44 | 162.159.6.44 | IPv4 | h2 | ✅ 成功 | 114 | cloudflare |
| 115 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h2 | ✅ 成功 | 115 | cloudflare |
| 268 | 162.159.0.115 | 162.159.0.115 | IPv4 | h2 | ✅ 成功 | 115 | cloudflare |
| 42 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h2 | ✅ 成功 | 116 | cloudflare |
| 321 | 162.159.36.26 | 162.159.36.26 | IPv4 | h2 | ✅ 成功 | 116 | cloudflare |
| 352 | 172.64.52.15 | 172.64.52.15 | IPv4 | h2 | ✅ 成功 | 116 | cloudflare |
| 413 | 162.159.38.67 | 162.159.38.67 | IPv4 | h2 | ✅ 成功 | 116 | cloudflare |
| 250 | 162.159.13.51 | 162.159.13.51 | IPv4 | h2 | ✅ 成功 | 117 | cloudflare |
| 353 | 162.159.7.12 | 162.159.7.12 | IPv4 | h2 | ✅ 成功 | 118 | cloudflare |
| 18 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h2 | ✅ 成功 | 119 | cloudflare |
| 176 | 162.159.58.65 | 162.159.58.65 | IPv4 | h2 | ✅ 成功 | 120 | cloudflare |
| 94 | 172.64.35.24 | 172.64.35.24 | IPv4 | h2 | ✅ 成功 | 121 | cloudflare |
| 556 | 108.162.198.70 | 108.162.198.70 | IPv4 | h2 | ✅ 成功 | 123 | cloudflare |
| 382 | 162.159.1.111 | 162.159.1.111 | IPv4 | h2 | ✅ 成功 | 124 | cloudflare |
| 1 | 172.64.154.18 | 172.64.154.18 | IPv4 | h2 | ✅ 成功 | 287 | cloudflare |
| 2 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 288 | cloudflare |
| 699 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 430 | cloudflare |
| 567 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 438 | cloudflare |
| 419 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 470 | cloudflare |
| 137 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 1954 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 617 条记录
- **快 (50-100ms)**: 146 条记录
- **正常 (100-200ms)**: 21 条记录
- **慢 (200-500ms)**: 5 条记录
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
