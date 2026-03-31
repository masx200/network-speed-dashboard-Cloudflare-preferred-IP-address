# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/14 13:36:18
- **数据来源**: connectivity_results-20251214-133618.json
- **总测试数**: 447
- **失败测试数**: 3
- **成功测试数**: 444
- **失败率**: 0.67%
- **平均延迟**: 73.30ms
- **最小延迟**: 52ms
- **最大延迟**: 885ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/14 13:36:18
- **IP地址**: 2a09:bac5:6193:78::c:35d
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

- **连接超时: I/O超时**: 2 次 (66.7%)
- **DNS解析错误: 其他DNS错误**: 1 次 (33.3%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (2 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 84   | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 130  | ct.877774.xyz    | ct.877774.xyz  | IPv4   | none | N/A    | 0        | N/A    | dial tcp ct.877774.xyz:443: i/o timeout  |

#### DNS解析错误: 其他DNS错误 (1 次测试)

| 序号 | 主机/域名        | 目标IP  | IP版本  | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息      |
| ---- | ---------------- | ------- | ------- | ---- | ------ | -------- | ------ | ------------- |
| 125  | cfip.1323123.xyz | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析无结果 |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 2 次 (66.7%)
- **DNS解析错误**: 1 次 (33.3%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 3 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP                    | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | ------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 307  | bestcf.030101.xyz                     | 162.159.133.251           | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 403  | cf.090227.xyz                         | 2606:4700:4407::ac40:9052 | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 71   | 456.cloudflare.182682.xyz             | 2606:4700:20::681a:9a0    | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 186  | steamdb.info                          | 2606:4700:10::6814:22d4   | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 220  | ifconfig.co                           | 104.21.54.91              | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 400  | cf.090227.xyz                         | 172.64.144.82             | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 402  | cf.090227.xyz                         | 2a06:98c1:310d::6812:2bae | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 15   | www.ipget.net                         | 2606:4700:3036::6815:fd4  | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 91   | 162.159.133.85                        | 162.159.133.85            | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 121  | uriah.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c2   | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 129  | tasteatlas.com                        | 2606:4700::6811:2569      | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 132  | www.udemy.com                         | 104.16.142.237            | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 175  | ipinfo.in                             | 2606:4700:3037::ac43:c6cb | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 181  | lewis.ns.cloudflare.com               | 2606:4700:58::a29f:2c9f   | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 195  | japan.com                             | 2606:4700:20::ac43:465c   | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 221  | ifconfig.co                           | 2606:4700:3037::6815:365b | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 229  | iplocation.io                         | 2606:4700:20::681a:bde    | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 263  | www.visa.cn                           | 162.159.152.2             | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 268  | abdullah.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cb   | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 394  | www.whatismyip.com                    | 104.26.13.23              | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 428  | cmcc.877774.xyz                       | 104.16.148.12             | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 429  | cmcc.877774.xyz                       | 104.16.148.244            | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 430  | cmcc.877774.xyz                       | 104.16.149.1              | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 108  | dnschecker.org                        | 2606:4700:20::681a:759    | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 128  | tasteatlas.com                        | 2606:4700::6811:2469      | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 134  | www.udemy.com                         | 2606:4700::6810:8eed      | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 143  | icook.tw                              | 2606:4700:10::ac42:9e73   | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 145  | 104.26.13.31                          | 104.26.13.31              | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 176  | ipinfo.in                             | 2606:4700:3031::6815:1581 | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 187  | steamdb.info                          | 2606:4700:10::ac42:affa   | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 191  | ipv4.ip.sb                            | 104.26.13.31              | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 197  | japan.com                             | 2606:4700:20::681a:53c    | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 201  | 104.16.45.84                          | 104.16.45.84              | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 202  | 172.67.75.172                         | 172.67.75.172             | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 205  | www.gov.ua                            | 2606:4700:3033::ac43:d17f | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 212  | ct.877774.xyz                         | 172.64.229.174            | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 214  | ct.877774.xyz                         | 172.64.229.195            | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 222  | ifconfig.co                           | 2606:4700:3030::ac43:a86a | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 253  | huxley.ns.cloudflare.com              | 162.159.44.188            | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 259  | www.csgo.com                          | 195.85.59.161             | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 276  | www.hugedomains.com                   | 104.26.7.37               | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 308  | bestcf.030101.xyz                     | 104.19.53.201             | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 332  | xn--b6gac.eu.org                      | 104.21.90.78              | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 346  | pranab.ns.cloudflare.com              | 2a06:98c1:50::ac40:23c7   | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 369  | zread.ai                              | 2606:4700:3033::6815:4cf0 | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 372  | cloudflare-ip.mofashi.ltd             | 172.67.155.172            | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 26   | rustam.ns.cloudflare.com              | 2a06:98c1:50::ac40:2394   | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 48   | ip.sb                                 | 2606:4700:20::ac43:4bac   | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 53   | 104.17.68.85                          | 104.17.68.85              | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 57   | ip.gs                                 | 2606:4700:3035::ac43:a01c | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 86   | cfip.xxxxxxxx.tk                      | 104.16.232.223            | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 87   | cfip.xxxxxxxx.tk                      | 104.16.241.229            | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 90   | 104.18.14.76                          | 104.18.14.76              | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 93   | julio.ns.cloudflare.com               | 108.162.195.209           | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 122  | uriah.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c2   | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 153  | otto.ns.cloudflare.com                | 162.159.44.135            | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 156  | otto.ns.cloudflare.com                | 2803:f800:50::6ca2:c387   | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 168  | www.pcmag.com                         | 2606:4700::6810:1476      | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 171  | cf.0sm.com                            | 2606:4700:3032::6815:785  | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 182  | lewis.ns.cloudflare.com               | 2803:f800:50::6ca2:c39f   | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 207  | 104.18.37.13                          | 104.18.37.13              | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 219  | ifconfig.co                           | 172.67.168.106            | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 239  | www.wto.org                           | 2a06:98c1:3102::6812:29be | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 245  | stock.hostmonit.com                   | 104.21.7.193              | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 250  | www.4chan.org                         | 104.16.229.229            | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 255  | huxley.ns.cloudflare.com              | 2606:4700:58::a29f:2cbc   | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 272  | www.okcupid.com                       | 104.16.223.254            | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 284  | cf.877771.xyz                         | 172.67.152.183            | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 295  | yx-auto.pages.dev                     | 172.66.47.112             | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 301  | cu.877774.xyz                         | 104.26.4.119              | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 311  | www.visa.com.sg                       | 104.18.12.229             | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 312  | www.visa.com.sg                       | 104.18.13.229             | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 315  | asia.877774.xyz                       | 104.17.139.62             | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 336  | na.877774.xyz                         | 104.18.187.25             | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 344  | pranab.ns.cloudflare.com              | 162.159.44.199            | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 359  | cris.ns.cloudflare.com                | 2606:4700:58::a29f:2cca   | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 361  | toy-people.com                        | 172.67.72.18              | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 363  | toy-people.com                        | 2606:4700:20::ac43:4812   | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 399  | www.whatismyip.com                    | 2606:4700:20::681a:d17    | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 419  | bowen.ns.cloudflare.com               | 2606:4700:58::a29f:2c53   | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 11   | comicabc.com                          | 2606:4700:3030::ac43:ae15 | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 28   | rustam.ns.cloudflare.com              | 2606:4700:58::a29f:2c94   | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 47   | ip.sb                                 | 2606:4700:20::681a:c1f    | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 52   | palera.in                             | 2606:4700:3032::ac43:9d7a | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 58   | singapore.com                         | 104.26.13.140             | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 63   | singapore.com                         | 2606:4700:20::ac43:4bc2   | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 65   | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 72   | whatismyipaddress.com                 | 104.19.222.79             | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 77   | www.visa.com.hk                       | 104.18.20.69              | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 98   | julio.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d1   | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 105  | dnschecker.org                        | 172.67.73.216             | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 109  | dnschecker.org                        | 2606:4700:20::681a:659    | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 114  | cf.877774.xyz                         | 2606:4700:4406::ac40:9242 | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 155  | otto.ns.cloudflare.com                | 108.162.195.135           | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 157  | otto.ns.cloudflare.com                | 2606:4700:58::a29f:2c87   | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 170  | cf.0sm.com                            | 172.67.187.145            | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 185  | steamdb.info                          | 104.20.34.212             | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 193  | japan.com                             | 104.26.5.60               | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 196  | japan.com                             | 2606:4700:20::681a:43c    | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 198  | 104.19.223.58                         | 104.19.223.58             | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |

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

- **none**: 3 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
