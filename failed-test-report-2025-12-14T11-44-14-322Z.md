# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/14 11:44:14
- **数据来源**: connectivity_results-20251214-114413.json
- **总测试数**: 452
- **失败测试数**: 2
- **成功测试数**: 450
- **失败率**: 0.44%
- **平均延迟**: 85.56ms
- **最小延迟**: 61ms
- **最大延迟**: 982ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/14 11:44:14
- **IP地址**: 2a09:bac1:7680:8628::3cd:20
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 41.1446, -104.8116
- **时区**: America/Denver
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 2 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (2 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 120  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 341  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 2 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 2 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP                                  | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 230  | singapore.com                         | 2606:4700:20::681a:c8c                  | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 242  | 456.cloudflare.182682.xyz             | 2606:4700:20::681a:8a0                  | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 119  | fbi.gov                               | 2606:4700::6810:95f4                    | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 135  | cf.090227.xyz                         | 172.64.144.82                           | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 187  | ipv4.ip.sb                            | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 188  | ipv4.ip.sb                            | 104.26.12.31                            | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 200  | ip.sb                                 | 2606:4700:20::681a:c1f                  | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 383  | eur.877774.xyz                        | 104.21.26.150                           | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 97   | www.whatismyip.com                    | 2606:4700:20::ac43:4581                 | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 131  | cfip.xxxxxxxx.tk                      | 104.25.105.1                            | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 169  | www.glassdoor.com                     | 104.17.64.70                            | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 199  | ip.sb                                 | 2606:4700:20::ac43:4bac                 | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 228  | singapore.com                         | 2606:4700:20::681a:d8c                  | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 231  | 172.67.75.172                         | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 271  | whatismyipaddress.com                 | 2606:4700::6813:df4f                    | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 278  | www.hugedomains.com                   | 2606:4700:20::681a:725                  | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 291  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182                          | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 346  | www.udemy.com                         | 104.16.143.237                          | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 349  | www.udemy.com                         | 2606:4700::6810:8fed                    | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 355  | icook.tw                              | 2606:4700:10::ac42:9e73                 | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 361  | www.digitalocean.com                  | 104.19.173.68                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 381  | eur.877774.xyz                        | 104.21.29.164                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 382  | eur.877774.xyz                        | 104.21.47.209                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 406  | ifconfig.co                           | 104.21.54.91                            | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 408  | ifconfig.co                           | 2606:4700:3037::6815:365b               | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 418  | www.wto.org                           | 172.64.146.66                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 25   | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2c90               | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 58   | toy-people.com                        | 104.26.3.36                             | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 69   | asia.877774.xyz                       | 104.16.211.153                          | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 77   | 104.16.223.179                        | 104.16.223.179                          | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 82   | zread.ai                              | 2606:4700:3032::ac43:ca4e               | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 83   | cloudflare-ip.mofashi.ltd             | 104.21.72.233                           | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 84   | cloudflare-ip.mofashi.ltd             | 172.67.155.172                          | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 86   | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::ac43:9bac               | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 94   | www.whatismyip.com                    | 104.26.13.23                            | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 98   | www.whatismyip.com                    | 2606:4700:20::681a:d17                  | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 106  | saas.sin.fan                          | 162.159.36.5                            | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 114  | cf.zhetengsha.eu.org                  | 2606:4700:4407::ac40:9052               | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 151  | www.ipchicken.com                     | 104.26.6.112                            | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 153  | www.ipchicken.com                     | 104.26.7.112                            | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 173  | time.is                               | 2606:4700:20::681a:d36                  | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 182  | palera.in                             | 104.21.58.72                            | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 186  | ipv4.ip.sb                            | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 195  | ip.sb                                 | 104.26.12.31                            | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 211  | ipinfo.in                             | 2606:4700:3037::ac43:c6cb               | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 219  | www.gov.ua                            | 172.67.209.127                          | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 224  | 104.16.45.84                          | 104.16.45.84                            | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 225  | singapore.com                         | 104.26.12.140                           | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 238  | 456.cloudflare.182682.xyz             | 104.26.9.160                            | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 248  | 104.18.254.88                         | 104.18.254.88                           | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 251  | icook.hk                              | 104.21.90.210                           | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 253  | icook.hk                              | 2606:4700:3037::ac43:a168               | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 259  | iplocation.io                         | 104.26.10.222                           | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 263  | iplocation.io                         | 2606:4700:20::681a:bde                  | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 276  | www.hugedomains.com                   | 2606:4700:20::681a:625                  | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 277  | www.hugedomains.com                   | 2606:4700:20::ac43:46bf                 | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 282  | trevor.ns.cloudflare.com              | 2803:f800:50::6ca2:c39a                 | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 298  | ashton.ns.cloudflare.com              | 2803:f800:50::6ca2:c3ad                 | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 308  | www.visa.com.hk                       | 104.18.21.69                            | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 310  | cu.877774.xyz                         | 104.26.4.111                            | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 315  | cu.877774.xyz                         | 104.26.4.116                            | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 338  | uriah.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c2                 | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 345  | tasteatlas.com                        | 2606:4700::6811:2569                    | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 357  | 198.62.62.4                           | 198.62.62.4                             | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 379  | japan.com                             | 2606:4700:20::681a:43c                  | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 401  | stock.hostmonit.com                   | 104.21.7.193                            | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 417  | www.wto.org                           | 104.18.41.190                           | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 434  | cmcc.877774.xyz                       | 104.16.148.2                            | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 436  | cmcc.877774.xyz                       | 104.16.149.3                            | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 438  | cmcc.877774.xyz                       | 104.16.149.11                           | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 439  | cmcc.877774.xyz                       | 104.16.149.244                          | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 442  | cmcc.877774.xyz                       | 104.16.148.6                            | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 17   | comicabc.com                          | 2606:4700:3030::ac43:ae15               | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 18   | cf.877771.xyz                         | 172.67.152.183                          | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 26   | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2f70               | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 31   | huxley.ns.cloudflare.com              | 2803:f800:50::6ca2:c3bc                 | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 43   | freeyx.cloudflare88.eu.org            | 2606:4700:3009:aa73:82e3:b098:2d2c:5ebd | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 53   | cris.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ca                 | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 60   | toy-people.com                        | 2606:4700:20::681a:224                  | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 78   | 104.17.79.11                          | 104.17.79.11                            | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 85   | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::6815:48e9               | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 102  | bestcf.030101.xyz                     | 104.19.153.222                          | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 103  | bestcf.030101.xyz                     | 2606:4700::8d:f082:8938:66d8            | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 115  | cf.zhetengsha.eu.org                  | 2a06:98c1:310d::6812:2bae               | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 129  | cfip.xxxxxxxx.tk                      | 190.93.244.201                          | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 136  | cf.090227.xyz                         | 2a06:98c1:310d::6812:2bae               | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 161  | cf.0sm.com                            | 2606:4700:3037::ac43:bb91               | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 181  | rustam.ns.cloudflare.com              | 2606:4700:58::a29f:2c94                 | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 198  | ip.sb                                 | 2606:4700:20::681a:d1f                  | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 205  | ct.877774.xyz                         | 172.64.229.161                          | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 208  | ct.877774.xyz                         | 172.64.229.185                          | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 209  | ipinfo.in                             | 104.21.21.129                           | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 218  | www.gov.ua                            | 104.21.23.72                            | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 227  | singapore.com                         | 172.67.75.194                           | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 237  | 456.cloudflare.182682.xyz             | 104.26.8.160                            | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 245  | ip.gs                                 | 104.21.14.176                           | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 247  | ip.gs                                 | 2606:4700:3036::6815:eb0                | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 249  | www.4chan.org                         | 104.16.228.229                          | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 250  | www.4chan.org                         | 104.16.229.229                          | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 256  | [2606:4700:4408::18c5:3304]           | 2606:4700:4408::18c5:3304               | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 100 条记录
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

_此报告由 HTTP/3 连接测试报告生成器自动生成_
