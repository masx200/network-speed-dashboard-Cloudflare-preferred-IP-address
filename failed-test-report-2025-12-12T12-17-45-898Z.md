# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 12:17:45
- **数据来源**: connectivity_results-20251212-121745.json
- **总测试数**: 444
- **失败测试数**: 2
- **成功测试数**: 442
- **失败率**: 0.45%
- **平均延迟**: 62.76ms
- **最小延迟**: 38ms
- **最大延迟**: 988ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 12:17:45
- **IP地址**: 2a09:bac5:9f22:1678::23d:af
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

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 37   | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 187  | ct.877774.xyz    | ct.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout  |

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
| 374  | uriah.ns.cloudflare.com               | 2606:4700:58::a29f:2cc2                 | IPv6   | h2   | ✅ 成功 | 38       | cloudflare |
| 342  | japan.com                             | 104.26.5.60                             | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 119  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::ac43:9bac               | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 219  | cmcc.877774.xyz                       | 104.16.148.10                           | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 361  | stock.hostmonit.com                   | 104.21.7.193                            | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 362  | stock.hostmonit.com                   | 172.67.187.251                          | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 307  | 456.cloudflare.182682.xyz             | 2606:4700:20::681a:9a0                  | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 380  | cf.877774.xyz                         | 104.18.41.190                           | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 8    | www.pcmag.com                         | 2606:4700::6810:1476                    | IPv6   | h2   | ✅ 成功 | 42       | cloudflare |
| 60   | steamdb.info                          | 2606:4700:10::ac42:affa                 | IPv6   | h2   | ✅ 成功 | 42       | cloudflare |
| 196  | www.whatismyip.com                    | 104.26.13.23                            | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 199  | www.whatismyip.com                    | 2606:4700:20::681a:d17                  | IPv6   | h2   | ✅ 成功 | 42       | cloudflare |
| 65   | ipv4.ip.sb                            | 104.26.12.31                            | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 67   | 172.67.75.172                         | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 77   | www.gov.ua                            | 2606:4700:3031::6815:1748               | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 102  | www.okcupid.com                       | 104.16.223.254                          | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 112  | www.hugedomains.com                   | 2606:4700:20::681a:625                  | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 378  | gamer.com.tw                          | 104.18.2.197                            | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 381  | cf.877774.xyz                         | 172.64.146.66                           | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 388  | tasteatlas.com                        | 2606:4700::6811:2569                    | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 390  | www.udemy.com                         | 104.16.143.237                          | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 442  | cf.zhetengsha.eu.org                  | 104.18.43.174                           | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 13   | comicabc.com                          | 104.21.64.10                            | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 40   | cf.0sm.com                            | 104.21.7.133                            | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 41   | cf.0sm.com                            | 172.67.187.145                          | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 47   | ct.877774.xyz                         | 172.64.229.161                          | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 53   | ct.877774.xyz                         | 172.64.229.236                          | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 55   | ipinfo.in                             | 104.21.21.129                           | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 59   | steamdb.info                          | 172.66.175.250                          | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 86   | icook.hk                              | 2606:4700:3037::ac43:a168               | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 113  | www.hugedomains.com                   | 2606:4700:20::681a:725                  | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 156  | pranab.ns.cloudflare.com              | 2a06:98c1:50::ac40:23c7                 | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 236  | icook.tw                              | 2606:4700:10::6814:1c4a                 | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 272  | 104.26.13.31                          | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 298  | ip.gs                                 | 2606:4700:3035::ac43:a01c               | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 355  | www.wto.org                           | 104.18.41.190                           | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 367  | www.csgo.com                          | 195.85.59.95                            | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 403  | 104.16.223.179                        | 104.16.223.179                          | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 410  | cf.090227.xyz                         | 2a06:98c1:3101::ac40:919e               | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 20   | time.is                               | 2606:4700:20::ac43:449d                 | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 42   | cf.0sm.com                            | 2606:4700:3032::6815:785                | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 61   | steamdb.info                          | 2606:4700:10::6814:22d4                 | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 87   | iplocation.io                         | 104.26.10.222                           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 90   | iplocation.io                         | 2606:4700:20::ac43:4664                 | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 111  | www.hugedomains.com                   | 104.26.6.37                             | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 150  | freeyx.cloudflare88.eu.org            | 141.101.120.219                         | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 166  | toy-people.com                        | 172.67.72.18                            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 193  | dylan.ns.cloudflare.com               | 2a06:98c1:50::ac40:23bb                 | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 211  | cmcc.877774.xyz                       | 104.16.148.2                            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 216  | cmcc.877774.xyz                       | 104.16.148.7                            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 249  | www.digitalocean.com                  | 2606:4700::6813:ad44                    | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 251  | 198.62.62.4                           | 198.62.62.4                             | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 278  | singapore.com                         | 2606:4700:20::681a:c8c                  | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 308  | 104.19.223.58                         | 104.19.223.58                           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 310  | whatismyipaddress.com                 | 104.19.223.79                           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 331  | ifconfig.co                           | 2606:4700:3037::6815:365b               | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 339  | www.visa.com.hk                       | 104.18.21.69                            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 341  | japan.com                             | 104.26.4.60                             | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 343  | japan.com                             | 2606:4700:20::681a:53c                  | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 356  | www.wto.org                           | 2a06:98c1:3102::6812:29be               | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 365  | 172.67.106.26                         | 172.67.106.26                           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 368  | www.csgo.com                          | 195.85.59.161                           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 382  | cf.877774.xyz                         | 2606:4700:4406::ac40:9242               | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 400  | saas.sin.fan                          | 162.159.36.20                           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 407  | fbi.gov                               | 2606:4700::6810:95f4                    | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 415  | trevor.ns.cloudflare.com              | 2a06:98c1:50::ac40:239a                 | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 436  | bestcf.030101.xyz                     | 2606:4700::fffd:819d:acda               | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 4    | www.visa.cn                           | 162.159.153.2                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 14   | comicabc.com                          | 172.67.174.21                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 22   | time.is                               | 2606:4700:20::681a:d36                  | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 45   | 104.18.39.196                         | 104.18.39.196                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 70   | 172.67.110.232                        | 172.67.110.232                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 75   | www.gov.ua                            | 104.21.23.72                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 76   | www.gov.ua                            | 172.67.209.127                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 80   | www.4chan.org                         | 104.16.229.229                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 84   | icook.hk                              | 172.67.161.104                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 89   | iplocation.io                         | 172.67.70.100                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 101  | www.okcupid.com                       | 104.16.144.63                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 107  | cf.877771.xyz                         | 2606:4700:3033::6815:50b4               | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 110  | www.hugedomains.com                   | 104.26.7.37                             | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 115  | www.visa.com.sg                       | 104.18.13.229                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 116  | www.visa.com.sg                       | 104.18.12.229                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 117  | cloudflare-ip.mofashi.ltd             | 172.67.155.172                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 127  | 172.67.243.218                        | 172.67.243.218                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 128  | yx-auto.pages.dev                     | 172.66.44.144                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 133  | cu.877774.xyz                         | 104.26.4.114                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 134  | cu.877774.xyz                         | 104.26.4.113                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 139  | cu.877774.xyz                         | 104.26.4.112                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 152  | freeyx.cloudflare88.eu.org            | 2606:4700:3009:aa59:4b79:c9c1:cab1:57d6 | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 167  | toy-people.com                        | 104.26.3.36                             | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 194  | www.whatismyip.com                    | 104.26.12.23                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 198  | www.whatismyip.com                    | 2606:4700:20::681a:c17                  | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 203  | www.ipchicken.com                     | 104.26.7.112                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 218  | cmcc.877774.xyz                       | 104.16.148.9                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 224  | cmcc.877774.xyz                       | 104.16.149.2                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 234  | icook.tw                              | 172.66.158.115                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 247  | www.digitalocean.com                  | 104.19.173.68                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 288  | eur.877774.xyz                        | 104.21.47.209                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 314  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6               | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 344  | japan.com                             | 2606:4700:20::681a:43c                  | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |

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
