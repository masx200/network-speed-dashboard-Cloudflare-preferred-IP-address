# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 07:07:31
- **数据来源**: connectivity_results-20251212-070730.json
- **总测试数**: 457
- **失败测试数**: 2
- **成功测试数**: 455
- **失败率**: 0.44%
- **平均延迟**: 55.10ms
- **最小延迟**: 35ms
- **最大延迟**: 833ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 07:07:31
- **IP地址**: 2a09:bac1:7680:c8::3c0:3a
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

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 205  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 275  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 2 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 172.64（1
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
| 127  | singapore.com                         | 172.67.75.194                           | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 300  | ipv4.ip.sb                            | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 327  | cu.877774.xyz                         | 104.26.4.112                            | IPv4   | h2   | ✅ 成功 | 35       | cloudflare |
| 37   | www.whatismyip.com                    | 2606:4700:20::681a:c17                  | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 136  | 456.cloudflare.182682.xyz                          | 2606:4700:20::ac43:4bd0                 | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 140  | www.visa.com.hk                       | 104.18.20.69                            | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 149  | local-aria2-webui.masx200.ddns-ip.net | 104.21.14.41                            | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 150  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6               | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 188  | yx-auto.pages.dev                     | 172.67.161.98                           | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 190  | yx-auto.pages.dev                     | 2606:4700:3034::6815:9e6                | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 249  | www.wto.org                           | 104.18.41.190                           | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 261  | stock.hostmonit.com                   | 2606:4700:3037::6815:7c1                | IPv6   | h2   | ✅ 成功 | 36       | cloudflare |
| 311  | 172.67.75.172                         | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 325  | cu.877774.xyz                         | 104.26.4.119                            | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 435  | cmcc.877774.xyz                       | 104.16.149.5                            | IPv4   | h2   | ✅ 成功 | 36       | cloudflare |
| 48   | xn--b6gac.eu.org                      | 104.21.90.78                            | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 85   | time.is                               | 2606:4700:20::681a:d36                  | IPv6   | h2   | ✅ 成功 | 37       | cloudflare |
| 108  | palera.in                             | 104.21.58.72                            | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 118  | ae8a9c24-83de.masx200.ddns-ip.net     | 172.67.157.182                          | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 120  | ae8a9c24-83de.masx200.ddns-ip.net     | 2606:4700:3030::6815:e29                | IPv6   | h2   | ✅ 成功 | 37       | cloudflare |
| 137  | 456.cloudflare.182682.xyz                          | 2606:4700:20::681a:8a0                  | IPv6   | h2   | ✅ 成功 | 37       | cloudflare |
| 168  | dnschecker.org                        | 2606:4700:20::681a:659                  | IPv6   | h2   | ✅ 成功 | 37       | cloudflare |
| 180  | cf.877774.xyz                         | 2a06:98c1:3102::6812:29be               | IPv6   | h2   | ✅ 成功 | 37       | cloudflare |
| 184  | uriah.ns.cloudflare.com               | 2606:4700:58::a29f:2cc2                 | IPv6   | h2   | ✅ 成功 | 37       | cloudflare |
| 203  | 104.17.142.12                         | 104.17.142.12                           | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 214  | icook.tw                              | 2606:4700:10::ac42:9e73                 | IPv6   | h2   | ✅ 成功 | 37       | cloudflare |
| 250  | www.wto.org                           | 172.64.146.66                           | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 251  | www.wto.org                           | 2606:4700:4406::ac40:9242               | IPv6   | h2   | ✅ 成功 | 37       | cloudflare |
| 277  | www.pcmag.com                         | 104.16.20.118                           | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 308  | steamdb.info                          | 2606:4700:10::ac42:affa                 | IPv6   | h2   | ✅ 成功 | 37       | cloudflare |
| 310  | shopify.com                           | 23.227.38.33                            | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 316  | ct.877774.xyz                         | 172.64.229.217                          | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 353  | iplocation.io                         | 2606:4700:20::681a:bde                  | IPv6   | h2   | ✅ 成功 | 37       | cloudflare |
| 356  | 172.67.120.0                          | 172.67.120.0                            | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 376  | www.hugedomains.com                   | 2606:4700:20::681a:625                  | IPv6   | h2   | ✅ 成功 | 37       | cloudflare |
| 382  | yx-auto.pages.dev                     | 104.21.6.60                             | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 400  | freeyx.cloudflare88.eu.org            | 141.101.121.18                          | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 401  | freeyx.cloudflare88.eu.org            | 2606:4700:3009:aa59:4b67:cd47:f6a2:e5d6 | IPv6   | h2   | ✅ 成功 | 37       | cloudflare |
| 403  | toy-people.com                        | 172.67.72.18                            | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 405  | toy-people.com                        | 2606:4700:20::681a:224                  | IPv6   | h2   | ✅ 成功 | 37       | cloudflare |
| 456  | cmcc.877774.xyz                       | 104.16.148.244                          | IPv4   | h2   | ✅ 成功 | 37       | cloudflare |
| 14   | zread.ai                              | 2606:4700:3033::6815:4cf0               | IPv6   | h2   | ✅ 成功 | 38       | cloudflare |
| 33   | www.whatismyip.com                    | 104.26.13.23                            | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 59   | fbi.gov                               | 2606:4700::6810:95f4                    | IPv6   | h2   | ✅ 成功 | 38       | cloudflare |
| 84   | time.is                               | 104.26.13.54                            | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 86   | time.is                               | 2606:4700:20::681a:c36                  | IPv6   | h2   | ✅ 成功 | 38       | cloudflare |
| 133  | 456.cloudflare.182682.xyz                          | 172.67.75.208                           | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 138  | 456.cloudflare.182682.xyz                          | 2606:4700:20::681a:9a0                  | IPv6   | h2   | ✅ 成功 | 38       | cloudflare |
| 152  | 104.18.14.76                          | 104.18.14.76                            | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 197  | tasteatlas.com                        | 2606:4700::6811:2469                    | IPv6   | h2   | ✅ 成功 | 38       | cloudflare |
| 210  | [2606:4700:83be::11:74f]              | 2606:4700:83be::11:74f                  | IPv6   | h2   | ✅ 成功 | 38       | cloudflare |
| 233  | 172.67.181.209                        | 172.67.181.209                          | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 242  | ifconfig.co                           | 104.21.54.91                            | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 263  | japan.com                             | 104.26.5.60                             | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 268  | japan.com                             | 2606:4700:20::681a:53c                  | IPv6   | h2   | ✅ 成功 | 38       | cloudflare |
| 279  | www.pcmag.com                         | 2606:4700::6810:1476                    | IPv6   | h2   | ✅ 成功 | 38       | cloudflare |
| 280  | cf.0sm.com                            | 104.21.7.133                            | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 291  | trevor.ns.cloudflare.com              | 2606:4700:58::a29f:2c9a                 | IPv6   | h2   | ✅ 成功 | 38       | cloudflare |
| 313  | 104.26.6.112                          | 104.26.6.112                            | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 324  | cu.877774.xyz                         | 104.26.4.118                            | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 326  | cu.877774.xyz                         | 104.26.4.111                            | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 341  | www.visa.com.sg                       | 104.18.12.229                           | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 345  | icook.hk                              | 104.21.90.210                           | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 346  | icook.hk                              | 172.67.161.104                          | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 354  | iplocation.io                         | 2606:4700:20::681a:ade                  | IPv6   | h2   | ✅ 成功 | 38       | cloudflare |
| 371  | www.hugedomains.com                   | 104.26.6.37                             | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 372  | www.hugedomains.com                   | 104.26.7.37                             | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 439  | cmcc.877774.xyz                       | 104.16.149.9                            | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 454  | cmcc.877774.xyz                       | 104.16.148.11                           | IPv4   | h2   | ✅ 成功 | 38       | cloudflare |
| 10   | comicabc.com                          | 2606:4700:3036::6815:400a               | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 11   | zread.ai                              | 104.21.76.240                           | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 32   | 172.64.151.55                         | 172.64.151.55                           | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 36   | www.whatismyip.com                    | 2606:4700:20::681a:d17                  | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 49   | xn--b6gac.eu.org                      | 172.67.153.253                          | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 57   | fbi.gov                               | 104.16.149.244                          | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 72   | braden.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a9                 | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 87   | time.is                               | 2606:4700:20::ac43:449d                 | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 111  | palera.in                             | 2606:4700:3035::6815:3a48               | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 117  | ip.sb                                 | 2606:4700:20::681a:d1f                  | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 121  | ae8a9c24-83de.masx200.ddns-ip.net     | 2606:4700:3031::ac43:9db6               | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 128  | singapore.com                         | 104.26.13.140                           | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 130  | singapore.com                         | 2606:4700:20::681a:c8c                  | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 144  | whatismyipaddress.com                 | 2606:4700::6813:df4f                    | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 157  | 162.159.133.85                        | 162.159.133.85                          | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 165  | dnschecker.org                        | 104.26.6.89                             | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 167  | dnschecker.org                        | 104.26.7.89                             | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 169  | dnschecker.org                        | 2606:4700:20::681a:759                  | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 177  | cf.877774.xyz                         | 104.18.41.190                           | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 187  | [2606:4700:4403::7357:544f]           | 2606:4700:4403::7357:544f               | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 201  | www.udemy.com                         | 2606:4700::6810:8fed                    | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 206  | cfip.1323123.xyz                      | 104.16.133.220                          | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 213  | icook.tw                              | 2606:4700:10::6814:1c4a                 | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 215  | 104.26.13.31                          | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 237  | damien.ns.cloudflare.com              | 2606:4700:58::a29f:2ca8                 | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 247  | www.csgo.com                          | 195.85.59.161                           | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 276  | www.pcmag.com                         | 104.16.21.118                           | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 303  | 104.18.39.196                         | 104.18.39.196                           | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 312  | 104.18.37.13                          | 104.18.37.13                            | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 318  | ct.877774.xyz                         | 172.64.229.185                          | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 334  | www.gov.ua                            | 2606:4700:3031::6815:1748               | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 100 条记录
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

_此报告由 HTTP/3 连接测试报告生成器自动生成_
