# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/2/25 04:44:31
- **数据来源**: connectivity_results-20260225-044430.json
- **总测试数**: 835
- **失败测试数**: 4
- **成功测试数**: 831
- **失败率**: 0.48%
- **平均延迟**: 64.77ms
- **最小延迟**: 38ms
- **最大延迟**: 673ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/2/25 04:44:31
- **IP地址**: 2a09:bac1:7680:8e0::1bd:f8
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

- **连接超时: I/O超时**: 4 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (4 次测试)

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息 |
|------|-----------|--------|--------|------|--------|----------|--------|----------|
| 118 | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.212.130:443: i/o timeout |
| 271 | braden.ns.cloudflare.com | 108.162.195.169 | IPv4 | none | N/A | 0 | N/A | dial tcp 108.162.195.169:443: i/o timeout |
| 378 | 172.64.201.25 | 172.64.201.25 | IPv4 | none | N/A | 0 | N/A | dial tcp 172.64.201.25:443: i/o timeout |
| 458 | 198.41.194.162 | 198.41.194.162 | IPv4 | none | N/A | 0 | N/A | dial tcp 198.41.194.162:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 4 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 4 次超时，主要集中在IP段 198.41（2 次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 4 次失败是因为协议协商失败（protocol: none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: 所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好



---

## 🚀 延迟最低的 831 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|
| 482 | 104.31.16.158 | 104.31.16.158 | IPv4 | h2 | ✅ 成功 | 38 | cloudflare |
| 344 | cf.877774.xyz | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 40 | cloudflare |
| 160 | toy-people.com | 104.26.2.36 | IPv4 | h2 | ✅ 成功 | 41 | cloudflare |
| 487 | 104.18.189.153 | 104.18.189.153 | IPv4 | h2 | ✅ 成功 | 43 | cloudflare |
| 495 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6 | h2 | ✅ 成功 | 43 | cloudflare |
| 161 | toy-people.com | 104.26.3.36 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 288 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 | h2 | ✅ 成功 | 45 | cloudflare |
| 340 | dnschecker.org | 104.26.6.89 | IPv4 | h2 | ✅ 成功 | 45 | cloudflare |
| 75 | iplocation.io | 2606:4700:20::681a:ade | IPv6 | h2 | ✅ 成功 | 46 | cloudflare |
| 103 | www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 383 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 648 | 172.64.229.134 | 172.64.229.134 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 689 | 104.17.193.113 | 104.17.193.113 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 714 | 104.26.5.101 | 104.26.5.101 | IPv4 | h2 | ✅ 成功 | 46 | cloudflare |
| 13 | www.pcmag.com | 104.16.20.118 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 18 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 22 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 67 | 172.67.49.134 | 172.67.49.134 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 68 | cu.877774.xyz | 104.18.42.54 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 72 | iplocation.io | 104.26.10.222 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 91 | www.visa.cn | 162.159.153.2 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 170 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 179 | asia.877774.xyz | 104.17.139.62 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 225 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 235 | 172.64.151.55 | 172.64.151.55 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 244 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 246 | fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 286 | palera.in | 172.67.157.122 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 292 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 309 | ip.gs | 104.21.14.176 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 341 | dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 353 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 519 | 104.26.3.176 | 104.26.3.176 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 557 | 104.26.12.227 | 104.26.12.227 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 668 | cloudflare.182682.xyz | 2606:4700:e7::3151:47a9 | IPv6 | h2 | ✅ 成功 | 47 | cloudflare |
| 726 | 104.26.10.239 | 104.26.10.239 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 794 | 104.26.1.194 | 104.26.1.194 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 802 | 104.25.240.123 | 104.25.240.123 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 808 | 104.20.17.51 | 104.20.17.51 | IPv4 | h2 | ✅ 成功 | 47 | cloudflare |
| 7 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 49 | ct.877774.xyz | 172.64.229.185 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 80 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 81 | icook.hk | 2606:4700:3037::ac43:a168 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 92 | www.okcupid.com | 104.16.223.254 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 105 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 123 | cfip.xxxxxxxx.tk | 104.18.228.35 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 131 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 141 | na.877774.xyz | 104.18.38.235 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 158 | cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 163 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 166 | zread.ai | 104.21.76.240 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 185 | kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 218 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 219 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 222 | cmcc.877774.xyz | 104.16.148.4 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 223 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 232 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 233 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 238 | cf.zhetengsha.eu.org | 104.18.42.98 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 239 | cf.zhetengsha.eu.org | 2606:4700:4407::ac40:9052 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 248 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 251 | cf.090227.xyz | 104.18.42.98 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 268 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 269 | www.glassdoor.com | 104.16.25.46 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 270 | www.glassdoor.com | 104.17.64.70 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 290 | time.is | 104.26.13.54 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 291 | time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 303 | ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 314 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 318 | singapore.com | 104.26.12.140 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 330 | www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 343 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 371 | tasteatlas.com | 104.17.37.105 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 389 | [2606:4700:83be::11:74f] | 2606:4700:83be::11:74f | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 397 | 104.19.223.58 | 104.19.223.58 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 403 | ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 446 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 48 | cloudflare |
| 477 | 104.19.220.22 | 104.19.220.22 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 483 | 104.17.167.134 | 104.17.167.134 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 555 | 104.26.4.190 | 104.26.4.190 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 556 | 104.20.18.47 | 104.20.18.47 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 558 | 104.20.19.201 | 104.20.19.201 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 563 | 104.17.119.199 | 104.17.119.199 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 594 | 104.18.32.174 | 104.18.32.174 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 652 | 104.20.28.239 | 104.20.28.239 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 653 | 104.26.6.247 | 104.26.6.247 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 654 | 104.26.14.117 | 104.26.14.117 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 674 | 104.20.20.192 | 104.20.20.192 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 683 | 172.64.145.119 | 172.64.145.119 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 687 | 104.25.241.198 | 104.25.241.198 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 694 | 104.18.42.61 | 104.18.42.61 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 696 | 104.17.111.150 | 104.17.111.150 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 725 | 172.67.72.36 | 172.67.72.36 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 772 | 172.67.67.0 | 172.67.67.0 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 787 | 104.18.41.101 | 104.18.41.101 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 806 | 172.67.79.150 | 172.67.79.150 | IPv4 | h2 | ✅ 成功 | 48 | cloudflare |
| 15 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 19 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 56 | 104.18.37.13 | 104.18.37.13 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 65 | 103.160.204.59 | 103.160.204.59 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 97 | cf.877771.xyz | 104.21.80.180 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 104 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 107 | www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 112 | cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 122 | cfip.xxxxxxxx.tk | 104.17.127.110 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 124 | cfip.xxxxxxxx.tk | 104.20.255.53 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 146 | freeyx.cloudflare88.eu.org | 2606:4700:3010:0:51:d1e4:65f0:5ed1 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 164 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 165 | zread.ai | 172.67.202.78 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 171 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 177 | decker.ns.cloudflare.com | 2606:4700:58::a29f:2c9b | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 180 | asia.877774.xyz | 104.17.142.146 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 196 | www.whatismyip.com | 104.26.12.23 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 207 | bestcf.030101.xyz | 2606:4700:0:dd:df95:6eb1:ffa4:6779 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 221 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 237 | cf.zhetengsha.eu.org | 172.64.145.158 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 250 | cf.090227.xyz | 172.64.145.158 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 302 | ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 304 | ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 306 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 312 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 316 | whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 319 | singapore.com | 104.26.13.140 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 329 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 332 | 104.18.42.26 | 104.18.42.26 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 336 | 162.159.133.85 | 162.159.133.85 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 345 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 360 | gamer.com.tw | 104.18.3.197 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 370 | tasteatlas.com | 104.17.36.105 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 394 | www.digitalocean.com | 2606:4700::6813:ad44 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 399 | eur.877774.xyz | 104.21.29.164 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 402 | ifconfig.co | 172.67.168.106 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 420 | 172.64.82.114 | 172.64.82.114 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 443 | japan.com | 104.26.5.60 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 457 | 104.26.5.134 | 104.26.5.134 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 461 | 104.26.3.162 | 104.26.3.162 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 469 | 162.159.140.116 | 162.159.140.116 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 474 | 104.17.162.3 | 104.17.162.3 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 480 | 104.19.212.207 | 104.19.212.207 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 504 | 104.17.101.37 | 104.17.101.37 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 512 | 172.67.74.57 | 172.67.74.57 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 516 | 104.20.24.107 | 104.20.24.107 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 528 | 104.17.105.198 | 104.17.105.198 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 551 | 104.26.8.148 | 104.26.8.148 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 559 | 104.17.60.233 | 104.17.60.233 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 566 | 104.16.155.76 | 104.16.155.76 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 570 | 104.18.35.166 | 104.18.35.166 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 608 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 610 | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 620 | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 665 | cloudflare.182682.xyz | 104.21.227.134 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 666 | cloudflare.182682.xyz | 2606:4700:8ca0::3dc4:21a2 | IPv6 | h2 | ✅ 成功 | 49 | cloudflare |
| 685 | 104.26.13.110 | 104.26.13.110 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 697 | 172.64.153.140 | 172.64.153.140 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 699 | 104.18.166.232 | 104.18.166.232 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 701 | 104.25.250.174 | 104.25.250.174 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 716 | 104.26.3.120 | 104.26.3.120 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 718 | 172.67.75.11 | 172.67.75.11 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 771 | 172.67.70.56 | 172.67.70.56 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 778 | 172.64.152.215 | 172.64.152.215 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 783 | 104.20.24.244 | 104.20.24.244 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 784 | 104.20.22.91 | 104.20.22.91 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 793 | 104.26.11.160 | 104.26.11.160 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 803 | 104.25.249.225 | 104.25.249.225 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 828 | 172.64.153.183 | 172.64.153.183 | IPv4 | h2 | ✅ 成功 | 49 | cloudflare |
| 20 | ipinfo.in | 172.67.198.203 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 23 | ipinfo.in | 2606:4700:3031::6815:1581 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 52 | steamdb.info | 172.66.175.250 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 57 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 63 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 69 | cu.877774.xyz | 172.64.145.202 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 77 | www.4chan.org | 104.16.228.229 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 78 | icook.hk | 104.21.90.210 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 93 | www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 101 | www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 113 | cfip.xxxxxxxx.tk | 188.114.96.125 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 128 | sullivan.ns.cloudflare.com | 2606:4700:58::a29f:2ca1 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 145 | freeyx.cloudflare88.eu.org | 2606:4700:3009:aa59:4b67:cdf7:f50d:f763 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 152 | pranab.ns.cloudflare.com | 2606:4700:58::a29f:2cc7 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 169 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 197 | www.whatismyip.com | 172.67.69.129 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 203 | saas.sin.fan | 162.159.36.5 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 208 | bestcf.030101.xyz | 2606:4700:0:d9:2acf:b5e0:5a46:4358 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 229 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 252 | cf.090227.xyz | 2606:4700:440a::ac40:98f1 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 266 | www.ipchicken.com | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 267 | www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 287 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 289 | time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 300 | benedict.ns.cloudflare.com | 2606:4700:58::a29f:2ccd | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 305 | ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 307 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 310 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 317 | whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 323 | singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 331 | www.visa.com.hk | 104.18.21.69 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 338 | dnschecker.org | 104.26.7.89 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 377 | www.udemy.com | 2606:4700::6810:8eed | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 381 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 423 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 428 | 162.159.62.6 | 162.159.62.6 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 452 | 173.245.49.194 | 173.245.49.194 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 478 | 104.19.154.200 | 104.19.154.200 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 479 | 104.18.255.167 | 104.18.255.167 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 481 | 104.17.69.244 | 104.17.69.244 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 484 | 104.18.223.253 | 104.18.223.253 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 486 | 104.17.142.212 | 104.17.142.212 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 492 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 493 | 2a06:98c1:3121:0:efde:82d1:8124:3fed | 2a06:98c1:3121:0:efde:82d1:8124:3fed | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 496 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 498 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 509 | 172.64.150.30 | 172.64.150.30 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 511 | 104.26.15.85 | 104.26.15.85 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 518 | 172.67.72.254 | 172.67.72.254 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 545 | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 568 | 104.16.157.50 | 104.16.157.50 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 598 | 104.18.42.106 | 104.18.42.106 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 642 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | 2a06:98c1:3104:0:4:5eb4:7182:42a0 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 643 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 644 | 2a06:98c1:3100:0:a3:1339:d974:e2c | 2a06:98c1:3100:0:a3:1339:d974:e2c | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 649 | 104.18.42.16 | 104.18.42.16 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 651 | 104.26.1.88 | 104.26.1.88 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 663 | cloudflare.182682.xyz | 104.18.185.26 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 670 | cloudflare.182682.xyz | 2606:4700:3035::1a4f:5642 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 673 | 104.26.8.171 | 104.26.8.171 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 688 | 104.25.244.239 | 104.25.244.239 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 695 | 104.18.172.20 | 104.18.172.20 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 719 | 104.25.245.215 | 104.25.245.215 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 728 | 104.17.170.137 | 104.17.170.137 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 732 | 104.19.153.47 | 104.19.153.47 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 751 | 162.159.61.106 | 162.159.61.106 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 755 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85 | IPv6 | h2 | ✅ 成功 | 50 | cloudflare |
| 790 | 104.18.148.235 | 104.18.148.235 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 799 | 104.25.247.78 | 104.25.247.78 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 800 | 104.25.244.36 | 104.25.244.36 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 804 | 104.25.254.14 | 104.25.254.14 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 810 | 104.26.7.7 | 104.26.7.7 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 815 | 104.16.155.230 | 104.16.155.230 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 816 | 104.17.25.87 | 104.17.25.87 | IPv4 | h2 | ✅ 成功 | 50 | cloudflare |
| 6 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 12 | www.pcmag.com | 104.16.21.118 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 14 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 21 | ipinfo.in | 104.21.21.129 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 61 | www.gov.ua | 104.21.23.72 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 66 | 104.18.254.88 | 104.18.254.88 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 74 | iplocation.io | 2606:4700:20::681a:bde | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 76 | www.4chan.org | 104.16.229.229 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 79 | icook.hk | 172.67.161.104 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 90 | www.visa.cn | 162.159.152.2 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 100 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 102 | www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 142 | na.877774.xyz | 104.19.74.233 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 144 | freeyx.cloudflare88.eu.org | 141.101.121.110 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 157 | cris.ns.cloudflare.com | 2a06:98c1:50::ac40:23ca | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 192 | dylan.ns.cloudflare.com | 2606:4700:58::a29f:2cbb | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 195 | www.whatismyip.com | 104.26.13.23 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 201 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 204 | saas.sin.fan | 162.159.36.20 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 205 | bestcf.030101.xyz | 104.17.222.192 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 212 | cmcc.877774.xyz | 104.16.148.9 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 213 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 217 | cmcc.877774.xyz | 104.16.148.2 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 224 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 242 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 245 | fbi.gov | 104.16.148.244 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 265 | rustam.ns.cloudflare.com | 2a06:98c1:50::ac40:2394 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 311 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 320 | singapore.com | 172.67.75.194 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 321 | singapore.com | 2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 325 | silkbook.com | 104.26.9.160 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 337 | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 379 | 104.17.142.12 | 104.17.142.12 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 396 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 422 | www.csgo.com | 195.85.59.161 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 432 | stock.hostmonit.com | 2606:4700:3037::6815:7c1 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 439 | 198.41.208.15 | 198.41.208.15 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 449 | 172.64.91.69 | 172.64.91.69 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 468 | 104.26.8.117 | 104.26.8.117 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 471 | 104.18.89.52 | 104.18.89.52 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 473 | 104.19.148.121 | 104.19.148.121 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 476 | 104.17.139.37 | 104.17.139.37 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 491 | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 506 | 104.18.47.253 | 104.18.47.253 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 547 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | 2803:f800:51:6a7b:7c95:5585:9678:1549 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 549 | 104.20.26.58 | 104.20.26.58 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 567 | 104.17.100.254 | 104.17.100.254 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 573 | 104.18.37.110 | 104.18.37.110 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 616 | 104.16.251.143 | 104.16.251.143 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 637 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 669 | cloudflare.182682.xyz | 2a06:98c1:3120::5692:61a4 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 680 | 172.64.145.242 | 172.64.145.242 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 693 | 172.64.146.67 | 172.64.146.67 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 698 | 104.25.245.233 | 104.25.245.233 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 713 | 104.20.21.147 | 104.20.21.147 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 720 | 2a06:98c1:310d:85:ac4c:8137:506:5188 | 2a06:98c1:310d:85:ac4c:8137:506:5188 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 724 | 104.20.25.82 | 104.20.25.82 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 766 | 172.64.229.156 | 172.64.229.156 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 773 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | 2a06:98c1:310b:0:e474:ff3f:ec26:c616 | IPv6 | h2 | ✅ 成功 | 51 | cloudflare |
| 798 | 104.17.56.208 | 104.17.56.208 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 807 | 172.67.67.152 | 172.67.67.152 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 811 | 104.17.153.151 | 104.17.153.151 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 814 | 104.17.110.226 | 104.17.110.226 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 817 | 104.25.250.205 | 104.25.250.205 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 821 | 172.64.229.15 | 172.64.229.15 | IPv4 | h2 | ✅ 成功 | 51 | cloudflare |
| 24 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 41 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 42 | shopify.com | 23.227.38.33 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 73 | iplocation.io | 2606:4700:20::ac43:4664 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 111 | cfip.xxxxxxxx.tk | 104.25.105.1 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 143 | na.877774.xyz | 104.18.187.25 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 167 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 168 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 172 | cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 181 | asia.877774.xyz | 104.16.211.153 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 194 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 202 | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 206 | bestcf.030101.xyz | 104.19.153.222 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 215 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 216 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 227 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 241 | xn--b6gac.eu.org | 104.21.90.78 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 247 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 282 | moura.ns.cloudflare.com | 2a06:98c1:50::ac40:23d9 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 339 | dnschecker.org | 172.67.73.216 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 361 | gamer.com.tw | 104.18.2.197 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 390 | 104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 391 | 104.18.78.214 | 104.18.78.214 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 411 | 172.67.181.209 | 172.67.181.209 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 418 | 104.19.175.123 | 104.19.175.123 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 424 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 427 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 447 | japan.com | 2606:4700:20::681a:53c | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 448 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 459 | 162.159.137.204 | 162.159.137.204 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 472 | 104.18.166.129 | 104.18.166.129 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 485 | 104.16.105.166 | 104.16.105.166 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 488 | 104.16.65.1 | 104.16.65.1 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 514 | 172.67.65.159 | 172.67.65.159 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 517 | 104.20.30.198 | 104.20.30.198 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 546 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 552 | 172.67.67.5 | 172.67.67.5 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 553 | 104.20.21.161 | 104.20.21.161 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 562 | 104.18.40.39 | 104.18.40.39 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 565 | 104.17.101.208 | 104.17.101.208 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 575 | 104.18.39.228 | 104.18.39.228 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 585 | 104.26.2.166 | 104.26.2.166 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 586 | 104.20.16.244 | 104.20.16.244 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 588 | 104.26.6.159 | 104.26.6.159 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 597 | 104.18.47.46 | 104.18.47.46 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 601 | 172.64.154.226 | 172.64.154.226 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 607 | 104.17.30.164 | 104.17.30.164 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 638 | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 679 | 172.64.151.235 | 172.64.151.235 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 692 | 172.64.153.141 | 172.64.153.141 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 715 | 172.67.73.120 | 172.67.73.120 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 717 | 104.26.1.181 | 104.26.1.181 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 748 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 752 | 2a06:98c1:51::c0bc:f0fe:59ba | 2a06:98c1:51::c0bc:f0fe:59ba | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 782 | 104.26.11.33 | 104.26.11.33 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 786 | 172.67.72.212 | 172.67.72.212 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 788 | 104.20.19.180 | 104.20.19.180 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 812 | 104.19.144.159 | 104.19.144.159 | IPv4 | h2 | ✅ 成功 | 52 | cloudflare |
| 826 | 2a06:98c1:3104:da84:1c63:f149:4d21:b339 | 2a06:98c1:3104:da84:1c63:f149:4d21:b339 | IPv6 | h2 | ✅ 成功 | 52 | cloudflare |
| 1 | 104.26.12.113 | 104.26.12.113 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 43 | ct.877774.xyz | 172.64.229.217 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 51 | 172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 109 | 172.67.243.218 | 172.67.243.218 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 114 | cfip.xxxxxxxx.tk | 188.114.97.144 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 116 | cfip.xxxxxxxx.tk | 190.93.246.67 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 132 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 159 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 178 | decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 187 | kyree.ns.cloudflare.com | 2a06:98c1:50::ac40:23cf | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 200 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 226 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 240 | cf.zhetengsha.eu.org | 2a06:98c1:310d::6812:2bae | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 294 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 346 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 357 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 376 | www.udemy.com | 2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 398 | eur.877774.xyz | 104.21.26.150 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 421 | www.7749tv.com | 104.19.13.63 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 431 | stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 440 | 162.159.140.85 | 162.159.140.85 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 489 | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 508 | 104.18.45.95 | 104.18.45.95 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 520 | 104.17.169.180 | 104.17.169.180 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 543 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | 2a06:98c1:310d:4c:4b41:a84:50ee:e810 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 561 | 104.19.44.238 | 104.19.44.238 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 580 | 2a06:98c1:3108:0:edda:98f0:da65:4271 | 2a06:98c1:3108:0:edda:98f0:da65:4271 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 590 | 172.67.70.253 | 172.67.70.253 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 617 | 104.17.53.25 | 104.17.53.25 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 645 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 655 | 2a06:98c1:3106::c5:5d39:736d | 2a06:98c1:3106::c5:5d39:736d | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 686 | 104.17.97.228 | 104.17.97.228 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 711 | 104.26.2.2 | 104.26.2.2 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 721 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 727 | 104.17.211.218 | 104.17.211.218 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 731 | 104.17.215.66 | 104.17.215.66 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 753 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793 | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 768 | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | 2a06:98c1:310c:6a:19f2:494:88cc:d5f | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 776 | 172.64.154.113 | 172.64.154.113 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 801 | 104.25.241.85 | 104.25.241.85 | IPv4 | h2 | ✅ 成功 | 53 | cloudflare |
| 834 | 2a06:98c1:3107:54:2c60:eafc:f14d:ca4b | 2a06:98c1:3107:54:2c60:eafc:f14d:ca4b | IPv6 | h2 | ✅ 成功 | 53 | cloudflare |
| 10 | comicabc.com | 2606:4700:3030::ac43:ae15 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 16 | cf.0sm.com | 172.67.187.145 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 45 | ct.877774.xyz | 172.64.229.44 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 47 | ct.877774.xyz | 172.64.229.173 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 48 | ct.877774.xyz | 172.64.229.174 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 50 | ct.877774.xyz | 172.64.229.195 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 94 | www.okcupid.com | 104.18.160.63 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 98 | cf.877771.xyz | 172.67.152.183 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 108 | www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 133 | yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 134 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 188 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 234 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 295 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 301 | benedict.ns.cloudflare.com | 2803:f800:50::6ca2:c3cd | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 308 | ip.gs | 172.67.160.28 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 324 | silkbook.com | 172.67.75.208 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 327 | silkbook.com | 2606:4700:20::681a:9a0 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 333 | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 342 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 375 | www.udemy.com | 104.16.143.237 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 385 | icook.tw | 104.20.28.74 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 386 | icook.tw | 172.66.158.115 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 414 | damien.ns.cloudflare.com | 162.159.44.168 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 426 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 430 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 442 | 162.159.61.183 | 162.159.61.183 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 456 | 172.64.229.7 | 172.64.229.7 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 460 | 162.159.128.253 | 162.159.128.253 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 539 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 550 | 172.67.77.196 | 172.67.77.196 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 564 | 104.19.50.35 | 104.19.50.35 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 581 | 104.16.251.254 | 104.16.251.254 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 582 | 104.17.187.186 | 104.17.187.186 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 615 | 104.17.154.254 | 104.17.154.254 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 619 | 104.16.144.235 | 104.16.144.235 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 661 | cloudflare.182682.xyz | 104.16.250.22 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 664 | cloudflare.182682.xyz | 104.21.224.5 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 672 | 104.26.0.210 | 104.26.0.210 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 700 | 104.18.40.216 | 104.18.40.216 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 735 | 104.17.25.241 | 104.17.25.241 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 736 | 104.17.189.30 | 104.17.189.30 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 741 | 172.64.53.40 | 172.64.53.40 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 757 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | 2a06:98c1:310a:0:15:735e:c4e:e2f7 | IPv6 | h2 | ✅ 成功 | 54 | cloudflare |
| 780 | 172.64.145.108 | 172.64.145.108 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 781 | 104.18.37.177 | 104.18.37.177 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 805 | 104.20.25.161 | 104.20.25.161 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 809 | 104.26.4.4 | 104.26.4.4 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 829 | 172.64.144.132 | 172.64.144.132 | IPv4 | h2 | ✅ 成功 | 54 | cloudflare |
| 4 | www.ipget.net | 172.67.207.26 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 8 | comicabc.com | 104.21.64.10 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 9 | comicabc.com | 172.67.174.21 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 11 | comicabc.com | 2606:4700:3036::6815:400a | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 46 | ct.877774.xyz | 172.64.229.161 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 58 | 172.67.110.232 | 172.67.110.232 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 83 | 172.67.120.0 | 172.67.120.0 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 115 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 209 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 211 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 322 | singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 328 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 335 | 104.18.14.76 | 104.18.14.76 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 362 | [2606:4700:4403::7357:544f] | 2606:4700:4403::7357:544f | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 373 | tasteatlas.com | 2606:4700::6811:2569 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 374 | www.udemy.com | 104.16.142.237 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 387 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 393 | www.digitalocean.com | 104.19.173.68 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 475 | 104.18.151.172 | 104.18.151.172 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 540 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | 2a06:98c1:310e:68:b803:ed16:7e58:d249 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 544 | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 578 | 104.17.214.136 | 104.17.214.136 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 579 | 104.18.44.25 | 104.18.44.25 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 583 | 104.18.36.1 | 104.18.36.1 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 600 | 104.18.40.202 | 104.18.40.202 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 602 | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 603 | 104.17.24.232 | 104.17.24.232 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 662 | cloudflare.182682.xyz | 104.17.25.173 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 676 | 172.64.53.181 | 172.64.53.181 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 730 | 104.19.34.231 | 104.19.34.231 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 740 | 162.159.39.136 | 162.159.39.136 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 785 | 104.26.4.135 | 104.26.4.135 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare |
| 822 | 2a06:98c1:51:a594:2926:2b16:6e8d:843e | 2a06:98c1:51:a594:2926:2b16:6e8d:843e | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 823 | 2a06:98c1:51:aa:3e22:dd48:6279:eeb9 | 2a06:98c1:51:aa:3e22:dd48:6279:eeb9 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 825 | 2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b | 2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 835 | 2a06:98c1:3106:6a:7ba4:346b:e06c:71c7 | 2a06:98c1:3106:6a:7ba4:346b:e06c:71c7 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare |
| 5 | www.ipget.net | 104.21.15.212 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 33 | trevor.ns.cloudflare.com | 2a06:98c1:50::ac40:239a | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 37 | 104.16.45.84 | 104.16.45.84 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 62 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 95 | www.okcupid.com | 104.16.239.254 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 96 | www.okcupid.com | 104.17.48.63 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 106 | www.hugedomains.com | 2606:4700:20::681a:725 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 117 | cfip.xxxxxxxx.tk | 190.93.247.169 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 120 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 162 | toy-people.com | 2606:4700:20::681a:324 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 199 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 228 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 285 | palera.in | 104.21.58.72 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 293 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 315 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 358 | julio.ns.cloudflare.com | 2606:4700:58::a29f:2cd1 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 388 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 395 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 406 | otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 408 | otto.ns.cloudflare.com | 2a06:98c1:50::ac40:2387 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 410 | 198.41.208.224 | 198.41.208.224 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 453 | 104.26.4.90 | 104.26.4.90 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 454 | 162.159.136.89 | 162.159.136.89 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 470 | 104.18.81.19 | 104.18.81.19 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 494 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | 2a06:98c1:3121:0:ef18:6ab0:b648:d756 | IPv6 | h2 | ✅ 成功 | 56 | cloudflare |
| 560 | 104.16.148.187 | 104.16.148.187 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 587 | 172.67.68.252 | 172.67.68.252 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 622 | 172.64.147.235 | 172.64.147.235 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 633 | 162.159.38.226 | 162.159.38.226 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 733 | 104.17.209.79 | 104.17.209.79 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare |
| 2 | 2a06:98c1:3104::f3:8fed:cac0 | 2a06:98c1:3104::f3:8fed:cac0 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 17 | cf.0sm.com | 104.21.7.133 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 38 | 104.18.39.196 | 104.18.39.196 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 59 | [2606:4700:9add::880:52fc] | 2606:4700:9add::880:52fc | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 130 | sullivan.ns.cloudflare.com | 2803:f800:50::6ca2:c3a1 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 230 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 231 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 326 | silkbook.com | 104.26.8.160 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 380 | 172.67.79.211 | 172.67.79.211 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 382 | 198.62.62.4 | 198.62.62.4 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 438 | lewis.ns.cloudflare.com | 2a06:98c1:50::ac40:239f | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 444 | japan.com | 172.67.70.92 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 510 | 172.67.68.211 | 172.67.68.211 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 515 | 104.20.22.185 | 104.20.22.185 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 541 | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 576 | 104.18.44.148 | 104.18.44.148 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 589 | 104.20.17.233 | 104.20.17.233 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 636 | 172.64.52.189 | 172.64.52.189 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 667 | cloudflare.182682.xyz | 2606:4700:3032::818:669e | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 671 | 172.67.65.150 | 172.67.65.150 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 734 | 104.19.35.242 | 104.19.35.242 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 749 | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7 | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7 | IPv6 | h2 | ✅ 成功 | 57 | cloudflare |
| 796 | 172.67.78.67 | 172.67.78.67 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 820 | 162.159.38.83 | 162.159.38.83 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 832 | 172.64.151.253 | 172.64.151.253 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare |
| 71 | iplocation.io | 104.26.11.222 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 82 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 88 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 110 | cfip.xxxxxxxx.tk | 104.21.91.19 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 119 | cfip.xxxxxxxx.tk | 198.41.214.141 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 125 | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 150 | pranab.ns.cloudflare.com | 2a06:98c1:50::ac40:23c7 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 183 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 210 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 236 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 253 | cf.090227.xyz | 2a06:98c1:3105::6812:230f | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 347 | cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 400 | eur.877774.xyz | 104.21.47.209 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 542 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 613 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 723 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 729 | 104.17.115.224 | 104.17.115.224 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 770 | 104.20.31.132 | 104.20.31.132 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 813 | 104.17.53.129 | 104.17.53.129 | IPv4 | h2 | ✅ 成功 | 58 | cloudflare |
| 824 | 2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea | 2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea | IPv6 | h2 | ✅ 成功 | 58 | cloudflare |
| 3 | 172.64.154.18 | 172.64.154.18 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 44 | ct.877774.xyz | 172.64.229.236 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 70 | iplocation.io | 172.67.70.100 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 153 | cris.ns.cloudflare.com | 108.162.195.202 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 243 | xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 260 | rustam.ns.cloudflare.com | 108.162.195.148 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 413 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 417 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 445 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 490 | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 569 | 162.159.34.55 | 162.159.34.55 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 591 | 104.26.8.192 | 104.26.8.192 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 593 | 172.67.76.195 | 172.67.76.195 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 612 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 625 | 172.64.52.15 | 172.64.52.15 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 640 | 2a06:98c1:3105:0:db:557f:8a53:2469 | 2a06:98c1:3105:0:db:557f:8a53:2469 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 681 | 162.159.18.240 | 162.159.18.240 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 690 | 162.159.1.111 | 162.159.1.111 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 758 | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6 | h2 | ✅ 成功 | 59 | cloudflare |
| 795 | 104.26.6.171 | 104.26.6.171 | IPv4 | h2 | ✅ 成功 | 59 | cloudflare |
| 409 | otto.ns.cloudflare.com | 2803:f800:50::6ca2:c387 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare |
| 554 | 104.20.20.42 | 104.20.20.42 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 592 | 104.26.4.44 | 104.26.4.44 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 684 | 172.64.42.158 | 172.64.42.158 | IPv4 | h2 | ✅ 成功 | 60 | cloudflare |
| 39 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 55 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 61 | cloudflare |
| 425 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 571 | 162.159.42.140 | 162.159.42.140 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 604 | 104.17.50.237 | 104.17.50.237 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 606 | 104.16.147.114 | 104.16.147.114 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 789 | 104.26.5.194 | 104.26.5.194 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 99 | cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 173 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 190 | dylan.ns.cloudflare.com | 172.64.35.187 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 198 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 313 | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 392 | www.digitalocean.com | 104.19.174.68 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 614 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 635 | 172.64.53.165 | 172.64.53.165 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 639 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | 2a06:98c1:3101:6cce:1edc:88:628d:fd50 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 677 | 162.159.41.141 | 162.159.41.141 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 761 | 162.159.39.20 | 162.159.39.20 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare |
| 833 | 2a06:98c1:3100:f702:ebbf:618b:76c:9ba7 | 2a06:98c1:3100:f702:ebbf:618b:76c:9ba7 | IPv6 | h2 | ✅ 成功 | 62 | cloudflare |
| 334 | 172.67.106.26 | 172.67.106.26 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 372 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 548 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | 2a06:98c1:3107:0:cf9c:104d:1e03:9644 | IPv6 | h2 | ✅ 成功 | 63 | cloudflare |
| 660 | 104.20.25.181 | 104.20.25.181 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 742 | 162.159.44.133 | 162.159.44.133 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare |
| 738 | 162.159.42.146 | 162.159.42.146 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 777 | 172.64.41.216 | 172.64.41.216 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare |
| 261 | rustam.ns.cloudflare.com | 162.159.44.148 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 364 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 572 | 108.162.195.1 | 108.162.195.1 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 709 | 198.41.222.191 | 198.41.222.191 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |
| 30 | trevor.ns.cloudflare.com | 172.64.35.154 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 54 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 60 | www.gov.ua | 172.67.209.127 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 121 | cfip.xxxxxxxx.tk | 104.16.241.229 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 429 | stock.hostmonit.com | 104.21.7.193 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 522 | 104.17.156.81 | 104.17.156.81 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 529 | 162.159.44.176 | 162.159.44.176 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 618 | 104.17.21.106 | 104.17.21.106 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare |
| 745 | 2a06:98c1:51:6e:e874:db4f:a1d5:2163 | 2a06:98c1:51:6e:e874:db4f:a1d5:2163 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare |
| 40 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 84 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 175 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 628 | 162.159.39.99 | 162.159.39.99 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 739 | 162.159.21.222 | 162.159.21.222 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 791 | 172.64.32.77 | 172.64.32.77 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare |
| 214 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 262 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 538 | 162.159.39.62 | 162.159.39.62 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 609 | 2a06:98c1:3105:0:2359:4222:d558:10fb | 2a06:98c1:3105:0:2359:4222:d558:10fb | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 746 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare |
| 350 | ashton.ns.cloudflare.com | 162.159.44.173 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 405 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 527 | 104.18.39.15 | 104.18.39.15 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 605 | 104.16.153.12 | 104.16.153.12 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 630 | 162.159.44.128 | 162.159.44.128 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 760 | 162.159.38.45 | 162.159.38.45 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 126 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 437 | lewis.ns.cloudflare.com | 2803:f800:50::6ca2:c39f | IPv6 | h2 | ✅ 成功 | 70 | cloudflare |
| 531 | 162.159.38.192 | 162.159.38.192 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 532 | 172.64.53.0 | 172.64.53.0 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 534 | 108.162.194.125 | 108.162.194.125 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 657 | 162.159.39.177 | 162.159.39.177 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 764 | 108.162.198.198 | 108.162.198.198 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 497 | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6 | h2 | ✅ 成功 | 71 | cloudflare |
| 135 | craig.ns.cloudflare.com | 108.162.195.192 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 137 | craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 363 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | h2 | ✅ 成功 | 72 | cloudflare |
| 530 | 108.162.198.48 | 108.162.198.48 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 675 | 172.64.34.153 | 172.64.34.153 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 737 | 172.64.52.67 | 172.64.52.67 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 763 | 162.159.45.67 | 162.159.45.67 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |
| 184 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 451 | 162.159.24.131 | 162.159.24.131 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 537 | 172.64.52.110 | 172.64.52.110 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 596 | 162.159.1.145 | 162.159.1.145 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare |
| 272 | braden.ns.cloudflare.com | 162.159.44.169 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 279 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 535 | 162.159.0.115 | 162.159.0.115 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 831 | 172.64.52.181 | 172.64.52.181 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 26 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 536 | 162.159.45.93 | 162.159.45.93 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 574 | 162.159.36.26 | 162.159.36.26 | IPv4 | h2 | ✅ 成功 | 75 | cloudflare |
| 191 | dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 441 | 172.64.52.127 | 172.64.52.127 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 595 | 172.64.40.196 | 172.64.40.196 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 650 | 162.159.36.205 | 162.159.36.205 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 750 | 162.159.45.165 | 162.159.45.165 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 759 | 172.64.53.41 | 172.64.53.41 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 767 | 162.159.0.41 | 162.159.0.41 | IPv4 | h2 | ✅ 成功 | 76 | cloudflare |
| 384 | 172.64.35.24 | 172.64.35.24 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 775 | 162.159.6.44 | 162.159.6.44 | IPv4 | h2 | ✅ 成功 | 77 | cloudflare |
| 513 | 172.67.64.214 | 172.67.64.214 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 710 | 162.159.1.39 | 162.159.1.39 | IPv4 | h2 | ✅ 成功 | 78 | cloudflare |
| 354 | julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 507 | 173.245.58.237 | 173.245.58.237 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 533 | 172.64.50.51 | 172.64.50.51 | IPv4 | h2 | ✅ 成功 | 79 | cloudflare |
| 255 | bowen.ns.cloudflare.com | 162.159.44.83 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 505 | 162.159.46.238 | 162.159.46.238 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 705 | 162.159.12.120 | 162.159.12.120 | IPv4 | h2 | ✅ 成功 | 80 | cloudflare |
| 404 | otto.ns.cloudflare.com | 162.159.44.135 | IPv4 | h2 | ✅ 成功 | 81 | cloudflare |
| 277 | 104.16.61.163 | 104.16.61.163 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 450 | 172.64.48.226 | 172.64.48.226 | IPv4 | h2 | ✅ 成功 | 82 | cloudflare |
| 412 | damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h2 | ✅ 成功 | 83 | cloudflare |
| 629 | 162.159.7.12 | 162.159.7.12 | IPv4 | h2 | ✅ 成功 | 84 | cloudflare |
| 281 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 348 | ashton.ns.cloudflare.com | 108.162.195.173 | IPv4 | h2 | ✅ 成功 | 85 | cloudflare |
| 463 | abdullah.ns.cloudflare.com | 172.64.35.203 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 499 | 162.159.17.243 | 162.159.17.243 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 704 | 162.159.38.134 | 162.159.38.134 | IPv4 | h2 | ✅ 成功 | 86 | cloudflare |
| 298 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 584 | 172.64.49.146 | 172.64.49.146 | IPv4 | h2 | ✅ 成功 | 87 | cloudflare |
| 774 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4 | IPv6 | h2 | ✅ 成功 | 87 | cloudflare |
| 186 | kyree.ns.cloudflare.com | 2803:f800:50::6ca2:c3cf | IPv6 | h2 | ✅ 成功 | 88 | cloudflare |
| 702 | 162.159.11.128 | 162.159.11.128 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 830 | 162.159.45.150 | 162.159.45.150 | IPv4 | h2 | ✅ 成功 | 88 | cloudflare |
| 176 | decker.ns.cloudflare.com | 2a06:98c1:50::ac40:239b | IPv6 | h2 | ✅ 成功 | 89 | cloudflare |
| 503 | 162.159.13.51 | 162.159.13.51 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 577 | 162.159.19.37 | 162.159.19.37 | IPv4 | h2 | ✅ 成功 | 89 | cloudflare |
| 28 | wilson.ns.cloudflare.com | 2a06:98c1:50::ac40:236e | IPv6 | h2 | ✅ 成功 | 90 | cloudflare |
| 140 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | h2 | ✅ 成功 | 90 | cloudflare |
| 151 | pranab.ns.cloudflare.com | 2803:f800:50::6ca2:c3c7 | IPv6 | h2 | ✅ 成功 | 90 | cloudflare |
| 258 | bowen.ns.cloudflare.com | 2a06:98c1:50::ac40:2353 | IPv6 | h2 | ✅ 成功 | 90 | cloudflare |
| 259 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | h2 | ✅ 成功 | 90 | cloudflare |
| 274 | braden.ns.cloudflare.com | 2606:4700:58::a29f:2ca9 | IPv6 | h2 | ✅ 成功 | 90 | cloudflare |
| 524 | 104.17.16.248 | 104.17.16.248 | IPv4 | h2 | ✅ 成功 | 90 | cloudflare |
| 641 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4 | IPv6 | h2 | ✅ 成功 | 90 | cloudflare |
| 31 | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 156 | cris.ns.cloudflare.com | 2803:f800:50::6ca2:c3ca | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 264 | rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 284 | moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 366 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 415 | damien.ns.cloudflare.com | 2a06:98c1:50::ac40:23a8 | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 436 | lewis.ns.cloudflare.com | 2606:4700:58::a29f:2c9f | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 747 | 2a06:98c1:51::ee:b8fb:877f | 2a06:98c1:51::ee:b8fb:877f | IPv6 | h2 | ✅ 成功 | 91 | cloudflare |
| 765 | 162.159.44.202 | 162.159.44.202 | IPv4 | h2 | ✅ 成功 | 91 | cloudflare |
| 35 | trevor.ns.cloudflare.com | 2606:4700:58::a29f:2c9a | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 283 | moura.ns.cloudflare.com | 2606:4700:58::a29f:2cd9 | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 465 | abdullah.ns.cloudflare.com | 2803:f800:50::6ca2:c3cb | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 722 | 2803:f800:51:0:fc87:e2d6:88c3:378b | 2803:f800:51:0:fc87:e2d6:88c3:378b | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 819 | 2803:f800:50:9516:e4a1:4ba9:1c5e:7533 | 2803:f800:50:9516:e4a1:4ba9:1c5e:7533 | IPv6 | h2 | ✅ 成功 | 92 | cloudflare |
| 87 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 193 | dylan.ns.cloudflare.com | 2803:f800:50::6ca2:c3bb | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 263 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 275 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 433 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h2 | ✅ 成功 | 93 | cloudflare |
| 611 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59 | IPv6 | h2 | ✅ 成功 | 93 | cloudflare |
| 29 | wilson.ns.cloudflare.com | 2803:f800:50::6ca2:c36e | IPv6 | h2 | ✅ 成功 | 94 | cloudflare |
| 359 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | h2 | ✅ 成功 | 94 | cloudflare |
| 754 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6 | h2 | ✅ 成功 | 94 | cloudflare |
| 756 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | 2a06:98c1:50:8be4:5078:7eea:e43d:164 | IPv6 | h2 | ✅ 成功 | 94 | cloudflare |
| 138 | craig.ns.cloudflare.com | 2606:4700:58::a29f:2cc0 | IPv6 | h2 | ✅ 成功 | 95 | cloudflare |
| 139 | craig.ns.cloudflare.com | 2803:f800:50::6ca2:c3c0 | IPv6 | h2 | ✅ 成功 | 95 | cloudflare |
| 154 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 276 | braden.ns.cloudflare.com | 2a06:98c1:50::ac40:23a9 | IPv6 | h2 | ✅ 成功 | 95 | cloudflare |
| 299 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6 | h2 | ✅ 成功 | 95 | cloudflare |
| 466 | abdullah.ns.cloudflare.com | 2a06:98c1:50::ac40:23cb | IPv6 | h2 | ✅ 成功 | 95 | cloudflare |
| 621 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | 2a06:98c1:50:f771:e9b:84bd:5505:3935 | IPv6 | h2 | ✅ 成功 | 95 | cloudflare |
| 647 | 162.159.39.146 | 162.159.39.146 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 792 | 162.159.16.136 | 162.159.16.136 | IPv4 | h2 | ✅ 成功 | 95 | cloudflare |
| 34 | trevor.ns.cloudflare.com | 2803:f800:50::6ca2:c39a | IPv6 | h2 | ✅ 成功 | 96 | cloudflare |
| 129 | sullivan.ns.cloudflare.com | 2a06:98c1:50::ac40:23a1 | IPv6 | h2 | ✅ 成功 | 96 | cloudflare |
| 189 | dylan.ns.cloudflare.com | 162.159.44.187 | IPv4 | h2 | ✅ 成功 | 96 | cloudflare |
| 257 | bowen.ns.cloudflare.com | 2803:f800:50::6ca2:c353 | IPv6 | h2 | ✅ 成功 | 96 | cloudflare |
| 352 | ashton.ns.cloudflare.com | 2a06:98c1:50::ac40:23ad | IPv6 | h2 | ✅ 成功 | 96 | cloudflare |
| 27 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e | IPv6 | h2 | ✅ 成功 | 97 | cloudflare |
| 89 | huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | h2 | ✅ 成功 | 97 | cloudflare |
| 416 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 | h2 | ✅ 成功 | 97 | cloudflare |
| 435 | lewis.ns.cloudflare.com | 162.159.44.159 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 634 | 162.159.45.65 | 162.159.45.65 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 744 | 108.162.198.148 | 108.162.198.148 | IPv4 | h2 | ✅ 成功 | 97 | cloudflare |
| 351 | ashton.ns.cloudflare.com | 2606:4700:58::a29f:2cad | IPv6 | h2 | ✅ 成功 | 98 | cloudflare |
| 356 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h2 | ✅ 成功 | 98 | cloudflare |
| 407 | otto.ns.cloudflare.com | 2606:4700:58::a29f:2c87 | IPv6 | h2 | ✅ 成功 | 98 | cloudflare |
| 369 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 | IPv6 | h2 | ✅ 成功 | 99 | cloudflare |
| 627 | 108.162.198.85 | 108.162.198.85 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 659 | 172.64.52.90 | 172.64.52.90 | IPv4 | h2 | ✅ 成功 | 99 | cloudflare |
| 155 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 691 | 162.159.22.29 | 162.159.22.29 | IPv4 | h2 | ✅ 成功 | 100 | cloudflare |
| 296 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h2 | ✅ 成功 | 101 | cloudflare |
| 365 | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4 | h2 | ✅ 成功 | 101 | cloudflare |
| 467 | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb | IPv6 | h2 | ✅ 成功 | 102 | cloudflare |
| 632 | 172.64.53.144 | 172.64.53.144 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 658 | 162.159.45.145 | 162.159.45.145 | IPv4 | h2 | ✅ 成功 | 102 | cloudflare |
| 368 | uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | h2 | ✅ 成功 | 103 | cloudflare |
| 646 | 162.159.44.101 | 162.159.44.101 | IPv4 | h2 | ✅ 成功 | 103 | cloudflare |
| 136 | craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h2 | ✅ 成功 | 104 | cloudflare |
| 367 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 | h2 | ✅ 成功 | 104 | cloudflare |
| 500 | 162.159.21.116 | 162.159.21.116 | IPv4 | h2 | ✅ 成功 | 104 | cloudflare |
| 712 | 108.162.198.70 | 108.162.198.70 | IPv4 | h2 | ✅ 成功 | 105 | cloudflare |
| 174 | decker.ns.cloudflare.com | 162.159.44.155 | IPv4 | h2 | ✅ 成功 | 106 | cloudflare |
| 256 | bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h2 | ✅ 成功 | 106 | cloudflare |
| 182 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 434 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 678 | 108.162.198.170 | 108.162.198.170 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 682 | 162.159.3.89 | 162.159.3.89 | IPv4 | h2 | ✅ 成功 | 107 | cloudflare |
| 278 | 162.159.36.104 | 162.159.36.104 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 355 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h2 | ✅ 成功 | 108 | cloudflare |
| 149 | pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 797 | 162.159.43.50 | 162.159.43.50 | IPv4 | h2 | ✅ 成功 | 109 | cloudflare |
| 521 | 172.64.229.191 | 172.64.229.191 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 818 | 162.159.33.191 | 162.159.33.191 | IPv4 | h2 | ✅ 成功 | 110 | cloudflare |
| 148 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4 | h2 | ✅ 成功 | 112 | cloudflare |
| 599 | 162.159.36.52 | 162.159.36.52 | IPv4 | h2 | ✅ 成功 | 113 | cloudflare |
| 280 | moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h2 | ✅ 成功 | 114 | cloudflare |
| 743 | 162.159.38.67 | 162.159.38.67 | IPv4 | h2 | ✅ 成功 | 114 | cloudflare |
| 769 | 162.159.20.46 | 162.159.20.46 | IPv4 | h2 | ✅ 成功 | 114 | cloudflare |
| 827 | 162.159.39.74 | 162.159.39.74 | IPv4 | h2 | ✅ 成功 | 114 | cloudflare |
| 25 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h2 | ✅ 成功 | 115 | cloudflare |
| 419 | 108.162.198.54 | 108.162.198.54 | IPv4 | h2 | ✅ 成功 | 115 | cloudflare |
| 626 | 162.159.19.219 | 162.159.19.219 | IPv4 | h2 | ✅ 成功 | 115 | cloudflare |
| 502 | 108.162.192.66 | 108.162.192.66 | IPv4 | h2 | ✅ 成功 | 116 | cloudflare |
| 762 | 172.64.52.224 | 172.64.52.224 | IPv4 | h2 | ✅ 成功 | 116 | cloudflare |
| 53 | steamdb.info | 104.20.34.212 | IPv4 | h2 | ✅ 成功 | 117 | cloudflare |
| 501 | 162.159.6.115 | 162.159.6.115 | IPv4 | h2 | ✅ 成功 | 117 | cloudflare |
| 706 | 162.159.58.17 | 162.159.58.17 | IPv4 | h2 | ✅ 成功 | 117 | cloudflare |
| 523 | 104.16.255.1 | 104.16.255.1 | IPv4 | h2 | ✅ 成功 | 119 | cloudflare |
| 624 | 108.162.198.69 | 108.162.198.69 | IPv4 | h2 | ✅ 成功 | 119 | cloudflare |
| 86 | huxley.ns.cloudflare.com | 108.162.195.188 | IPv4 | h2 | ✅ 成功 | 120 | cloudflare |
| 656 | 162.159.38.171 | 162.159.38.171 | IPv4 | h2 | ✅ 成功 | 120 | cloudflare |
| 455 | 162.159.58.65 | 162.159.58.65 | IPv4 | h2 | ✅ 成功 | 121 | cloudflare |
| 708 | 172.64.53.202 | 172.64.53.202 | IPv4 | h2 | ✅ 成功 | 123 | cloudflare |
| 297 | benedict.ns.cloudflare.com | 162.159.44.205 | IPv4 | h2 | ✅ 成功 | 124 | cloudflare |
| 703 | 162.159.3.128 | 162.159.3.128 | IPv4 | h2 | ✅ 成功 | 124 | cloudflare |
| 32 | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4 | h2 | ✅ 成功 | 125 | cloudflare |
| 623 | 162.159.45.176 | 162.159.45.176 | IPv4 | h2 | ✅ 成功 | 126 | cloudflare |
| 631 | 162.159.38.35 | 162.159.38.35 | IPv4 | h2 | ✅ 成功 | 127 | cloudflare |
| 779 | 162.159.36.223 | 162.159.36.223 | IPv4 | h2 | ✅ 成功 | 127 | cloudflare |
| 707 | 162.159.39.180 | 162.159.39.180 | IPv4 | h2 | ✅ 成功 | 129 | cloudflare |
| 254 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h2 | ✅ 成功 | 146 | cloudflare |
| 349 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h2 | ✅ 成功 | 146 | cloudflare |
| 127 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h2 | ✅ 成功 | 148 | cloudflare |
| 85 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h2 | ✅ 成功 | 149 | cloudflare |
| 525 | 104.17.168.159 | 104.17.168.159 | IPv4 | h2 | ✅ 成功 | 149 | cloudflare |
| 273 | braden.ns.cloudflare.com | 172.64.35.169 | IPv4 | h2 | ✅ 成功 | 155 | cloudflare |
| 526 | 104.17.170.110 | 104.17.170.110 | IPv4 | h2 | ✅ 成功 | 155 | cloudflare |
| 147 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h2 | ✅ 成功 | 159 | cloudflare |
| 464 | abdullah.ns.cloudflare.com | 162.159.44.203 | IPv4 | h2 | ✅ 成功 | 161 | cloudflare |
| 462 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h2 | ✅ 成功 | 168 | cloudflare |
| 220 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h2 | ✅ 成功 | 225 | cloudflare |
| 249 | 141.147.185.63 | 141.147.185.63 | IPv4 | h2 | ✅ 成功 | 306 | cloudflare |
| 36 | 34.143.159.175 | 34.143.159.175 | IPv4 | h2 | ✅ 成功 | 424 | cloudflare |
| 64 | 168.138.165.174 | 168.138.165.174 | IPv4 | h2 | ✅ 成功 | 453 | cloudflare |
| 401 | 3.0.50.69 | 3.0.50.69 | IPv4 | h2 | ✅ 成功 | 673 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 165 条记录
- **快 (50-100ms)**: 599 条记录
- **正常 (100-200ms)**: 62 条记录
- **慢 (200-500ms)**: 4 条记录
- **很慢 (>500ms)**: 1 条记录


---

## 详细分析

### 按IP版本统计
- **IPv4 失败**: 4 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 4 次失败


---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

*此报告由 HTTP/3 连接测试报告生成器自动生成*
